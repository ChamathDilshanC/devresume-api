# Deploying devresume-api to Azure (Free Tier)

**Status: deployed and live** at `https://devresume-api.salmondune-b6d2a6eb.centralindia.azurecontainerapps.io/health` (resource group `devresume-rg`, subscription "Azure for Students"). Everything below reflects what actually got provisioned, including two gotchas this specific subscription hit that a fresh subscription might not.

Architecture:

- **Database:** Azure Database for PostgreSQL — Flexible Server, Burstable `Standard_B1ms`, 32GB, region `centralindia` (the SKU covered by Azure's 12-months-free allowance)
- **Backend:** Azure Container Apps (Consumption plan — has a permanent "always free" monthly grant of 180,000 vCPU-seconds / 360,000 GiB-seconds / 2M requests, separate from the 12-month trial), deployed into the pre-existing `nexuscart-env` environment in `NexusCart-RG` (see Section 2 — this subscription caps at **one** Container Apps environment total)
- **Registry:** GitHub Container Registry (ghcr.io), package set to public — free, avoids Azure Container Registry's ~$5/mo Basic tier

## 1. Dockerfile / port compatibility

The app already reads `PORT` from the environment and falls back to `8080` ([crates/common/src/config.rs](crates/common/src/config.rs)), and binds `0.0.0.0:<port>` ([apps/api/src/main.rs](apps/api/src/main.rs)). The only thing that was Azure-incompatible was the `Dockerfile`, which hardcoded `PORT=7860`/`EXPOSE 7860` for Hugging Face Spaces. That's now `PORT=8080`/`EXPOSE 8080`, matching Container Apps' typical target port. If you ever need a different port, override it at deploy time with `--env-vars PORT=<port>` — no code or image rebuild required.

No other Dockerfile changes were needed: `sqlx::migrate!` embeds migrations into the binary at compile time (they don't need to be copied into the runtime stage), and `configs/app.yaml` isn't read by the app at all (dead file — nothing depends on it at runtime).

`sqlx` is already built with `runtime-tokio-native-tls`, so TLS connections work out of the box — you just need `?sslmode=require` in `DATABASE_URL`, which Azure Postgres Flexible Server enforces by default anyway.

## 2. One-time provisioning

Run [scripts/azure-deploy.ps1](scripts/azure-deploy.ps1) from this directory. It creates the resource group, Postgres Flexible Server + database, allow-lists the Postgres extensions the migrations need, builds/pushes the image to GHCR, and creates the Container App with `DATABASE_URL`/`JWT_SECRET` wired up as secrets. Read the script's header comments for prerequisites (`az login`, the `containerapp` extension, Docker Desktop running).

It prints a generated Postgres admin password and JWT secret — **save them somewhere durable** (a password manager), they aren't stored anywhere else.

### Gotchas hit on this subscription (worth checking on any subscription before assuming defaults work)

- **Region policy.** This subscription has a system Azure Policy capping deployments to five regions: `austriaeast`, `indonesiacentral`, `eastasia`, `koreacentral`, `centralindia` (check via `az policy assignment list -o json`). `eastus`/`eastus2` — the usual tutorial defaults — are blocked outright. Postgres was placed in `centralindia`.
- **Resource provider registration.** A fresh subscription isn't registered for `Microsoft.DBforPostgreSQL` or `Microsoft.App` by default; `az postgres flexible-server create` fails with `MissingSubscriptionRegistration` until you run `az provider register --namespace <ns>` and wait for `registrationState` to reach `Registered` (a minute or two).
- **One Container Apps environment per subscription.** Not per-region — the whole subscription. This one already had `nexuscart-env` (a separate, pre-existing project, `NexusCart-RG`) from before this deployment, so devresume-api was deployed as a second, independent app inside that same environment via `--environment <full-resource-id>` rather than creating a new one. Apps in a shared environment run and scale independently; the only real coupling is that deleting the environment or its resource group takes every app in it down too.
- **Postgres extension allow-listing.** Azure Postgres Flexible Server refuses `CREATE EXTENSION` for anything not explicitly allow-listed first, even extensions bundled with the image (`uuid-ossp`, `vector`/pgvector, both used by this repo's migrations). Without `az postgres flexible-server parameter set --name azure.extensions --value "uuid-ossp,vector"` run *before* the app's first migration attempt, startup fails with `extension "uuid-ossp" is not allow-listed for users in Azure Database for PostgreSQL`. The script now does this automatically.

## 3. Ongoing deploys via GitHub Actions

[.github/workflows/azure-deploy.yml](.github/workflows/azure-deploy.yml) builds a new image and calls `az containerapp update --image ...` on every push to `main`. It uses OIDC (`azure/login`) instead of a stored client secret, which requires one-time setup:

```powershell
# Run once, after step 2 above (resource group must already exist)
$SubscriptionId = az account show --query id -o tsv
$App = az ad app create --display-name "devresume-api-github-deploy" | ConvertFrom-Json
az ad sp create --id $App.appId

az role assignment create `
  --assignee $App.appId `
  --role Contributor `
  --scope "/subscriptions/$SubscriptionId/resourceGroups/devresume-rg"

az ad app federated-credential create `
  --id $App.appId `
  --parameters '{
    "name": "devresume-api-main-branch",
    "issuer": "https://token.actions.githubusercontent.com",
    "subject": "repo:ChamathDilshanC/devresume-api:ref:refs/heads/main",
    "audiences": ["api://AzureADTokenExchange"]
  }'

Write-Host "AZURE_CLIENT_ID:       $($App.appId)"
Write-Host "AZURE_TENANT_ID:       $(az account show --query tenantId -o tsv)"
Write-Host "AZURE_SUBSCRIPTION_ID: $SubscriptionId"
```

Add the three printed values as **GitHub repo secrets** (Settings → Secrets and variables → Actions): `AZURE_CLIENT_ID`, `AZURE_TENANT_ID`, `AZURE_SUBSCRIPTION_ID`. Update the `subject` above if your GitHub org/repo name differs.

The workflow pushes to `ghcr.io/<owner>/devresume-api` using the built-in `GITHUB_TOKEN` (free, no PAT needed to *push*). For Container Apps to *pull* it later, either make the package public once (simplest — the image has no secrets baked in, they're all runtime env vars) or register pull credentials as shown commented-out in `azure-deploy.ps1`.

## 4. Secrets in Azure Container Apps

Never pass secrets as plain `--env-vars` — always create them as ACA secrets first, then reference with `secretref:<name>`. This is what `azure-deploy.ps1` does for `DATABASE_URL`/`JWT_SECRET`; to add more (GitHub/Google OAuth, AI provider keys from `.env.example`):

```powershell
az containerapp secret set `
  --name devresume-api --resource-group devresume-rg `
  --secrets "github-client-id=<value>" "github-client-secret=<value>" "openai-api-key=<value>"

az containerapp update `
  --name devresume-api --resource-group devresume-rg `
  --set-env-vars `
    "GITHUB_CLIENT_ID=secretref:github-client-id" `
    "GITHUB_CLIENT_SECRET=secretref:github-client-secret" `
    "OPENAI_API_KEY=secretref:openai-api-key"
```

To rotate a secret, run `az containerapp secret set` again with the new value, then `az containerapp revision restart` (or push a new revision) to pick it up — secrets aren't hot-reloaded into a running container.

To inspect what's currently set (values are redacted, names only):

```powershell
az containerapp secret list --name devresume-api --resource-group devresume-rg -o table
az containerapp show --name devresume-api --resource-group devresume-rg --query "properties.template.containers[0].env"
```

## 5. Verification

```powershell
$Fqdn = az containerapp show --name devresume-api --resource-group devresume-rg `
  --query properties.configuration.ingress.fqdn -o tsv

curl "https://$Fqdn/health"        # service metadata — status, version
curl "https://$Fqdn/health/live"   # liveness probe
curl "https://$Fqdn/health/ready"  # readiness probe (db/AI pipeline)
```

Confirmed live (2026-08-20):

```
$ curl https://devresume-api.salmondune-b6d2a6eb.centralindia.azurecontainerapps.io/health
{"architecture":"enterprise-modular-monolith-9.8","author":"ChamathDilshanC <dilshancolonne123@gmail.com>","service":"devresume-api","status":"healthy","version":"1.0.0"}

$ curl https://devresume-api.salmondune-b6d2a6eb.centralindia.azurecontainerapps.io/health/ready
{"ai_pipeline":"ready","database":"connected","status":"ready"}
```

The FQDN follows the pattern `https://<app-name>.<random-suffix>.<region>.azurecontainerapps.io`. A `200` with `{"status":"healthy",...}` from `/health` confirms the container started, bound to the right port, and Azure's ingress is routing to it. `/health/ready` doesn't currently check the DB connection in code ([apps/api/src/routes.rs](apps/api/src/routes.rs) — it's a static response) — so a real DB connectivity check is: watch the startup logs for `Database migrations applied successfully.`:

```powershell
az containerapp logs show --name devresume-api --resource-group devresume-rg --tail 30
```

If migrations fail there, check in this order: (1) the Postgres extension allow-list (Section 2's gotcha — the actual cause hit here), (2) the Postgres firewall rule (Section 2's `--public-access 0.0.0.0`), (3) a missing `sslmode=require` in `DATABASE_URL`.

Since `min-replicas 0` is set for cost control, the app scales to zero when idle — the first request after a quiet period is a cold start (image pull + Rust process boot + migration check) and can take 20–40+ seconds. Subsequent requests are fast until it scales back down.
