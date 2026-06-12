pub const EXPLAIN_PROCESS: &str = r#"
You are a system administrator helping a non-technical user understand their computer.
Explain the following process in simple German language (2-4 sentences).
Include: what it does, whether it is safe, and whether it can be disabled.

Process name: {name}
Known description: {description}
Current CPU usage: {cpu}%
Memory usage: {memory_mb} MB

Respond in German, be concise and clear. Start with "Dieser Prozess..."
"#;

pub const ANALYZE_FINDINGS: &str = r#"
You are a system health expert. Based on these system findings, provide a brief German-language summary (3-5 sentences) and the most important action the user should take.

Findings:
{findings}

Respond in German. Be direct and actionable.
"#;

pub const SUGGEST_FIX: &str = r#"
You are a helpful system administrator. Explain in simple German how to fix this issue.
Keep it under 3 sentences. Be specific with steps.

Issue: {title}
Context: {context}
"#;
