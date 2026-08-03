# Prompt: Project Summary Generator

**Used by**: `crates/parser/src/service.rs` → `ParserService::summarize_project()`
**Provider**: Any `AIProvider` implementation
**Output format**: Structured JSON

---

## System Prompt

```
You are a senior software engineer analyzing GitHub repositories to produce concise, accurate project summaries.

Rules:
- Identify the PRIMARY purpose of the project in one sentence.
- Detect the technology stack accurately from file manifests and README content.
- Assess project maturity based on: commit count, star count, test presence, CI configuration, documentation quality.
- Identify the target audience (developer tool, consumer app, library, API, etc.).
- Never guess technologies not evidenced in the provided data.
- Output must be valid JSON.
```

---

## User Prompt Template

```
Analyze this GitHub repository and generate a structured project summary.

# Repository Data
Name: {{repo.name}}
Description: {{repo.description}}
Primary Language: {{repo.primary_language}}
Stars: {{repo.stars}} | Forks: {{repo.forks}}
Total Commits: {{repo.commit_count}}
Open Issues: {{repo.open_issues_count}}
License: {{repo.license}}
Created: {{repo.created_at}} | Last Push: {{repo.pushed_at}}

# Detected File Manifests
{{#each repo.manifest_files}}
File: {{this.filename}}
Content snippet:
{{this.content_snippet}}
---
{{/each}}

# README Content
{{repo.readme_content | truncate 3000}}

# Recent Commit Messages (last 20)
{{#each repo.recent_commits}}
- {{this.message}}
{{/each}}

Output JSON:
{
  "summary": "string",              // 1-2 sentence project description
  "category": "string",             // api | library | cli | web_app | mobile | data | devops | other
  "maturity": "string",             // prototype | early | stable | production
  "technologies": {
    "languages": ["string"],
    "frameworks": ["string"],
    "databases": ["string"],
    "cloud": ["string"],
    "devops": ["string"],
    "testing": ["string"]
  },
  "architecture_patterns": ["string"],  // e.g. microservices, monolith, serverless, event_driven
  "key_features": ["string"],           // max 5 bullet points
  "impact_score": number,              // 0-100 based on stars, commits, recency
  "has_tests": boolean,
  "has_ci": boolean,
  "has_documentation": boolean,
  "is_public_facing": boolean
}
```

---

## Prompt Variables

| Variable | Source | Required |
|----------|--------|---------|
| `repo.name` | `repositories.name` | ✅ |
| `repo.description` | `repositories.description` | ⬜ |
| `repo.primary_language` | `repositories.primary_language` | ✅ |
| `repo.readme_content` | `repositories.readme_content` | ⬜ |
| `repo.manifest_files` | Parsed from repo filesystem | ⬜ |
| `repo.recent_commits` | `commits.message` last 20 | ✅ |
