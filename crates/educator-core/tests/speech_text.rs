//! Same cases as FrenchSpeechTextTests (C#): the output must be identical.
use educator_core::speech::{normalize, number_to_words};

#[test]
fn number_to_words_cases() {
    let cases = [
        (0, "zéro"),
        (1, "un"),
        (16, "seize"),
        (17, "dix-sept"),
        (21, "vingt et un"),
        (22, "vingt-deux"),
        (30, "trente"),
        (61, "soixante et un"),
        (70, "soixante-dix"),
        (71, "soixante et onze"),
        (77, "soixante-dix-sept"),
        (80, "quatre-vingts"),
        (81, "quatre-vingt-un"),
        (90, "quatre-vingt-dix"),
        (91, "quatre-vingt-onze"),
        (99, "quatre-vingt-dix-neuf"),
        (100, "cent"),
        (101, "cent un"),
        (180, "cent quatre-vingts"),
        (200, "deux cents"),
        (201, "deux cent un"),
        (371, "trois cent soixante et onze"),
        (999, "neuf cent quatre-vingt-dix-neuf"),
        (1000, "mille"),
        (1001, "mille un"),
        (2000, "deux mille"),
        (80000, "quatre-vingt mille"),
        (200300, "deux cent mille trois cents"),
        (1_000_000, "un million"),
    ];
    for (number, words) in cases {
        assert_eq!(words, number_to_words(number), "{number}");
    }
}

#[test]
fn normalize_cases() {
    let cases = [
        ("7 + 5", "sept plus cinq"),
        ("Jusqu'à 1 000.", "Jusqu'à mille."),
        (
            "12\u{a0}500 et 3\u{202f}000",
            "douze mille cinq cents et trois mille",
        ),
        ("De 300 à 400", "De trois cents à quatre cents"),
        ("7 + 5 = 12", "sept plus cinq égale douze"),
        ("12 − 5 = 7", "douze moins cinq égale sept"),
        ("12 - 5", "douze moins cinq"),
        ("7 + ? = 12", "sept plus combien égale douze"),
        ("? − 5 = 7", "combien moins cinq égale sept"),
        ("Combien font 7 plus 5 ?", "Combien font sept plus cinq ?"),
        (
            "Bravo ! Tu as 9 bonnes réponses sur 10.",
            "Bravo ! Tu as neuf bonnes réponses sur dix.",
        ),
        (
            "Peut-être 245 + 437",
            "Peut-être deux cent quarante-cinq plus quatre cent trente-sept",
        ),
    ];
    for (text, spoken) in cases {
        assert_eq!(spoken, normalize(text), "{text}");
    }
}
