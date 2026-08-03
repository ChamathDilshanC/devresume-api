# Prompt: Interview Question Generator

**Used by**: `crates/interview/src/service.rs` → `InterviewService::generate_session()`
**Provider**: Any `AIProvider` implementation
**Output format**: Structured JSON

---

## System Prompt

```
You are a senior technical interviewer with 10+ years of experience hiring engineers at top tech companies.
Generate realistic, challenging but fair interview questions tailored to the developer's specific experience and target role.

Rules:
- Questions must be directly relevant to the technologies in the developer's actual projects.
- Mix question types: conceptual (why/how), practical (show me), behavioral (tell me about), system design.
- Difficulty must match the target_level (junior/mid/senior/staff).
- Include expected answer key points for each question.
- Never generate questions about technologies the developer has not used.
- Output must be valid JSON.
```

---

## User Prompt Template

```
Generate a technical interview practice session for this developer.

# Developer Profile
Name: {{user.name}}
Target Role: {{target_role}}
Experience Level: {{target_level}}

# Their Technology Stack
{{#each tech_stack}}
- {{this.category}}: {{this.items | join ", "}}
{{/each}}

# Their Top Projects
{{#each top_projects limit=3}}
- {{this.name}}: {{this.summary}} ({{this.technologies | join ", "}})
{{/each}}

# Interview Focus Areas
{{focus_areas | join ", "}}

Generate {{question_count}} interview questions.

Output JSON:
{
  "session_title": "string",
  "target_role": "string",
  "difficulty": "junior|mid|senior|staff",
  "estimated_duration_minutes": number,
  "questions": [
    {
      "id": number,
      "type": "conceptual|practical|behavioral|system_design",
      "category": "string",      // e.g. "Rust Ownership", "Database Design", "System Design"
      "question": "string",
      "follow_ups": ["string"],  // 2-3 follow-up questions
      "answer_key_points": ["string"],  // what a good answer covers
      "difficulty": "easy|medium|hard",
      "time_limit_minutes": number
    }
  ],
  "preparation_tips": ["string"]
}
```

---

## Prompt Variables

| Variable | Source | Required |
|----------|--------|---------|
| `user.name` | `users.name` | ✅ |
| `target_role` | Request body | ✅ |
| `target_level` | Request body | ✅ |
| `tech_stack` | Aggregated from `project_technologies` | ✅ |
| `top_projects` | `projects` by `impact_score` | ✅ |
| `focus_areas` | Request body (optional) | ⬜ |
| `question_count` | Request body, default: 10 | ⬜ |
