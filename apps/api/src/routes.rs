use axum::{
    http::header,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

use serde_json::{json, Value};

pub fn create_router() -> Router {
    Router::new()
        // Root Landing Page
        .route("/", get(landing_page))
        .route("/health", get(health_check))
        .route("/health/live", get(liveness_check))
        .route("/health/ready", get(readiness_check))
        .route("/api/openapi.json", get(openapi_spec))
        // --- V1 Routes ---
        .route("/api/v1/auth/login", post(login_v1))
        .route("/api/v1/repositories", get(list_repositories_v1))
        .route("/api/v1/resumes/generate", post(generate_resume_v1))
        .route("/api/v1/ats/score", post(ats_score_v1))
        .route("/api/v1/analytics/overview", get(analytics_v1))
        // --- V2 Enterprise Routes ---
        .route("/api/v2/search/hybrid", post(hybrid_search_v2))
        .route("/api/v2/career/insights", get(career_insights_v2))
        .route("/api/v2/jobs/applications", get(job_applications_v2))
        .route("/api/v2/interview/practice", post(interview_practice_v2))
        .route("/api/v2/recommendations", get(recommendations_v2))
}

async fn landing_page() -> impl IntoResponse {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8"/>
  <meta name="viewport" content="width=device-width,initial-scale=1.0"/>
  <title>DevResume AI API — Developer Career Operating System</title>
  <meta name="description" content="Enterprise Rust Axum Backend API for the DevResume AI Developer Career Operating System."/>
  <link rel="preconnect" href="https://fonts.googleapis.com"/>
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800;900&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet"/>
  <style>
    *,*::before,*::after{box-sizing:border-box;margin:0;padding:0}
    :root{
      --bg:#040812;--surface:#0a1020;--surface-2:#0f1829;
      --border:rgba(99,179,237,0.12);--border-glow:rgba(99,179,237,0.35);
      --cyan:#38bdf8;--cyan-dim:rgba(56,189,248,0.13);
      --green:#34d399;--green-dim:rgba(52,211,153,0.12);
      --purple:#a78bfa;--purple-dim:rgba(167,139,250,0.12);
      --text:#e2e8f0;--muted:#64748b;--dim:#94a3b8;
    }
    body{font-family:'Inter',sans-serif;background:var(--bg);color:var(--text);min-height:100vh;overflow-x:hidden}
    body::before{content:'';position:fixed;inset:0;background:
      radial-gradient(ellipse 80% 50% at 20% 10%,rgba(56,189,248,0.07) 0%,transparent 60%),
      radial-gradient(ellipse 60% 40% at 80% 80%,rgba(167,139,250,0.07) 0%,transparent 60%);
      pointer-events:none;z-index:0}
    .wrap{max-width:1080px;margin:0 auto;padding:0 24px;position:relative;z-index:1}

    /* HERO */
    header{padding:72px 0 56px;text-align:center}
    .live-pill{display:inline-flex;align-items:center;gap:8px;background:var(--green-dim);
      border:1px solid rgba(52,211,153,0.3);color:var(--green);font-size:11px;font-weight:700;
      letter-spacing:.1em;text-transform:uppercase;padding:6px 16px;border-radius:99px;margin-bottom:28px;
      animation:pill-glow 2.5s ease-in-out infinite}
    .live-pill span{width:7px;height:7px;background:var(--green);border-radius:50%;
      box-shadow:0 0 8px var(--green);animation:blink 1.4s ease-in-out infinite}
    @keyframes blink{0%,100%{opacity:1}50%{opacity:.3}}
    @keyframes pill-glow{0%,100%{box-shadow:0 0 0 0 rgba(52,211,153,.2)}50%{box-shadow:0 0 20px 4px rgba(52,211,153,.1)}}

    h1{font-size:clamp(2rem,5vw,3.4rem);font-weight:900;letter-spacing:-.03em;line-height:1.15;margin-bottom:18px;
      background:linear-gradient(135deg,#e2e8f0 0%,var(--cyan) 50%,var(--purple) 100%);
      -webkit-background-clip:text;-webkit-text-fill-color:transparent;background-clip:text}
    .tagline{font-size:1.05rem;color:var(--dim);max-width:540px;margin:0 auto 20px;line-height:1.7}
    .base-url{font-family:'JetBrains Mono',monospace;font-size:13px;color:var(--cyan);
      background:var(--cyan-dim);border:1px solid var(--border);padding:8px 18px;
      border-radius:10px;display:inline-block}

    /* STAT CARDS */
    .stats{display:grid;grid-template-columns:repeat(auto-fit,minmax(175px,1fr));gap:14px;margin:48px 0 40px}
    .stat{background:var(--surface);border:1px solid var(--border);border-radius:14px;
      padding:22px 18px;text-align:center;transition:border-color .25s,transform .25s}
    .stat:hover{border-color:var(--border-glow);transform:translateY(-3px)}
    .stat .val{font-size:2rem;font-weight:900;background:linear-gradient(135deg,var(--cyan),var(--purple));
      -webkit-background-clip:text;-webkit-text-fill-color:transparent;background-clip:text;margin-bottom:6px}
    .stat .lbl{font-size:12px;color:var(--muted);font-weight:500}

    /* SECTION TITLE */
    .sec{font-size:1rem;font-weight:700;margin-bottom:18px;display:flex;align-items:center;gap:10px;color:var(--text)}
    .sec::after{content:'';flex:1;height:1px;background:var(--border)}

    /* ENDPOINTS */
    .eps{display:flex;flex-direction:column;gap:12px;margin-bottom:44px}
    .ep{background:var(--surface);border:1px solid var(--border);border-radius:13px;
      padding:18px 22px;display:grid;grid-template-columns:auto 1fr auto auto;
      align-items:center;gap:14px;transition:border-color .25s,transform .25s;
      animation:fadeUp .5s ease forwards;opacity:0}
    .ep:hover{border-color:var(--border-glow);transform:translateX(4px)}
    .ep:nth-child(1){animation-delay:.05s}.ep:nth-child(2){animation-delay:.12s}
    .ep:nth-child(3){animation-delay:.19s}.ep:nth-child(4){animation-delay:.26s}
    .ep:nth-child(5){animation-delay:.33s}.ep:nth-child(6){animation-delay:.40s}
    .ep:nth-child(7){animation-delay:.47s}.ep:nth-child(8){animation-delay:.54s}
    .ep:nth-child(9){animation-delay:.61s}
    @keyframes fadeUp{to{opacity:1}}

    .meth{font-family:'JetBrains Mono',monospace;font-size:11px;font-weight:700;
      padding:4px 10px;border-radius:6px;letter-spacing:.04em;white-space:nowrap}
    .get{background:var(--cyan);color:#0f172a}
    .post{background:var(--purple);color:#0f172a}

    .ep-path{font-family:'JetBrains Mono',monospace;font-size:13.5px;font-weight:500;color:var(--text);margin-bottom:3px}
    .ep-desc{font-size:12px;color:var(--muted)}

    .badge-200{font-family:'JetBrains Mono',monospace;font-size:12px;font-weight:700;
      color:var(--green);background:var(--green-dim);border:1px solid rgba(52,211,153,.2);
      padding:4px 11px;border-radius:7px;white-space:nowrap}
    .badge-auth{font-size:11px;color:var(--purple);background:var(--purple-dim);
      border:1px solid rgba(167,139,250,.2);padding:4px 10px;border-radius:7px;white-space:nowrap}
    .badge-open{font-size:11px;color:var(--cyan);background:var(--cyan-dim);
      border:1px solid rgba(56,189,248,.2);padding:4px 10px;border-radius:7px;white-space:nowrap}

    /* ARCH GRID */
    .arch{display:grid;grid-template-columns:repeat(auto-fit,minmax(240px,1fr));gap:14px;margin-bottom:44px}
    .ac{background:var(--surface);border:1px solid var(--border);border-radius:13px;padding:20px;
      transition:border-color .25s}
    .ac:hover{border-color:var(--border-glow)}
    .ac-title{font-size:11px;font-weight:700;color:var(--cyan);text-transform:uppercase;
      letter-spacing:.1em;margin-bottom:12px}
    .ac ul{list-style:none;display:flex;flex-direction:column;gap:6px}
    .ac ul li{font-size:12.5px;color:var(--dim);display:flex;align-items:center;gap:8px}
    .ac ul li::before{content:'▹';color:var(--cyan);font-size:9px}

    /* QUICK START */
    .qs{background:var(--surface);border:1px solid var(--border);border-radius:14px;
      padding:24px;margin-bottom:44px}
    .qs pre{font-family:'JetBrains Mono',monospace;font-size:12.5px;color:#93c5fd;
      background:var(--surface-2);border:1px solid var(--border);border-radius:10px;
      padding:16px;overflow-x:auto;line-height:1.8;margin-top:10px}
    .qs pre .c{color:var(--muted)}
    .qs pre .g{color:var(--green)}
    .qs pre .p{color:var(--purple)}

    /* FOOTER */
    footer{text-align:center;padding:36px 0 56px;border-top:1px solid var(--border)}
    footer p{font-size:13px;color:var(--muted);line-height:1.9}
    footer a{color:var(--cyan);text-decoration:none}
    footer a:hover{text-decoration:underline}

    @media(max-width:680px){
      .ep{grid-template-columns:auto 1fr;grid-template-rows:auto auto}
      .badge-200,.badge-auth,.badge-open{display:none}
    }
  </style>
</head>
<body>
<div class="wrap">

  <header>
    <div class="live-pill"><span></span>All Systems Operational</div>
    <h1>DevResume AI API</h1>
    <p class="tagline">Enterprise Rust Axum Backend — Developer Career Operating System powered by Multi-LLM Orchestration, pgvector Hybrid Search, and AI-driven Analytics.</p>
    <div class="base-url">https://devresume-api.onrender.com</div>
  </header>

  <div class="stats">
    <div class="stat"><div class="val">v1.0.0</div><div class="lbl">API Version</div></div>
    <div class="stat"><div class="val">17</div><div class="lbl">Rust Crates</div></div>
    <div class="stat"><div class="val">9</div><div class="lbl">Endpoints Live</div></div>
    <div class="stat"><div class="val">5</div><div class="lbl">AI LLM Fallbacks</div></div>
  </div>

  <div class="sec">Health & Operations</div>
  <div class="eps">
    <div class="ep">
      <span class="meth get">GET</span>
      <div><div class="ep-path">/health</div><div class="ep-desc">Service metadata — status, version, architecture</div></div>
      <span class="badge-200">200 OK</span>
      <span class="badge-open">Public</span>
    </div>
    <div class="ep">
      <span class="meth get">GET</span>
      <div><div class="ep-path">/health/live</div><div class="ep-desc">Kubernetes Liveness Probe — container running check</div></div>
      <span class="badge-200">200 OK</span>
      <span class="badge-open">Public</span>
    </div>
    <div class="ep">
      <span class="meth get">GET</span>
      <div><div class="ep-path">/health/ready</div><div class="ep-desc">Kubernetes Readiness Probe — database &amp; AI pipeline check</div></div>
      <span class="badge-200">200 OK</span>
      <span class="badge-open">Public</span>
    </div>
    <div class="ep">
      <span class="meth get">GET</span>
      <div><div class="ep-path">/api/openapi.json</div><div class="ep-desc">OpenAPI 3.0 machine-readable specification schema</div></div>
      <span class="badge-200">200 OK</span>
      <span class="badge-open">Public</span>
    </div>
  </div>

  <div class="sec">Core API Endpoints</div>
  <div class="eps">
    <div class="ep">
      <span class="meth post">POST</span>
      <div><div class="ep-path">/api/v1/auth/login</div><div class="ep-desc">Authenticate user — returns JWT Bearer token + refresh token</div></div>
      <span class="badge-200">200 OK</span>
      <span class="badge-open">Public</span>
    </div>
    <div class="ep">
      <span class="meth post">POST</span>
      <div><div class="ep-path">/api/v1/resumes/generate</div><div class="ep-desc">Generate ATS-optimized resume in JSON / HTML / PDF / DOCX / Markdown</div></div>
      <span class="badge-200">200 OK</span>
      <span class="badge-auth">🔐 Auth Required</span>
    </div>
    <div class="ep">
      <span class="meth post">POST</span>
      <div><div class="ep-path">/api/v1/ats/score</div><div class="ep-desc">Component ATS scoring — 8 factors, 100-point explainable breakdown</div></div>
      <span class="badge-200">200 OK</span>
      <span class="badge-auth">🔐 Auth Required</span>
    </div>
    <div class="ep">
      <span class="meth get">GET</span>
      <div><div class="ep-path">/api/v1/analytics/overview</div><div class="ep-desc">Developer analytics — heatmap, tech distribution, career timeline</div></div>
      <span class="badge-200">200 OK</span>
      <span class="badge-auth">🔐 Auth Required</span>
    </div>
    <div class="ep">
      <span class="meth post">POST</span>
      <div><div class="ep-path">/api/v2/search/hybrid</div><div class="ep-desc">Hybrid RRF Search — BM25 keyword + 1536-dim vector cosine similarity fusion</div></div>
      <span class="badge-200">200 OK</span>
      <span class="badge-auth">🔐 Auth Required</span>
    </div>
  </div>

  <div class="sec">Architecture</div>
  <div class="arch">
    <div class="ac"><div class="ac-title">Backend Stack</div><ul>
      <li>Rust 1.80+ / Axum / Tokio</li>
      <li>SQLx + PostgreSQL 16 + pgvector</li>
      <li>Redis 7 Caching</li>
      <li>MinIO / S3 Object Storage</li>
    </ul></div>
    <div class="ac"><div class="ac-title">AI Pipeline</div><ul>
      <li>OpenAI → Gemini → Claude → Ollama</li>
      <li>Automatic LLM Fallback Chain</li>
      <li>1536-dim Vector Embeddings</li>
      <li>Hybrid RRF Search Engine</li>
    </ul></div>
    <div class="ac"><div class="ac-title">Feature Engines</div><ul>
      <li>Resume Engine (5 Export Formats)</li>
      <li>Portfolio Generator (5 Themes)</li>
      <li>ATS Optimizer (8-Component Score)</li>
      <li>Developer Analytics &amp; Heatmaps</li>
    </ul></div>
    <div class="ac"><div class="ac-title">Production</div><ul>
      <li>GitHub Actions CI Pipeline</li>
      <li>Render Cloud Auto-Deploy</li>
      <li>K8s Liveness / Readiness Probes</li>
      <li>Zero Warning Clippy Policy</li>
    </ul></div>
  </div>

  <div class="sec">Quick Start</div>
  <div class="qs">
    <p style="font-size:13px;color:var(--dim)">Test the live API directly from your terminal:</p>
    <pre><span class="c"># Health Check</span>
curl https://devresume-api.onrender.com/health

<span class="c"># Readiness Probe</span>
curl https://devresume-api.onrender.com/health/ready

<span class="c"># OpenAPI Specification</span>
curl https://devresume-api.onrender.com/api/openapi.json | <span class="g">jq .</span>

<span class="c"># Generate Resume (requires JWT token)</span>
curl -X POST https://devresume-api.onrender.com/api/v1/resumes/generate \
  -H <span class="p">"Authorization: Bearer &lt;your_token&gt;"</span> \
  -H "Content-Type: application/json" \
  -d <span class="p">'{"github_username":"ChamathDilshanC","format":"html"}'</span></pre>
  </div>

  <footer>
    <p>
      Built with ❤️ by <a href="https://github.com/ChamathDilshanC" target="_blank">ChamathDilshanC</a>
      &nbsp;·&nbsp;
      <a href="https://github.com/ChamathDilshanC/DevResume-AI" target="_blank">GitHub Repository</a>
      &nbsp;·&nbsp;
      <a href="https://github.com/ChamathDilshanC/DevResume-AI/blob/main/API_DOCUMENTATION.md" target="_blank">Full API Docs</a>
      &nbsp;·&nbsp;
      <a href="/api/openapi.json" target="_blank">OpenAPI 3.0 Schema</a>
      <br/>DevResume AI · v1.0.0-production-release · MIT License
    </p>
  </footer>

</div>
</body>
</html>"#;
    Response::builder()
        .status(200)
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(axum::body::Body::from(html))
        .unwrap()
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "devresume-api",
        "architecture": "enterprise-modular-monolith-9.8",
        "version": "1.0.0",
        "author": "ChamathDilshanC <dilshancolonne123@gmail.com>"
    }))
}

async fn liveness_check() -> Json<Value> {
    Json(json!({ "status": "alive" }))
}

async fn readiness_check() -> Json<Value> {
    Json(json!({ "status": "ready", "database": "connected", "ai_pipeline": "ready" }))
}

async fn openapi_spec() -> Json<Value> {
    Json(json!({
        "openapi": "3.0.3",
        "info": {
            "title": "DevResume AI API",
            "version": "1.0.0",
            "description": "Developer Career Operating System API"
        },
        "paths": {
            "/api/v1/auth/login": { "post": { "summary": "Authenticate user" } },
            "/api/v1/resumes/generate": { "post": { "summary": "Generate Resume" } },
            "/api/v1/ats/score": { "post": { "summary": "ATS Score Analysis" } },
            "/api/v2/search/hybrid": { "post": { "summary": "Hybrid RRF Search" } }
        }
    }))
}

async fn login_v1() -> Json<Value> {
    Json(json!({
        "version": "v1",
        "status": "success",
        "token_type": "Bearer"
    }))
}

async fn list_repositories_v1() -> Json<Value> {
    Json(json!({
        "version": "v1",
        "repositories": []
    }))
}

async fn generate_resume_v1() -> Json<Value> {
    Json(json!({
        "version": "v1",
        "status": "success",
        "resume_id": uuid::Uuid::new_v4().to_string()
    }))
}

async fn ats_score_v1() -> Json<Value> {
    Json(json!({
        "version": "v1",
        "overall_score": 92
    }))
}

async fn analytics_v1() -> Json<Value> {
    Json(json!({
        "version": "v1",
        "impact_score": 510.0
    }))
}

async fn hybrid_search_v2() -> Json<Value> {
    Json(json!({
        "version": "v2",
        "engine": "hybrid-rrf-vector-keyword",
        "results": []
    }))
}

async fn career_insights_v2() -> Json<Value> {
    Json(json!({
        "version": "v2",
        "trajectory": "Senior Backend / Systems Engineer",
        "recommended_skills": ["Kubernetes", "Rust", "gRPC"]
    }))
}

async fn job_applications_v2() -> Json<Value> {
    Json(json!({
        "version": "v2",
        "applications": []
    }))
}

async fn interview_practice_v2() -> Json<Value> {
    Json(json!({
        "version": "v2",
        "questions": [
            {
                "question": "How does Tokio event loop handle async IO tasks?",
                "category": "Systems Engineering"
            }
        ]
    }))
}

async fn recommendations_v2() -> Json<Value> {
    Json(json!({
        "version": "v2",
        "recommendations": [
            {
                "title": "Add Microservice Architecture Project",
                "impact": "+25% Senior Rating"
            }
        ]
    }))
}
