//! Unicode composition (NFC) of the letters a French child may type: a base letter followed by a
//! combining accent (grave, acute, circumflex, diaeresis, cedilla) becomes the precomposed letter
//! (`e` + U+0301 → `é`). Written by hand to keep the core free of dependencies; the other
//! characters are left as they are (they are already composed in practice: keyboards and the
//! on-screen keyboard send precomposed letters).

const GRAVE: char = '\u{0300}';
const ACUTE: char = '\u{0301}';
const CIRCUMFLEX: char = '\u{0302}';
const DIAERESIS: char = '\u{0308}';
const CEDILLA: char = '\u{0327}';

/// `(base, combining mark, precomposed)`.
const COMPOSITIONS: &[(char, char, char)] = &[
    ('a', GRAVE, 'à'),
    ('a', CIRCUMFLEX, 'â'),
    ('a', DIAERESIS, 'ä'),
    ('e', GRAVE, 'è'),
    ('e', ACUTE, 'é'),
    ('e', CIRCUMFLEX, 'ê'),
    ('e', DIAERESIS, 'ë'),
    ('i', CIRCUMFLEX, 'î'),
    ('i', DIAERESIS, 'ï'),
    ('o', CIRCUMFLEX, 'ô'),
    ('o', DIAERESIS, 'ö'),
    ('u', GRAVE, 'ù'),
    ('u', CIRCUMFLEX, 'û'),
    ('u', DIAERESIS, 'ü'),
    ('y', DIAERESIS, 'ÿ'),
    ('c', CEDILLA, 'ç'),
    ('A', GRAVE, 'À'),
    ('A', CIRCUMFLEX, 'Â'),
    ('A', DIAERESIS, 'Ä'),
    ('E', GRAVE, 'È'),
    ('E', ACUTE, 'É'),
    ('E', CIRCUMFLEX, 'Ê'),
    ('E', DIAERESIS, 'Ë'),
    ('I', CIRCUMFLEX, 'Î'),
    ('I', DIAERESIS, 'Ï'),
    ('O', CIRCUMFLEX, 'Ô'),
    ('O', DIAERESIS, 'Ö'),
    ('U', GRAVE, 'Ù'),
    ('U', CIRCUMFLEX, 'Û'),
    ('U', DIAERESIS, 'Ü'),
    ('Y', DIAERESIS, 'Ÿ'),
    ('C', CEDILLA, 'Ç'),
];

/// The text with the French letters composed.
pub fn nfc(text: &str) -> String {
    let mut composed = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        let combined = chars.peek().and_then(|&mark| {
            COMPOSITIONS
                .iter()
                .find(|(base, m, _)| *base == c && *m == mark)
                .map(|(_, _, letter)| *letter)
        });
        match combined {
            Some(letter) => {
                composed.push(letter);
                chars.next();
            }
            None => composed.push(c),
        }
    }

    composed
}

#[cfg(test)]
mod tests {
    use super::nfc;

    #[test]
    fn combining_accents_are_composed() {
        assert_eq!("été", nfc("e\u{301}te\u{301}"));
        assert_eq!("garçon", nfc("garc\u{327}on"));
        assert_eq!("Noël", nfc("Noe\u{308}l"));
        assert_eq!("à côté", nfc("a\u{300} co\u{302}te\u{301}"));
    }

    #[test]
    fn composed_and_plain_text_is_unchanged() {
        assert_eq!("bateau été œuf", nfc("bateau été œuf"));
        // A mark on a letter without composition stays as it is.
        assert_eq!("x\u{301}", nfc("x\u{301}"));
    }
}
