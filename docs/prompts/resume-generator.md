# Prompt: Resume Generator

**Used by**: `crates/resume/src/service.rs` → `ResumeService::generate()`
**Provider**: Any `AIProvider` implementation
**Output format**: Structured JSON

---

## System Prompt

```
You are an expert technical resume writer specializing in software engineering careers.
Your task is to generate a professional, ATS-optimized resume for a software developer.

Rules:
- Write in clear, concise American English.
- Use strong action verbs (Built, Designed, Implemented, Optimized, Led, Reduced, Increased).
- Quantify achievements where possible (e.g., "Reduced API latency by 40%").
- Never fabricate technologies, projects, or companies not in the provided data.
- Never include personal opinions or informal language.
- Format skills in order of proficiency (most proficient first).
- Focus on technical impact and measurable results.
- Output must be valid JSON matching the provided schema exactly.
```

---

## User Prompt Template

```
Generate a professional resume for a software developer with the following profile:

# Developer Profile
Name: {{user.name}}
Location: {{user.location}}
Email: {{user.email}}
GitHub: {{user.github_username}}
Website: {{user.website}}

# Target Role
{{target_role}}

# GitHub Repositories ({{repositories | length}} total)
{{#each repositories}}
## {{this.name}}
- Description: {{this.description}}
- Stars: {{this.stars}} | Forks: {{this.forks}}
- Primary Language: {{this.primary_language}}
- Technologies: {{this.technologies | join ", "}}
- Commits: {{this.commit_count}} (last 12 months: {{this.recent_commits}})
- README Summary: {{this.readme_summary}}
{{/each}}

# Detected Skills
{{#each skills by category}}
## {{this.category}}
{{this.skills | join ", "}}
{{/each}}

# Career Timeline
{{#each career_events}}
- {{this.date}}: {{this.title}} ({{this.type}})
{{/each}}

Output the resume as a JSON object matching this exact schema:
{
  "name": "string",
  "title": "string",           // e.g. "Senior Rust Backend Engineer"
  "summary": "string",         // 2-3 sentence professional summary
  "contact": {
    "email": "string",
    "github": "string",
    "website": "string",
    "location": "string"
  },
  "skills": [
    { "category": "string", "items": ["string"] }
  ],
  "projects": [
    {
      "name": "string",
      "description": "string",  // 1-2 impact-focused sentences
      "technologies": ["string"],
      "impact": "string",        // quantified result if possible
      "url": "string"
    }
  ],
  "open_source": [
    { "repo": "string", "contribution": "string" }
  ],
  "education": [],
  "certifications": []
}
```

---

## Example Output Snippet

```json
{
  "name": "Chamath Dilshan",
  "title": "Senior Rust Backend Engineer",
  "summary": "Production-grade backend engineer specializing in Rust, PostgreSQL, and cloud-native systems. Built and deployed high-throughput APIs serving 100k+ requests/day. Passionate about performance, correctness, and developer tooling.",
  "contact": {
    "email": "dilshancolonne123@gmail.com",
    "github": "ChamathDilshanC",
    "website": "https://devresume.ai",
    "location": "Colombo, Sri Lanka"
  },
  "skills": [
    { "category": "Languages", "items": ["Rust", "TypeScript", "Python", "SQL"] },
    { "category": "Frameworks", "items": ["Axum", "Tokio", "Next.js", "React"] },
    { "category": "Databases", "items": ["PostgreSQL", "Redis", "pgvector"] },
    { "category": "DevOps", "items": ["Docker", "Kubernetes", "GitHub Actions", "Terraform"] }
  ]
}
```

---

## Prompt Variables

| Variable | Source | Required |
|----------|--------|---------|
| `user.name` | `users.name` | ✅ |
| `user.email` | `users.email` | ✅ |
| `user.github_username` | `users.github_username` | ✅ |
| `target_role` | Request body | ✅ |
| `repositories` | `repositories` table | ✅ |
| `skills` | `skill_embeddings` aggregated | ✅ |
| `career_events` | `career_timeline` | ⬜ Optional |
