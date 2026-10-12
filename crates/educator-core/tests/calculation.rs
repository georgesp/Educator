use educator_core::exercises::{Calculation, CalculationExercise, HiddenPart};

#[test]
fn addition_carry() {
    for (left, right, carry) in [
        (3, 4, false),
        (7, 5, true),
        (23, 14, false),
        (28, 14, true),
        (150, 60, true),
        (245, 437, true),
        (40, 9, false),
    ] {
        assert_eq!(
            carry,
            Calculation::addition(left, right).has_carry(),
            "{left} + {right}"
        );
    }
}

#[test]
fn subtraction_borrow() {
    for (left, right, borrow) in [
        (9, 4, false),
        (12, 5, true),
        (57, 23, false),
        (50, 6, true),
        (523, 7, true),
        (157, 5, false),
        (746, 80, true),
        (100, 1, true),
    ] {
        assert_eq!(
            borrow,
            Calculation::subtraction(left, right).has_carry(),
            "{left} − {right}"
        );
    }
}

#[test]
fn duplicate_key_commuted_additions_are_the_same() {
    assert_eq!(
        Calculation::addition(7, 5).duplicate_key(),
        Calculation::addition(5, 7).duplicate_key()
    );
    assert_ne!(
        Calculation::subtraction(7, 5).duplicate_key(),
        Calculation::addition(7, 5).duplicate_key()
    );
}

#[test]
fn display_is_the_complete_equality() {
    assert_eq!("7 + 5 = 12", Calculation::addition(7, 5).to_string());
    assert_eq!("12 − 5 = 7", Calculation::subtraction(12, 5).to_string());
}

#[test]
fn exercise_hides_the_asked_number() {
    let calculation = Calculation::addition(7, 5);
    let result = CalculationExercise::new(0, calculation, HiddenPart::Result);
    let missing_right = CalculationExercise::new(1, calculation, HiddenPart::Right);
    let missing_left = CalculationExercise::new(2, calculation, HiddenPart::Left);

    assert_eq!("7 + 5 = ?", result.prompt());
    assert_eq!(12, result.expected());
    assert_eq!("7 + ? = 12", missing_right.prompt());
    assert_eq!(5, missing_right.expected());
    assert_eq!("? + 5 = 12", missing_left.prompt());
    assert_eq!(7, missing_left.expected());
    assert_eq!("7 + 5 = 12", missing_left.explanation());
    assert_eq!("Combien font 7 plus 5 ?", result.spoken_prompt());
    assert_eq!(
        "Combien faut-il ajouter à 7 pour obtenir 12 ?",
        missing_right.spoken_prompt()
    );
}
