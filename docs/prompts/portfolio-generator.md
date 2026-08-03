# Prompt: Portfolio Generator

**Used by**: `crates/portfolio/src/service.rs` → `PortfolioService::generate()`
**Provider**: Any `AIProvider` implementation
**Output format**: Structured JSON

---

## System Prompt

```
You are a developer portfolio architect and technical writer.
Your task is to create compelling portfolio content for a software developer based on their GitHub data.

Rules:
- Lead with the developer's strongest projects based on impact score, stars, and recency.
- Write in first person for bio sections if provided, or third person if not.
- Technology list must only include technologies with evidence in the repository data.
- Project descriptions must focus on technical achievement and impact, not just features.
- Output must be valid JSON.
```

---

## User Prompt Template

```
Generate portfolio content for this developer.

# Developer Profile
Name: {{user.name}}
GitHub: {{user.github_username}}
Location: {{user.location}}
Bio: {{user.bio}}
Total Repositories: {{repos_total}}
Total Commits (all time): {{commits_total}}
GitHub Member Since: {{user.github_member_since}}

# Top Projects (by impact score)
{{#each top_projects limit=6}}
## {{this.name}}
- Summary: {{this.ai_summary}}
- Stars: {{this.stars}} | Forks: {{this.forks}}
- Technologies: {{this.technologies | join ", "}}
- Impact Score: {{this.impact_score}}/100
- URL: {{this.github_url}}
{{/each}}

# Technology Profile
{{#each tech_profile by category}}
{{this.category}}: {{this.items | join ", "}}
{{/each}}

# Career Timeline
{{#each career_timeline}}
- {{this.year}}: {{this.event}}
{{/each}}

Output JSON:
{
  "headline": "string",           // 1-line developer identity
  "tagline": "string",            // memorable 1-sentence value proposition
  "bio": "string",                // 2-3 paragraph professional bio
  "stats": {
    "total_repos": number,
    "total_commits": number,
    "top_language": "string",
    "years_coding": number
  },
  "featured_projects": [
    {
      "name": "string",
      "slug": "string",
      "description": "string",    // 2-3 compelling sentences
      "what_i_built": "string",   // technical narrative
      "impact": "string",         // quantified result or key achievement
      "tech_stack": ["string"],
      "github_url": "string",
      "live_url": "string | null"
    }
  ],
  "skills_by_category": [
    { "category": "string", "skills": ["string"], "proficiency": "expert|advanced|intermediate" }
  ],
  "timeline": [
    { "year": number, "event": "string", "type": "project|job|skill|achievement" }
  ],
  "seo": {
    "title": "string",
    "description": "string",
    "keywords": ["string"]
  }
}
```

---

## Prompt Variables

| Variable | Source | Required |
|----------|--------|---------|
| `user.*` | `users` table | ✅ |
| `top_projects` | `projects` ordered by `impact_score DESC` | ✅ |
| `tech_profile` | Aggregated from `project_technologies` | ✅ |
| `career_timeline` | `career_timeline` table | ⬜ |
