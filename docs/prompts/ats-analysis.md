# Prompt: ATS Resume Analyzer

**Used by**: `crates/ats/src/service.rs` → `AtsService::analyze()`
**Provider**: Any `AIProvider` implementation
**Output format**: Structured JSON

---

## System Prompt

```
You are an expert ATS (Applicant Tracking System) consultant and technical recruiter.
Your task is to analyze a developer resume against a job description and provide detailed, actionable scoring.

Rules:
- Score objectively based on keyword match, not subjective quality.
- Identify both present AND missing keywords from the job description.
- Suggestions must be specific and actionable (not "add more skills").
- Action verb analysis must cover only the experience/projects sections.
- Readability scoring uses Flesch-Kincaid Grade Level approximation.
- Output must be valid JSON matching the schema exactly.
```

---

## User Prompt Template

```
Analyze this resume against the provided job description for ATS compatibility.

# Resume Content
{{resume_text}}

# Job Description
{{job_description}}

Output JSON:
{
  "overall_score": number,         // 0-100
  "sections": {
    "keyword_match": {
      "score": number,             // 0-100
      "matched_keywords": ["string"],
      "missing_keywords": ["string"],
      "keyword_density": number    // percentage of JD keywords found
    },
    "readability": {
      "score": number,             // 0-100
      "grade_level": number,       // Flesch-Kincaid grade
      "verdict": "string"          // excellent | good | fair | poor
    },
    "action_verbs": {
      "score": number,
      "found": ["string"],
      "weak_phrases": ["string"],  // passive/weak phrases detected
      "suggestions": ["string"]    // strong replacements
    },
    "formatting": {
      "score": number,
      "issues": ["string"],        // detected formatting problems
      "has_contact_info": boolean,
      "has_summary": boolean,
      "section_count": number
    },
    "length": {
      "score": number,
      "word_count": number,
      "verdict": "string"          // too_short | optimal | too_long
    }
  },
  "improvements": [
    {
      "priority": "high" | "medium" | "low",
      "section": "string",
      "issue": "string",
      "suggestion": "string"
    }
  ],
  "missing_sections": ["string"],  // e.g. ["skills", "summary", "links"]
  "duplicate_phrases": ["string"]
}
```

---

## Scoring Rubric

| Component | Weight |
|-----------|--------|
| Keyword match | 40% |
| Action verbs | 20% |
| Readability | 15% |
| Formatting | 15% |
| Length | 10% |

---

## Prompt Variables

| Variable | Source | Required |
|----------|--------|---------|
| `resume_text` | Resume version markdown/text | ✅ |
| `job_description` | Request body | ✅ |
