use crate::models::Lang;

/// One prompt per language: the answer should be in the language of the interface.
pub fn explain_process(lang: Lang, name: &str, description: &str, cpu: f32, memory_mb: f64) -> String {
    let (language, start) = match lang {
        Lang::En => ("English", "This process"),
        Lang::De => ("German (Swiss spelling, no ß)", "Dieser Prozess"),
    };
    format!(
        "You are a system administrator helping a non-technical user understand their computer.\n\
         Explain the following process in simple {language} (2 to 4 sentences): what it does, \
         whether it is safe, and whether it can be quit or removed.\n\n\
         Process name: {name}\n\
         Known description: {description}\n\
         Current CPU usage: {cpu:.1}%\n\
         Memory usage: {memory_mb:.0} MB\n\n\
         Answer in {language} only. Start with \"{start}\"."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_prompt_asks_for_the_interface_language() {
        assert!(explain_process(Lang::En, "x", "", 0.0, 0.0).contains("simple English"));
        assert!(explain_process(Lang::De, "x", "", 0.0, 0.0).contains("Dieser Prozess"));
    }
}
