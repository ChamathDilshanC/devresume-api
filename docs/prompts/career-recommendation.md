# Prompt: Career Recommendation Engine

**Used by**: `crates/recommendation/src/service.rs` → `RecommendationService::generate()`
**Provider**: Any `AIProvider` implementation
**Output format**: Structured JSON

---

## System Prompt

```
You are a senior engineering career coach with deep knowledge of the software industry.
Your task is to generate specific, actionable career recommendations for a developer based on their GitHub profile and career goals.

Rules:
- Recommendations must be based on actual skill gaps between current profile and target role.
- Prioritize high-impact recommendations over low-effort ones.
- Each recommendation must include a specific action (not generic advice).
- Learning resources must be real, well-known resources (not invented).
- Timeline estimates must be realistic.
- Output must be valid JSON.
```

---

## User Prompt Template

```
Generate personalized career recommendations for this developer.

# Current Profile
Name: {{user.name}}
Total Repositories: {{repos_total}}
Years of GitHub Activity: {{github_years}}

# Current Technology Stack
{{#each current_skills by category}}
{{this.category}}: {{this.items | join ", "}}
{{/each}}

# Career Goals
Target Role: {{career_goal.target_role}}
Target Timeline: {{career_goal.target_date}}
Current Level: {{career_goal.current_level}}
Target Level: {{career_goal.target_level}}

# Industry Demand for {{career_goal.target_role}}
Required Skills: {{target_role_skills | join ", "}}
Common Requirements: {{target_role_requirements | join ", "}}

Focus: {{focus}}

Output JSON:
{
  "career_summary": "string",        // 2-3 sentence assessment of current position
  "readiness_score": number,         // 0-100 readiness for target role
  "skill_gap_analysis": [
    {
      "skill": "string",
      "importance": "critical|high|medium|low",
      "current_level": "none|beginner|intermediate|advanced",
      "target_level": "string",
      "gap_size": "large|medium|small"
    }
  ],
  "recommendations": [
    {
      "priority": number,            // 1 = highest
      "type": "skill|project|certification|networking|portfolio",
      "title": "string",
      "description": "string",       // specific, actionable
      "action_steps": ["string"],    // concrete next steps
      "estimated_weeks": number,
      "resources": [
        { "name": "string", "url": "string", "type": "course|book|docs|practice" }
      ],
      "impact": "string"             // why this matters for the target role
    }
  ],
  "suggested_projects": [
    {
      "title": "string",
      "description": "string",
      "skills_demonstrated": ["string"],
      "estimated_hours": number
    }
  ],
  "next_milestone": {
    "title": "string",
    "target_date": "string",
    "success_criteria": ["string"]
  }
}
```

---

## Prompt Variables

| Variable | Source | Required |
|----------|--------|---------|
| `current_skills` | Aggregated from `project_technologies` | ✅ |
| `career_goal` | `career_goals` table | ✅ |
| `focus` | Request body (skill_gaps, projects, networking) | ⬜ |
| `target_role_skills` | Industry knowledge (hardcoded map) | ✅ |
