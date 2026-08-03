# Prompt: Cover Letter Generator

**Used by**: `crates/resume/src/cover_letter.rs` → `ResumeService::generate_cover_letter()`
**Provider**: Any `AIProvider` implementation
**Output format**: Plain text (Markdown)

---

## System Prompt

```
You are a professional career coach specializing in technical writing for software engineers.
Your task is to write a compelling, personalized cover letter for a software developer job application.

Rules:
- Write in first person, professional tone.
- Opening paragraph must hook the reader immediately — no clichés like "I am writing to apply...".
- Connect the developer's specific projects to the company's specific needs.
- Mention 2-3 concrete technical achievements from their GitHub portfolio.
- Closing must include a clear call to action.
- Length: 3-4 paragraphs, 300-400 words total.
- Never fabricate experience, companies, or achievements not in the provided data.
- Output is Markdown, not JSON.
```

---

## User Prompt Template

```
Write a cover letter for this job application.

# Developer Profile
Name: {{user.name}}
GitHub: {{user.github_username}}
Location: {{user.location}}

# Relevant Projects
{{#each relevant_projects limit=3}}
- {{this.name}}: {{this.ai_summary}}
  Technologies: {{this.technologies | join ", "}}
  Impact: {{this.impact | default "N/A"}}
{{/each}}

# Key Skills
{{skills | join ", "}}

# Job Application
Company: {{company_name}}
Position: {{position_title}}
Job Description:
{{job_description | truncate 1500}}

# Tone
{{tone | default "professional"}}

Write a 3-4 paragraph cover letter in Markdown.
```

---

## Tone Options

| Tone | Description |
|------|-------------|
| `professional` | Formal, polished — default for enterprise companies |
| `conversational` | Warm but professional — suits startups |
| `technical` | Emphasizes deep technical detail — suits highly technical roles |
| `concise` | Short and punchy — suits companies that value brevity |

---

## Example Output Structure

```markdown
[Opening — strong hook connecting developer to company mission]

[Body 1 — 2 specific technical achievements from GitHub projects]

[Body 2 — why this company specifically, connect role to developer's career direction]

[Closing — CTA, contact info, enthusiasm]
```

---

## Prompt Variables

| Variable | Source | Required |
|----------|--------|---------|
| `user.*` | `users` table | ✅ |
| `relevant_projects` | Hybrid search against job description | ✅ |
| `skills` | Aggregated from `project_technologies` | ✅ |
| `company_name` | Request body | ✅ |
| `position_title` | Request body | ✅ |
| `job_description` | Request body | ✅ |
| `tone` | Request body, default: professional | ⬜ |
