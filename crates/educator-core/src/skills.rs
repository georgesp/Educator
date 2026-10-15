//! Skills: one notion worked on at a grade and a school period, described by generation rules
//! (`content/skills/math/*.json`, SUB000106). Exercises are never stored: the engine generates them
//! from these rules ([`crate::exercises::generate_series`]).

use crate::exercises::ProblemSet;
use crate::spelling::WordSet;
use crate::Grade;

pub const FIRST_PERIOD: u8 = 1;
pub const LAST_PERIOD: u8 = 5;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Skill {
    /// Stable identifier, e.g. `cp-add-sum-10` (also the file name).
    pub id: String,
    pub subject: Subject,
    pub grade: Grade,
    /// School period, 1 (rentrée → Toussaint) to 5 (printemps → été).
    pub period: u8,
    /// Short title shown to the child, in French.
    pub title: String,
    /// Instruction read and shown before the first exercise, in French.
    pub instruction: String,
    pub operation: SkillOperation,
    pub left: OperandRule,
    pub right: OperandRule,
    pub constraints: SkillConstraints,
    pub formats: Vec<ExerciseFormat>,
    pub answer_modes: Vec<AnswerMode>,
    /// Reference of the official programme justifying the skill.
    pub source: String,
    /// Other grades and periods where the skill is also offered (SUB000113), besides the default
    /// revision rule ([`SkillCatalog::offered_to`]).
    pub also_offered: Vec<Placement>,
    /// Rules of the families other than addition and subtraction (CE1 complements, SUB000118 to
    /// SUB000124): numbers, facts, fractions, problems, progression.
    pub params: SkillParams,
}

/// Generation rules beyond the two operands (empty for additions and subtractions).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SkillParams {
    /// Numbers worked on (numeration): 100 to 999, multiples of 2…
    pub numbers: Option<OperandRule>,
    /// Multiplication: the tables worked on (« tables de 3 et 4 »); the other values of the rule
    /// are revisions mixed in.
    pub focus: Vec<i32>,
    pub progression: Progression,
    /// Picture shown with each exercise.
    pub picture: Option<SkillPicture>,
    /// Doubles to know (SUB000122).
    pub doubles: Option<OperandRule>,
    /// Numbers whose half is asked.
    pub halves: Option<OperandRule>,
    /// Multiples of 25: « 3 fois 25 ».
    pub times_twenty_five: Vec<i32>,
    /// Fractions: allowed denominators (SUB000123).
    pub denominators: Vec<u32>,
    /// Fractions: only 1/b.
    pub unit_fractions_only: bool,
    /// Problems: the templates and words resolved at loading (SUB000124).
    pub problems: Option<ProblemSet>,
    /// Spelling: the words of the bank taken by the skill, resolved at loading (SUB001402).
    pub words: Option<WordSet>,
    /// Spelling: the theme shown to the child (required for a spelling skill, absent in math).
    pub theme: Option<SpellingTheme>,
}

/// Order of the exercises in a series.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Progression {
    /// Any order.
    #[default]
    Mixed,
    /// The easiest exercises first (× 2, × 5, × 10, small factors; halves of even tens…).
    EasyFirst,
}

/// Picture shown with each exercise of a skill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkillPicture {
    /// Rows of dots: 3 × 5 drawn as 3 rows of 5.
    ArrayGrid,
}

/// A grade and a period where a skill is offered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Placement {
    pub grade: Grade,
    pub period: u8,
}

impl Placement {
    pub const fn new(grade: Grade, period: u8) -> Self {
        Self { grade, period }
    }
}

/// Period of grade N whose skills are offered again, as revisions, to grade N + 1 (SUB000113).
pub const REVISED_PERIOD: u8 = LAST_PERIOD;

/// Period of grade N + 1 where the revisions of grade N are offered.
pub const REVISION_PERIOD: u8 = FIRST_PERIOD;

/// A skill as offered to a grade: its own skills, and the skills of another grade offered as revisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OfferedSkill<'a> {
    pub skill: &'a Skill,
    /// Period of the grade where it is offered (the skill's own period when it is not a revision).
    pub period: u8,
    /// Grade the skill comes from, when it is offered as a revision.
    pub revision_from: Option<Grade>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Subject {
    Math,
    /// Spelling (lot 2): dictation, missing letters, the right spelling.
    Spelling,
}

impl Subject {
    pub const ALL: [Subject; 2] = [Subject::Math, Subject::Spelling];

    /// Name in the content files (`"math"`) and in the database.
    pub fn as_str(self) -> &'static str {
        match self {
            Subject::Math => "math",
            Subject::Spelling => "spelling",
        }
    }

    pub fn from_name(name: &str) -> Option<Subject> {
        Subject::ALL.into_iter().find(|s| s.as_str() == name)
    }
}

/// Operation (family) of a skill. `Mixed`: each exercise is an addition or a subtraction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkillOperation {
    Addition,
    Subtraction,
    Mixed,
    Multiplication,
    /// Numbers: place value, reading and writing, comparing, ordering, number line, parity.
    Numeration,
    /// Doubles and halves (facts, half of an even number).
    DoubleHalf,
    Fraction,
    /// Word problems generated from templates.
    Problem,
    /// Spelling: words of the bank (dictation, missing letters, the right spelling).
    Spelling,
}

impl SkillOperation {
    /// Families generated from two operand rules (`left op right`).
    pub fn is_calculation(&self) -> bool {
        matches!(
            self,
            SkillOperation::Addition
                | SkillOperation::Subtraction
                | SkillOperation::Mixed
                | SkillOperation::Multiplication
        )
    }
}

/// Theme of a spelling skill, shown to the child with a color and a symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpellingTheme {
    /// Letters and the sounds they write: vowels, consonants, [ou], [an], [é]…
    LettersAndSounds,
    /// Small rules of writing: m before m, b, p; g, gu, ge.
    SpellingRules,
    SilentLetters,
    /// Agreement: plural of nouns (later: feminine, verb agreement).
    Agreement,
    /// Words learnt by heart: small function words, invariable words.
    WordsToKnow,
}

/// Family of a skill as shown to the child: a color and a symbol on every card of the exercises
/// and the lessons. Math families come from the operation, spelling ones from the theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SkillCategory {
    /// Numbers: place value, reading, comparing, number line, parity.
    Numbers,
    /// Additions and subtractions.
    AddSubtract,
    Multiply,
    /// Fractions, doubles and halves.
    FractionsHalves,
    Problems,
    LettersAndSounds,
    SpellingRules,
    SilentLetters,
    Agreement,
    WordsToKnow,
}

impl SkillCategory {
    /// Every category, in the order shown to the child (math, then spelling).
    pub const ALL: [SkillCategory; 10] = [
        SkillCategory::Numbers,
        SkillCategory::AddSubtract,
        SkillCategory::Multiply,
        SkillCategory::FractionsHalves,
        SkillCategory::Problems,
        SkillCategory::LettersAndSounds,
        SkillCategory::SpellingRules,
        SkillCategory::SilentLetters,
        SkillCategory::Agreement,
        SkillCategory::WordsToKnow,
    ];
}

impl From<SpellingTheme> for SkillCategory {
    fn from(theme: SpellingTheme) -> Self {
        match theme {
            SpellingTheme::LettersAndSounds => SkillCategory::LettersAndSounds,
            SpellingTheme::SpellingRules => SkillCategory::SpellingRules,
            SpellingTheme::SilentLetters => SkillCategory::SilentLetters,
            SpellingTheme::Agreement => SkillCategory::Agreement,
            SpellingTheme::WordsToKnow => SkillCategory::WordsToKnow,
        }
    }
}

impl Skill {
    /// Family shown to the child. A spelling skill without theme (only built in tests: the
    /// content requires one) counts as letters and sounds.
    pub fn category(&self) -> SkillCategory {
        match self.operation {
            SkillOperation::Numeration => SkillCategory::Numbers,
            SkillOperation::Addition | SkillOperation::Subtraction | SkillOperation::Mixed => {
                SkillCategory::AddSubtract
            }
            SkillOperation::Multiplication => SkillCategory::Multiply,
            SkillOperation::Fraction | SkillOperation::DoubleHalf => SkillCategory::FractionsHalves,
            SkillOperation::Problem => SkillCategory::Problems,
            SkillOperation::Spelling => self
                .params
                .theme
                .map_or(SkillCategory::LettersAndSounds, Into::into),
        }
    }
}

/// Allowed values of one operand: the integers of `[min, max]`, optionally only the multiples of
/// `multiple_of`, or exactly the listed `values`. For a subtraction, the left operand is the number
/// subtracted from (minuend).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OperandRule {
    pub min: i32,
    pub max: i32,
    pub multiple_of: Option<i32>,
    pub values: Option<Vec<i32>>,
}

impl OperandRule {
    pub fn range(min: i32, max: i32) -> Self {
        Self {
            min,
            max,
            multiple_of: None,
            values: None,
        }
    }

    pub fn multiples(min: i32, max: i32, multiple_of: i32) -> Self {
        Self {
            multiple_of: Some(multiple_of),
            ..Self::range(min, max)
        }
    }

    /// Every allowed value, in increasing order.
    pub fn allowed_values(&self) -> Vec<i32> {
        if let Some(values) = self.values.as_ref().filter(|v| !v.is_empty()) {
            let mut sorted = values.clone();
            sorted.sort_unstable();
            sorted.dedup();
            return sorted;
        }

        let step = self.multiple_of.filter(|m| *m > 0).unwrap_or(1);
        let first = if self.min % step == 0 {
            self.min
        } else {
            self.min + (step - self.min % step)
        };
        (first..=self.max).step_by(step as usize).collect()
    }

    /// Number of allowed values, without listing them (to refuse a rule that is too wide).
    pub fn allowed_count(&self) -> i64 {
        if let Some(values) = self.values.as_ref().filter(|v| !v.is_empty()) {
            let mut sorted = values.clone();
            sorted.sort_unstable();
            sorted.dedup();
            return sorted.len() as i64;
        }

        let step = i64::from(self.multiple_of.filter(|m| *m > 0).unwrap_or(1));
        let (min, max) = (i64::from(self.min), i64::from(self.max));
        let first = if min % step == 0 {
            min
        } else {
            min + (step - min % step)
        };
        if first > max {
            0
        } else {
            (max - first) / step + 1
        }
    }

    pub fn allows(&self, value: i32) -> bool {
        if let Some(values) = self.values.as_ref().filter(|v| !v.is_empty()) {
            return values.contains(&value);
        }

        value >= self.min
            && value <= self.max
            && self
                .multiple_of
                .filter(|m| *m > 0)
                .is_none_or(|m| value % m == 0)
    }
}

/// Constraints on the result and the kind of calculation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SkillConstraints {
    pub min_result: Option<i32>,
    pub max_result: Option<i32>,
    /// Result imposed (e.g. 10 for the complements to 10).
    pub exact_result: Option<i32>,
    /// The result must be a multiple of this number (e.g. 10: complement to the next ten).
    pub result_multiple_of: Option<i32>,
    pub carry: CarryRule,
    /// Trivial calculations never generated.
    pub exclude_trivial: Vec<TrivialCase>,
}

/// Carry (addition) or borrow (subtraction): « passage de la dizaine », « retenue ».
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CarryRule {
    #[default]
    Any,
    Forbidden,
    Required,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrivialCase {
    /// An addition with 0 (`7 + 0`).
    PlusZero,
    /// A subtraction of 0 (`7 − 0`).
    MinusZero,
    /// A number minus itself (`7 − 7`).
    MinusSelf,
}

/// How the exercise is asked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExerciseFormat {
    /// The result is asked: `7 + 5 = ?`.
    Result,
    /// An operand is asked: `7 + ? = 12` (multiplication: also `30 = 3 × ?`).
    MissingOperand,
    /// A sentence: « Quel est le double de 35 ? »; for a multiplication, the repeated addition
    /// `5 + 5 + 5 = ? × 5`.
    Phrase,
    /// Hundreds, tens and units → the number.
    Recompose,
    /// The number → `347 = 300 + ? + 7`.
    Decompose,
    /// Digit or number of tens: « Combien de dizaines dans 347 ? ».
    PlaceValue,
    /// A number in words → in digits.
    WriteNumber,
    /// A number in digits → pick its words.
    ReadNumber,
    /// `438 ? 483` → `<`, `>` or `=`.
    Compare,
    /// Put 3 to 5 numbers in order.
    Order,
    /// `… < 347 < …` between two tens or two hundreds.
    Frame,
    /// Read or place a number on a graduated line.
    NumberLinePoint,
    /// A yes / no question (« 573 est-il pair ? »).
    YesNo,
    /// Pick every right number of a list.
    MultiChoice,
    /// A colored bar or disc → the fraction.
    FractionPicture,
    /// « trois quarts » ↔ 3/4.
    FractionWords,
    /// A word problem.
    Problem,
    /// Spelling: the word is read, the child writes it (SUB001406).
    Dictation,
    /// Spelling: the word with its worked letters hidden, typed or picked (SUB001407).
    MissingLetters,
    /// Spelling: the word is read, the child picks the right spelling among 3 (SUB001408).
    ChooseSpelling,
}

impl ExerciseFormat {
    /// Every format with its name in the content files and the database (`"missingOperand"`).
    pub const NAMES: [(&'static str, ExerciseFormat); 20] = [
        ("result", ExerciseFormat::Result),
        ("missingOperand", ExerciseFormat::MissingOperand),
        ("phrase", ExerciseFormat::Phrase),
        ("recompose", ExerciseFormat::Recompose),
        ("decompose", ExerciseFormat::Decompose),
        ("placeValue", ExerciseFormat::PlaceValue),
        ("writeNumber", ExerciseFormat::WriteNumber),
        ("readNumber", ExerciseFormat::ReadNumber),
        ("compare", ExerciseFormat::Compare),
        ("order", ExerciseFormat::Order),
        ("frame", ExerciseFormat::Frame),
        ("numberLinePoint", ExerciseFormat::NumberLinePoint),
        ("yesNo", ExerciseFormat::YesNo),
        ("multiChoice", ExerciseFormat::MultiChoice),
        ("fractionPicture", ExerciseFormat::FractionPicture),
        ("fractionWords", ExerciseFormat::FractionWords),
        ("problem", ExerciseFormat::Problem),
        ("dictation", ExerciseFormat::Dictation),
        ("missingLetters", ExerciseFormat::MissingLetters),
        ("chooseSpelling", ExerciseFormat::ChooseSpelling),
    ];

    pub fn name(self) -> &'static str {
        Self::NAMES
            .iter()
            .find(|(_, f)| *f == self)
            .map(|(n, _)| *n)
            .unwrap_or("result")
    }

    pub fn from_name(name: &str) -> Option<ExerciseFormat> {
        Self::NAMES
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, f)| *f)
    }

    /// The formats that cannot be played without the voice: the word is only heard (SUB026704).
    /// The missing letters stay playable: the word is partly written.
    pub fn needs_listening(self) -> bool {
        matches!(
            self,
            ExerciseFormat::Dictation | ExerciseFormat::ChooseSpelling
        )
    }

    /// The spelling formats.
    pub fn is_spelling(self) -> bool {
        matches!(
            self,
            ExerciseFormat::Dictation
                | ExerciseFormat::MissingLetters
                | ExerciseFormat::ChooseSpelling
        )
    }
}

/// How the child answers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnswerMode {
    /// Types the number (on-screen keypad or keyboard).
    Keyboard,
    /// Picks one of 4 proposals.
    Choice,
}

impl AnswerMode {
    /// Stored name (the name of the former API).
    pub fn as_str(self) -> &'static str {
        match self {
            AnswerMode::Keyboard => "Keyboard",
            AnswerMode::Choice => "Choice",
        }
    }

    pub fn from_name(name: &str) -> Option<AnswerMode> {
        [AnswerMode::Keyboard, AnswerMode::Choice]
            .into_iter()
            .find(|m| m.as_str() == name)
    }
}

/// The skills loaded from the content (read-only). Implemented in `educator-content`.
pub trait SkillCatalog: Send + Sync {
    fn all(&self) -> &[Skill];

    fn find(&self, id: &str) -> Option<&Skill> {
        self.all().iter().find(|s| s.id == id)
    }

    /// Skills of a subject and a grade, sorted by period then id.
    fn for_grade(&self, subject: Subject, grade: Grade) -> Vec<&Skill> {
        let mut skills: Vec<&Skill> = self
            .all()
            .iter()
            .filter(|s| s.subject == subject && s.grade == grade)
            .collect();
        skills.sort_by(|a, b| a.period.cmp(&b.period).then_with(|| a.id.cmp(&b.id)));
        skills
    }

    /// Skills of a grade; when it has none yet, those of [`FALLBACK_GRADE`]. The returned grade is
    /// the grade whose skills are returned.
    fn for_or_fallback(&self, subject: Subject, grade: Grade) -> (Grade, Vec<&Skill>) {
        let skills = self.for_grade(subject, grade);
        if !skills.is_empty() || grade == FALLBACK_GRADE {
            (grade, skills)
        } else {
            (FALLBACK_GRADE, self.for_grade(subject, FALLBACK_GRADE))
        }
    }

    /// Everything offered to a grade (SUB000113): its own skills, plus as revisions the skills of
    /// period [`REVISED_PERIOD`] of the grade before (offered in period [`REVISION_PERIOD`]) and
    /// the skills declaring this grade in `also_offered`. A skill appears once (its own grade first).
    /// Sorted by period, the grade's own skills before the revisions, then by id.
    fn offered_to(&self, subject: Subject, grade: Grade) -> Vec<OfferedSkill<'_>> {
        let mut offered: Vec<OfferedSkill<'_>> = self
            .for_grade(subject, grade)
            .into_iter()
            .map(|skill| OfferedSkill {
                skill,
                period: skill.period,
                revision_from: None,
            })
            .collect();

        let previous = grade.previous();
        let mut revisions: Vec<OfferedSkill<'_>> = Vec::new();
        for skill in self
            .all()
            .iter()
            .filter(|s| s.subject == subject && s.grade != grade)
        {
            let by_default = (Some(skill.grade) == previous && skill.period == REVISED_PERIOD)
                .then_some(REVISION_PERIOD);
            let declared = skill
                .also_offered
                .iter()
                .find(|p| p.grade == grade)
                .map(|p| p.period);
            if let Some(period) = declared.or(by_default) {
                revisions.push(OfferedSkill {
                    skill,
                    period,
                    revision_from: Some(skill.grade),
                });
            }
        }

        offered.extend(revisions);
        offered.sort_by(|a, b| {
            a.period
                .cmp(&b.period)
                .then_with(|| a.revision_from.is_some().cmp(&b.revision_from.is_some()))
                .then_with(|| a.revision_from.cmp(&b.revision_from))
                .then_with(|| a.skill.id.cmp(&b.skill.id))
        });
        offered
    }

    /// [`SkillCatalog::offered_to`] a grade; when the grade has no skill of its own yet, what is
    /// offered to [`FALLBACK_GRADE`] (its skills and its revisions), like [`SkillCatalog::for_or_fallback`].
    fn offered_or_fallback(
        &self,
        subject: Subject,
        grade: Grade,
    ) -> (Grade, Vec<OfferedSkill<'_>>) {
        let (shown, _) = self.for_or_fallback(subject, grade);
        (shown, self.offered_to(subject, shown))
    }
}

/// Grade offered to the grades that have no skill yet (CE2 to CM2 in lot 1, SUB000108).
pub const FALLBACK_GRADE: Grade = Grade::CE1;

/// [`SkillCatalog`] over a list of skills.
#[derive(Debug, Clone, Default)]
pub struct SkillList(pub Vec<Skill>);

impl SkillCatalog for SkillList {
    fn all(&self) -> &[Skill] {
        &self.0
    }
}

/// First day of a period (month and day, every year).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PeriodStart {
    pub period: u8,
    pub month: u32,
    pub day: u32,
}

impl PeriodStart {
    pub const fn new(period: u8, month: u32, day: u32) -> Self {
        Self { period, month, day }
    }
}

/// A calendar day (local date of the computer).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl Date {
    pub const fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }
}

/// Splits the school year into 5 periods (rentrée → Toussaint → Noël → hiver → printemps → été).
/// The start dates are content (`content/calendar/periods.json`), chosen after the holidays of every
/// zone so that the result does not depend on the holiday zone. July and August belong to period 5.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchoolCalendar {
    starts: Vec<PeriodStart>,
}

impl SchoolCalendar {
    /// One start per period 1 to 5, in school-year order (period 1 starts the year).
    pub fn new(starts: Vec<PeriodStart>) -> Result<Self, String> {
        if starts.len() != usize::from(LAST_PERIOD)
            || starts
                .iter()
                .enumerate()
                .any(|(i, s)| usize::from(s.period) != i + 1)
        {
            return Err("Exactly one start per period 1 to 5, in order.".to_owned());
        }

        if starts
            .iter()
            .any(|s| !(1..=12).contains(&s.month) || s.day < 1 || s.day > days_in_month(s.month))
        {
            return Err("Period starts must be valid days of the year.".to_owned());
        }

        let offsets: Vec<u32> = starts
            .iter()
            .map(|s| offset_in_school_year(s.month, s.day, &starts[0]))
            .collect();
        if offsets.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err("Period starts must follow each other within the school year.".to_owned());
        }

        Ok(Self { starts })
    }

    /// Default dates: 1 Sept, 4 Nov, 5 Jan, 9 March, 4 May.
    pub fn default_calendar() -> Self {
        Self {
            starts: vec![
                PeriodStart::new(1, 9, 1),
                PeriodStart::new(2, 11, 4),
                PeriodStart::new(3, 1, 5),
                PeriodStart::new(4, 3, 9),
                PeriodStart::new(5, 5, 4),
            ],
        }
    }

    pub fn starts(&self) -> &[PeriodStart] {
        &self.starts
    }

    pub fn period_of(&self, date: Date) -> u8 {
        let offset = offset_in_school_year(date.month, date.day, &self.starts[0]);
        let mut period = 1;
        for start in &self.starts {
            if offset >= offset_in_school_year(start.month, start.day, &self.starts[0]) {
                period = start.period;
            }
        }

        period
    }
}

impl Default for SchoolCalendar {
    fn default() -> Self {
        Self::default_calendar()
    }
}

const DAYS_BEFORE_MONTH: [u32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

fn days_in_month(month: u32) -> u32 {
    match month {
        2 => 29,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// Days since the start of period 1, in a non-leap reference year (29 Feb counts as 28 Feb).
fn offset_in_school_year(month: u32, day: u32, year_start: &PeriodStart) -> u32 {
    let day_of_year = |m: u32, d: u32| {
        DAYS_BEFORE_MONTH[(m - 1) as usize] + if m == 2 && d == 29 { 28 } else { d }
    };
    (day_of_year(month, day) + 365 - day_of_year(year_start.month, year_start.day)) % 365
}
