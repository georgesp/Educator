//! Numbers written in French words (SUB000121): the traditional spelling shown to the child, the
//! spelling of the 1990 rectification (never a wrong answer), and the words of the simple fractions
//! (SUB000123).

use crate::speech::number_to_words;

/// Largest number spelled by [`spell`] (CE1 works up to 1 000, the later grades go further).
pub const MAX_SPELLED: i32 = 999_999;

/// How a number is written in words.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Spelling {
    /// Traditional spelling, shown to the child: « deux cent quatre-vingts », « vingt et un ».
    Traditional,
    /// Rectification of 1990: hyphens between every numeral word, « et » included:
    /// « deux-cent-quatre-vingts », « vingt-et-un ».
    Rectified1990,
}

/// French words of `number` (0 to [`MAX_SPELLED`]).
pub fn spell(number: i32, spelling: Spelling) -> String {
    assert!(
        (0..=MAX_SPELLED).contains(&number),
        "number out of the spelled range"
    );
    let traditional = number_to_words(number.unsigned_abs());
    match spelling {
        Spelling::Traditional => traditional,
        // « mille » is a numeral too: every space becomes a hyphen below a million.
        Spelling::Rectified1990 => traditional.replace(' ', "-"),
    }
}

/// Lower case, single spaces, no space around hyphens, typographic apostrophes straightened.
pub fn normalize_words(text: &str) -> String {
    let lower = text.trim().to_lowercase().replace('’', "'");
    let collapsed = lower.split_whitespace().collect::<Vec<_>>().join(" ");
    collapsed.replace(" -", "-").replace("- ", "-")
}

/// Whether `text` writes `number` correctly, in the traditional spelling or the 1990 one.
pub fn is_correct_spelling(text: &str, number: i32) -> bool {
    if !(0..=MAX_SPELLED).contains(&number) {
        return false;
    }

    let given = normalize_words(text);
    given == spell(number, Spelling::Traditional) || given == spell(number, Spelling::Rectified1990)
}

/// Credible misspellings of `number` (SUB000121): « cents » or « vingts » wrongly agreed or wrongly
/// left singular, a missing or extra « et », hyphens and spaces swapped. None of them is a correct
/// spelling (traditional or 1990). Deterministic order, without duplicates.
pub fn misspellings(number: i32) -> Vec<String> {
    let right = spell(number, Spelling::Traditional);
    let mut candidates: Vec<String> = Vec::new();

    // Agreement of « cent » and « vingt ».
    if right.contains("cents") {
        candidates.push(right.replace("cents", "cent"));
    }

    if right.contains("vingts") {
        candidates.push(right.replace("vingts", "vingt"));
    }

    if right.contains("cent ") && !right.contains("cents") {
        let hundreds = number / 100 % 10;
        if hundreds > 1 {
            candidates.push(right.replacen("cent ", "cents ", 1));
        }
    }

    if right.ends_with("quatre-vingt") || right.contains("quatre-vingt-") {
        candidates.push(right.replacen("quatre-vingt", "quatre-vingts", 1));
    }

    if right.contains("cents") && right.contains("vingt") && !right.contains("vingts") {
        candidates.push(right.replace("cents", "cent").replace("vingt", "vingts"));
    }

    // « et » : 21, 31… 71 take it, 81 and 91 do not.
    if right.contains(" et ") {
        candidates.push(right.replace(" et ", "-"));
    } else if right.contains("quatre-vingt-un") {
        candidates.push(right.replace("quatre-vingt-un", "quatre-vingt et un"));
    } else if right.contains("quatre-vingt-onze") {
        candidates.push(right.replace("quatre-vingt-onze", "quatre-vingt et onze"));
    }

    // Hyphens and spaces swapped in the tens.
    if let Some(position) = right.rfind('-') {
        let mut swapped = right.clone();
        swapped.replace_range(position..=position, " ");
        candidates.push(swapped);
    }

    // Both agreement errors at once (« deux cents quatre-vingt »).
    if right.contains("cent quatre-vingts") && number / 100 % 10 > 1 {
        candidates.push(right.replace("cent quatre-vingts", "cents quatre-vingt"));
    }

    let mut result: Vec<String> = Vec::new();
    for candidate in candidates {
        if candidate != right
            && !is_correct_spelling(&candidate, number)
            && !result.contains(&candidate)
        {
            result.push(candidate);
        }
    }

    result
}

/// Words of a fraction: « un demi », « deux tiers », « trois quarts », « cinq huitièmes ».
pub fn fraction_words(numerator: u32, denominator: u32) -> String {
    let count = number_to_words(numerator);
    let plural = numerator > 1;
    let name = match denominator {
        2 => (if plural { "demis" } else { "demi" }).to_owned(),
        3 => "tiers".to_owned(),
        4 => (if plural { "quarts" } else { "quart" }).to_owned(),
        _ => {
            let base = number_to_words(denominator);
            // cinq → cinquième, neuf → neuvième, onze → onzième, quatre → quatrième.
            let stem = if let Some(stripped) = base.strip_suffix('e') {
                stripped.to_owned()
            } else if let Some(stripped) = base.strip_suffix('f') {
                format!("{stripped}v")
            } else if base == "cinq" {
                "cinqu".to_owned()
            } else {
                base
            };
            format!("{stem}ième{}", if plural { "s" } else { "" })
        }
    };
    format!("{count} {name}")
}
