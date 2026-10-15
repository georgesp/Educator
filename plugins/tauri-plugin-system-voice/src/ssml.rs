/// Pause after each colon, in milliseconds. The system voice barely pauses there, so the core could
/// not cut « Le mot : jaune. » before the word (ADR 0004: it looks for a pause of at least 150 ms;
/// Piper pauses 230 to 530 ms). Measured on the iPad on 2026-10-15 (ADR 0005).
pub const COLON_PAUSE_MS: u32 = 400;

/// SSML read by the system voice: `text` escaped, with a pause of `pause_ms` after each colon.
pub fn ssml_with_colon_pauses(text: &str, pause_ms: u32) -> String {
    let mut ssml = String::with_capacity(text.len() + 32);
    ssml.push_str("<speak>");
    for c in text.chars() {
        match c {
            '&' => ssml.push_str("&amp;"),
            '<' => ssml.push_str("&lt;"),
            '>' => ssml.push_str("&gt;"),
            '"' => ssml.push_str("&quot;"),
            '\'' => ssml.push_str("&apos;"),
            ':' => {
                ssml.push(':');
                ssml.push_str(&format!("<break time=\"{pause_ms}ms\"/>"));
            }
            _ => ssml.push(c),
        }
    }
    ssml.push_str("</speak>");
    ssml
}

#[cfg(test)]
mod tests {
    use super::ssml_with_colon_pauses;

    #[test]
    fn a_pause_follows_the_colon_of_the_carrier() {
        assert_eq!(
            ssml_with_colon_pauses("Le mot : jaune.", 400),
            "<speak>Le mot :<break time=\"400ms\"/> jaune.</speak>"
        );
    }

    #[test]
    fn texts_without_colon_are_only_wrapped() {
        assert_eq!(
            ssml_with_colon_pauses("un stylo.", 400),
            "<speak>un stylo.</speak>"
        );
    }

    #[test]
    fn special_characters_are_escaped() {
        assert_eq!(
            ssml_with_colon_pauses("J'écris <a> & \"b\"", 300),
            "<speak>J&apos;écris &lt;a&gt; &amp; &quot;b&quot;</speak>"
        );
    }
}
