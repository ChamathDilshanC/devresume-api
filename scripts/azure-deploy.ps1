<#
.SYNOPSIS
  One-time provisioning of devresume-api's Azure Container App (Free Tier friendly).

.DESCRIPTION
  Creates:
    1. Resource Group
    2. A GHCR-hosted image, built and pushed from the local Dockerfile
    3. The devresume-api Container App itself, wired up with DATABASE_URL / JWT_SECRET
       as secrets and scale-to-zero enabled

  The database is Supabase Postgres, not Azure — pass its connection string via
  -DatabaseUrl (Supabase Dashboard → Settings → Database → Connection string →
  "Transaction pooler" tab; NOT "Direct connection", which is IPv6-only and
  unreachable from Container Apps' IPv4-only outbound). URL-encode special
  characters in the password (e.g. `@` -> `%40`) and append `?sslmode=require`.
  See AZURE_DEPLOYMENT.md Section 2 for the full walkthrough and why this
  replaced an earlier Azure Postgres Flexible Server setup (cost).

  NOTE: this subscription allows only ONE Container Apps environment total
  (MaxNumberOfGlobalEnvironmentsInSubExceeded), and one already exists — "nexuscart-env"
  in resource group NexusCart-RG (a separate, pre-existing project on this account). So
  this script does NOT create a new environment; it deploys devresume-api as a second,
  independent app inside that existing environment via --environment <full-resource-id>.
  Apps in a shared Container Apps environment run and scale independently of each other.

  Run this once from the devresume-api/ directory. Re-running is mostly safe — `az`
  will error on resources that already exist, but won't silently duplicate anything.

.PARAMETER DatabaseUrl
  Supabase Supavisor transaction-pooler connection string, with password URL-encoded
  and `?sslmode=require` appended.

.PARAMETER JwtSecret
  Optional. If omitted, a random 48-char secret is generated and printed once.

.PREREQUISITES
  - Azure CLI >= 2.60, logged in: az login
  - Container Apps extension:      az extension add --name containerapp --upgrade
  - Docker Desktop running
  - Push access to ghcr.io for your GitHub account/org (docker login ghcr.io)

.EXAMPLE
  ./azure-deploy.ps1 -DatabaseUrl "postgresql://postgres.xxxx:pw%40123@aws-0-ap-northeast-1.pooler.supabase.com:6543/postgres?sslmode=require"
#>

param(
  [Parameter(Mandatory = $true)]
  [string]$DatabaseUrl,

  [string]$JwtSecret = (-join ((48..57) + (65..90) + (97..122) | Get-Random -Count 48 | ForEach-Object { [char]$_ }))
)

$ErrorActionPreference = "Stop"

# ---------------------------------------------------------------------------
# Configurable variables — adjust before running
# ---------------------------------------------------------------------------
$ResourceGroup = "devresume-rg"

# This subscription has a system Azure Policy ("Allowed resource deployment regions")
# restricting deployments to only: austriaeast, indonesiacentral, eastasia, koreacentral,
# centralindia (check via `az policy assignment list -o json`).
$Location = "centralindia"

# Existing Container Apps environment this subscription is limited to (see NOTE above).
# Get with: az containerapp env show --name nexuscart-env --resource-group NexusCart-RG --query id -o tsv
$AcaEnvId   = "/subscriptions/29145745-d1e4-43ec-bbac-3f1fca977d02/resourceGroups/NexusCart-RG/providers/Microsoft.App/managedEnvironments/nexuscart-env"
$AcaAppName = "devresume-api"

$GhcrOwner = "ChamathDilshanC"   # GitHub user/org that will own the image
$ImageName = "ghcr.io/$($GhcrOwner.ToLower())/devresume-api:latest"

Write-Host "`nJWT secret in use — save this now if it was just generated:" -ForegroundColor Yellow
Write-Host "  $JwtSecret`n" -ForegroundColor Yellow

# ---------------------------------------------------------------------------
# 0. Extensions
# ---------------------------------------------------------------------------
az extension add --name containerapp --upgrade --only-show-errors

# ---------------------------------------------------------------------------
# 1. Resource Group
# ---------------------------------------------------------------------------
az group create --name $ResourceGroup --location $Location

# ---------------------------------------------------------------------------
# 2. Build & push the image to GHCR (free registry — avoids ACR's ~$5/mo Basic tier)
#    Run `docker login ghcr.io -u $GhcrOwner` first if you haven't already
#    (password = a GitHub PAT with `write:packages` scope).
# ---------------------------------------------------------------------------
docker build -t $ImageName .
docker push $ImageName

Write-Host "`nAfter this first push, go to https://github.com/users/$GhcrOwner/packages/container/devresume-api/settings" -ForegroundColor Cyan
Write-Host "and set the package visibility to Public, so Container Apps can pull it without extra credentials." -ForegroundColor Cyan
Write-Host "(If you'd rather keep it private, see the 'private GHCR' note in AZURE_DEPLOYMENT.md.)`n" -ForegroundColor Cyan
Read-Host "Press Enter once the package is public (or credentials are set up) to continue"

# ---------------------------------------------------------------------------
# 3. The Container App — deployed into the existing nexuscart-env (see NOTE above),
#    not a new environment. Secrets stay as ACA "secretref"s, never plain env vars.
# ---------------------------------------------------------------------------
az containerapp create `
  --name $AcaAppName `
  --resource-group $ResourceGroup `
  --environment $AcaEnvId `
  --image $ImageName `
  --target-port 8080 `
  --ingress external `
  --min-replicas 0 `
  --max-replicas 3 `
  --cpu 0.5 --memory 1.0Gi `
  --secrets "database-url=$DatabaseUrl" "jwt-secret=$JwtSecret" `
  --env-vars "DATABASE_URL=secretref:database-url" "JWT_SECRET=secretref:jwt-secret" "PORT=8080" "ENVIRONMENT=production"

# If you kept the GHCR image private instead of making it public, register pull
# credentials once (PAT needs `read:packages` scope):
#
# az containerapp registry set `
#   --name $AcaAppName --resource-group $ResourceGroup `
#   --server ghcr.io --username $GhcrOwner --password <GHCR_PAT>

# To add OAuth/AI keys later, extend the --secrets / --env-vars pairs above, e.g.:
#
# az containerapp secret set --name $AcaAppName --resource-group $ResourceGroup `
#   --secrets "github-client-secret=<value>" "openai-api-key=<value>"
# az containerapp update --name $AcaAppName --resource-group $ResourceGroup `
#   --set-env-vars "GITHUB_CLIENT_SECRET=secretref:github-client-secret" "OPENAI_API_KEY=secretref:openai-api-key"

# ---------------------------------------------------------------------------
# 4. Print the public URL
# ---------------------------------------------------------------------------
$Fqdn = az containerapp show --name $AcaAppName --resource-group $ResourceGroup --query properties.configuration.ingress.fqdn -o tsv
Write-Host "`nDeployed. Verify with:" -ForegroundColor Green
Write-Host "  curl https://$Fqdn/health`n" -ForegroundColor Green

# ---------------------------------------------------------------------------
# HISTORICAL — Azure Database for PostgreSQL Flexible Server provisioning.
# Not executed by this script anymore (database moved to Supabase 2026-08-20 for
# cost reasons). Kept here for reference only — running this would create a new
# paid Azure Postgres server. See AZURE_DEPLOYMENT.md Section 2 for why, and for
# the CREATE EXTENSION allow-listing gotcha this setup required.
# ---------------------------------------------------------------------------
<#
$PgServerName    = "devresume-pg-$(Get-Random -Minimum 1000 -Maximum 9999)"
$PgAdminUser     = "devresumeadmin"
$PgAdminPassword = -join ((48..57) + (65..90) + (97..122) | Get-Random -Count 24 | ForEach-Object { [char]$_ })
$DbName          = "devresume_db"

az postgres flexible-server create `
  --resource-group $ResourceGroup --name $PgServerName --location $Location `
  --admin-user $PgAdminUser --admin-password $PgAdminPassword `
  --sku-name Standard_B1ms --tier Burstable --storage-size 32 --version 16 `
  --public-access 0.0.0.0 --yes

az postgres flexible-server db create `
  --resource-group $ResourceGroup --server-name $PgServerName --name $DbName

az postgres flexible-server parameter set `
  --resource-group $ResourceGroup --server-name $PgServerName `
  --name azure.extensions --value "uuid-ossp,vector"

$DatabaseUrl = "postgres://${PgAdminUser}:${PgAdminPassword}@${PgServerName}.postgres.database.azure.com:5432/${DbName}?sslmode=require"
#>
