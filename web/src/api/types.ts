// Generated from web/src-tauri/src/contracts.rs by `npm run api:generate` (cargo test). Do not edit.

export type Grade = "CP" | "CE1" | "CE2" | "CM1" | "CM2";

export type AnswerMode = "Keyboard" | "Choice";

export type Subject = "Math" | "Spelling";

export type SpellingFormat = "Dictation" | "MissingLetters" | "ChooseSpelling";

export type SessionType = "Exercise" | "Exam" | "Timed" | "LessonQuiz" | "Placement" | "Review";

export type CharacterKind = "Animal" | "Child";

export type StartupStatus = { "state": "starting" } | { "state": "ready" } | { "state": "failed", message: string, };

export type StatusResponse = { version: string, database: DatabaseStatusResponse, speech: SpeechStatusResponse, };

export type DatabaseStatusResponse = { 
/**
 * True once the database is created and migrated.
 */
isReady: boolean, };

export type SpeechStatusResponse = { isAvailable: boolean, 
/**
 * Voice name, when available (e.g. `fr_FR-siwis-medium`).
 */
voice: string | null, 
/**
 * Why the voice is unavailable (English, diagnostics only).
 */
problem: string | null, };

export type SkillsResponse = { subject: Subject, 
/**
 * Grade asked for.
 */
grade: Grade, 
/**
 * Grade of the returned skills: the asked one, or CE1 when the asked grade has no skill yet.
 */
skillsGrade: Grade, 
/**
 * School period of today (1 to 5), from `content/calendar/periods.json`.
 */
currentPeriod: number, skills: Array<SkillResponse>, };

export type SkillResponse = { id: string, title: string, instruction: string, 
/**
 * Period where the skill is offered to this grade.
 */
period: number, 
/**
 * True for the skills of the current period (revisions excepted).
 */
isCurrentPeriod: boolean, 
/**
 * Grade the skill comes from when it is offered as a revision (SUB000113), otherwise `null`.
 */
revisionFrom: Grade | null, answerModes: Array<AnswerMode>, family: SkillFamily, 
/**
 * Family shown to the child: a color and a symbol on the card.
 */
category: SkillCategory, 
/**
 * Spelling: the kinds of exercise offered (dictation, missing letters, the right spelling).
 */
spellingFormats: Array<SpellingFormat>, 
/**
 * Spelling: the kinds of exercise where the word is only heard, which cannot be played
 * without the voice (SUB026704).
 */
listeningFormats: Array<SpellingFormat>, 
/**
 * Mastery of the skill by the profile (0 to 100, SUB001401), when it was practised.
 */
mastery: number | null, 
/**
 * Best stars of the profile on the skill (0 to 3, SUB001502).
 */
stars: number, };

export type SkillFamily = "Calculation" | "Multiplication" | "Numeration" | "DoubleHalf" | "Fraction" | "Problem" | "Spelling";

export type SkillCategory = "Numbers" | "AddSubtract" | "Multiply" | "FractionsHalves" | "Problems" | "LettersAndSounds" | "SpellingRules" | "SilentLetters" | "Agreement" | "WordsToKnow";

export type CharacterResponse = { id: string, name: string, kind: CharacterKind, };

export type ProfileResponse = { id: string, name: string, grade: Grade, characterId: string, 
/**
 * The accessories worn by the avatar (SUB001504).
 */
accessories: Array<WornAccessoryResponse>, points: number, 
/**
 * Seconds given per question of the level assessment.
 */
placementSeconds: number, 
/**
 * Seconds given per question of an exam; 0 is no timer.
 */
examSeconds: number, 
/**
 * ISO 8601 instant in UTC, e.g. `2026-10-11T08:00:00.000Z`.
 */
lastUsedAt: string, };

export type CreateProfileRequest = { name: string, grade: Grade, 
/**
 * Character shown on the creation screen (`draw_character`); drawn at random when absent.
 */
characterId?: string, };

export type UpdateProfileRequest = { name?: string, grade?: Grade, characterId?: string, 
/**
 * Seconds per question of the level assessment (15, 20, 30, 45 or 60).
 */
placementSeconds?: number, 
/**
 * Seconds per question of an exam (0 for no timer, 20, 30, 45 or 60).
 */
examSeconds?: number, };

export type StartSeriesRequest = { profileId: string, skillId: string, answerMode: AnswerMode, 
/**
 * Spelling: the kind of exercise chosen; every kind allowed by the answer mode when absent.
 */
format?: SpellingFormat, 
/**
 * Fixed seed to replay a series (tests, debugging); random when absent.
 */
seed?: number, };

export type SeriesResponse = { seriesId: string, sessionType: SessionType, subject: Subject, seed: number, 
/**
 * The skill of a series of exercises; null for an exam or a timed challenge.
 */
skillId: string | null, 
/**
 * The skill's instruction; empty for an exam, a timed challenge or a quiz (the front has its own).
 */
instruction: string, answerMode: AnswerMode, 
/**
 * Duration of a timed challenge, in seconds.
 */
timeLimitSeconds: number | null, 
/**
 * Seconds given per question (level assessment, exam with a timer); null without a timer.
 */
questionSeconds: number | null, exercises: Array<ExerciseResponse>, };

export type ExerciseResponse = { index: number, 
/**
 * Sentence shown above the statement (« Quel est le double de 35 ? », a problem).
 */
question: string | null, 
/**
 * The written statement, e.g. `7`, `+`, slot, `=`, `12`.
 */
statement: Array<TokenResponse>, picture: PictureResponse | null, input: InputResponse, 
/**
 * Proposals of a pick (QCM), otherwise null.
 */
choices: Array<ChoiceResponse> | null, 
/**
 * Written form with `?` for the answer, e.g. `7 + ? = 12` (accessibility).
 */
prompt: string, 
/**
 * French text to read aloud (send it to `speak`). For a spelling exercise, play the word
 * with `speak_series_word` instead.
 */
spokenPrompt: string, 
/**
 * Spelling: the word is read (`speak_series_word`) and, for the missing letters, the letters
 * around the blank. The word itself is not sent.
 */
word: WordPromptResponse | null, };

export type WordPromptResponse = { 
/**
 * Letters before the blank (missing letters), otherwise null.
 */
before: string | null, 
/**
 * Letters after the blank (missing letters), otherwise null.
 */
after: string | null, 
/**
 * Missing letters: how many letters the blank hides (shown as dashes), otherwise null.
 */
missing: number | null, 
/**
 * The word can also be heard in a short sentence.
 */
hasExample: boolean, 
/**
 * A noun: its article (« un », « une »), shown before the answer; the child writes the noun
 * only. Null for the other words.
 */
article: string | null, };

export type TokenResponse = { "kind": "number", value: number, } | { "kind": "sign", value: string, } | { "kind": "text", value: string, } | { "kind": "fraction", numerator: number, denominator: number, } | { "kind": "slot", slot: number, };

export type FractionShape = "Bar" | "Disc";

export type PictureResponse = { "kind": "baseTen", hundreds: number, tens: number, units: number, } | { "kind": "fractionShape", shape: FractionShape, parts: number, colored: number, } | { "kind": "numberLine", start: number, step: number, ticks: number, labeled: Array<number>, arrow: number | null, } | { "kind": "arrayGrid", rows: number, columns: number, split: number | null, } | { "kind": "barModel", bars: Array<Array<number | null>>, total: number | null, } | { "kind": "groups", groups: number, perGroup: number, };

export type InputResponse = { "kind": "number", slots: number, } | { "kind": "fraction" } | { "kind": "comparison" } | { "kind": "yesNo" } | { "kind": "order", items: Array<number>, ascending: boolean, } | { "kind": "multiChoice", items: Array<number>, } | { "kind": "pick" } | { "kind": "numberLine" } | { "kind": "spelling", maxLength: number, };

export type ChoiceResponse = { label: string, answer: AnswerPayload, };

export type ComparisonSign = "<" | ">" | "=";

export type AnswerPayload = { "kind": "number", value: number, } | { "kind": "fraction", numerator: number, denominator: number, } | { "kind": "comparison", sign: ComparisonSign, } | { "kind": "yesNo", value: boolean, } | { "kind": "sequence", values: Array<number>, } | { "kind": "set", values: Array<number>, } | { "kind": "text", value: string, } | { "kind": "timedOut" };

export type SeriesAnswerResponse = { correct: boolean, expected: AnswerPayload, 
/**
 * The expected answer as written in the correction (« 3/4 », « < », « 300 et 400 »).
 */
expectedLabel: string, 
/**
 * Correction shown and read after a mistake, e.g. `7 + 5 = 12`.
 */
explanation: string, 
/**
 * Picture shown with the correction (decomposition, bar model).
 */
help: PictureResponse | null, 
/**
 * Spelling: the child's word letter by letter, to highlight the mistakes (SUB001405).
 */
correction: SpellingCorrectionResponse | null, 
/**
 * Set when this answer ended the series: points awarded.
 */
summary: SeriesSummaryResponse | null, };

export type SpellingCorrectionResponse = { correct: boolean, letters: Array<LetterResponse>, };

export type LetterKindName = "Right" | "Wrong" | "Missing" | "Extra";

export type LetterResponse = { kind: LetterKindName, given: string | null, expected: string | null, };

export type SeriesSummaryResponse = { correctCount: number, exerciseCount: number, pointsEarned: number, 
/**
 * New total of the profile.
 */
profilePoints: number, 
/**
 * Results by skill, in the order of the series (an exam mixes several skills).
 */
skills: Array<SkillResultResponse>, 
/**
 * What the mascot says at the end (SUB001417): a party, happy or encouraging.
 */
mascotSituation: MascotSituation, 
/**
 * Timed challenge (SUB001415): the best score before this one at the same duration.
 */
previousBest: number | null, 
/**
 * Timed challenge: a new personal record, to celebrate.
 */
newRecord: boolean, 
/**
 * Lesson quiz (SUB001411): fewer than half right answers, « Revoir la leçon » comes first.
 */
reviewLesson: boolean, 
/**
 * Stars, level, goal of the day, badges and accessories brought by the session (lot 3).
 */
rewards: SessionRewardsResponse | null, };

export type SkillResultResponse = { skillId: string, title: string, correctCount: number, questionCount: number, 
/**
 * Lesson of the skill (« Revoir la leçon »), when there is one.
 */
lessonId: string | null, 
/**
 * A strong point of an exam (at least two thirds right); otherwise a notion to work on again.
 */
strong: boolean, };

export type TimedChallengeResponse = { subject: Subject, 
/**
 * Skills mastered enough (60 % or more) to enter the challenge; none: no challenge yet.
 */
skillCount: number, 
/**
 * The durations offered, in seconds, and the best score of each.
 */
durations: Array<TimedDurationResponse>, defaultSeconds: number, };

export type MascotExpression = "Happy" | "Thinking" | "Encouraging" | "Party";

export type MascotSituation = "LessonStart" | "SeriesPerfect" | "SeriesWell" | "SeriesEncourage" | "ExamEnd" | "TimedEnd" | "LevelUp";

export type MascotResponse = { name: string, pictures: Array<MascotPictureResponse>, };

export type MascotPictureResponse = { expression: MascotExpression, svg: string, };

export type MascotPhraseResponse = { text: string, expression: MascotExpression, };

export type TimedDurationResponse = { seconds: number, 
/**
 * Best number of right answers, when the profile already played this duration.
 */
best: number | null, };

export type PlacementStatusResponse = { 
/**
 * One entry per subject, in the order of the menu.
 */
subjects: Array<PlacementSubjectResponse>, 
/**
 * Seconds given per question, from the profile's options.
 */
questionSeconds: number, };

export type PlacementSubjectResponse = { subject: Subject, 
/**
 * Not done yet and the grade has skills in the subject.
 */
available: boolean, 
/**
 * Finished: an abandoned assessment does not count.
 */
done: boolean, };

export type MultiplicationTableResponse = { number: number, lines: Array<TableLineResponse>, };

export type TableLineResponse = { left: number, right: number, result: number, 
/**
 * « 3 fois 7, 21. »
 */
spoken: string, };

export type LessonsResponse = { subject: Subject, 
/**
 * Grade asked for.
 */
grade: Grade, 
/**
 * Grade of the returned lessons: the asked one, or CE1 when the asked grade has no lesson yet.
 */
lessonsGrade: Grade, 
/**
 * School period of today (1 to 5).
 */
currentPeriod: number, lessons: Array<LessonSummaryResponse>, };

export type LessonSummaryResponse = { id: string, title: string, period: number, 
/**
 * True for the lessons of the current period.
 */
isCurrentPeriod: boolean, durationMinutes: number, 
/**
 * Family of the main skill of the lesson (its first one), `null` when it is not loaded.
 */
category: SkillCategory | null, 
/**
 * Best stars of the profile on the quiz of the lesson (0 to 3, SUB001502).
 */
stars: number, 
/**
 * The main skill of the lesson (its first one): the progress map links it (SUB001506).
 */
skillId: string | null, };

export type LessonResponse = { id: string, title: string, subject: Subject, grade: Grade, 
/**
 * Skills practised after the lesson (« Je m'entraîne »), the first one being the main one.
 */
skillIds: Array<string>, slides: Array<SlideResponse>, 
/**
 * The questions of the quiz (answered through `start_lesson_quiz` and `answer_series`).
 */
quiz: Array<QuizQuestionResponse>, 
/**
 * The slide where the profile left the lesson, when it did not finish it (SUB001410).
 */
resumeSlide: number | null, };

export type SlideResponse = { title: string, 
/**
 * Shown.
 */
text: string, 
/**
 * Read aloud (send it to `speak`).
 */
speech: string, 
/**
 * The child has to do something before going on.
 */
interactive: boolean, animation: AnimationResponse, };

export type AnimationResponse = { "type": "titleCard", title: string, subtitle: string | null, } | { "type": "recapCard", text: string, example: string | null, } | { "type": "calcChain", chains: Array<ChainResponse>, mode: ChainMode, } | { "type": "numberLine", from: number, to: number, step: number, start: number, jumps: Array<JumpResponse>, 
/**
 * The child clicks each landing graduation, in order.
 */
interactive: boolean, 
/**
 * The value of a graduation shows under the pointer (interactive line).
 */
hoverValues: boolean, 
/**
 * Graduations marked with a checkered flag.
 */
flags: Array<number>, } | { "type": "tensUnits", value: number, hundreds: number, tens: number, units: number, missing: number, steps: Array<BlockStepResponse>, 
/**
 * The number added or removed, or the number of parts of a share.
 */
change: number | null, 
/**
 * The sum, the difference, or one part of the share.
 */
result: number, parts: number, interactive: boolean, task: BlockTask | null, 
/**
 * Interactive: the blocks the child starts with, and the blocks to reach.
 */
interactiveStart: BlocksResponse, target: BlocksResponse, } | { "type": "columnOperation", operator: LessonOperator, operands: Array<number>, result: number, layouts: Array<WrittenOperationResponse>, 
/**
 * Columns the animation computes, from the units (0: the numbers are only set in columns).
 */
computedColumns: number, 
/**
 * The child writes the digit of each column of the result, from the units.
 */
interactive: boolean, } | { "type": "equation", left: number, operator: LessonOperator, right: number, result: number, missing: EquationTerm | null, 
/**
 * The value in the box.
 */
missingValue: number | null, 
/**
 * The equality first, then the other equalities of its family.
 */
equalities: Array<EqualityResponse>, 
/**
 * The child picks the missing term among `choices`.
 */
interactive: boolean, choices: Array<number>, } | { "type": "compareNumbers" } & CompareNumbersResponse | { "type": "sequencePattern" } & SequencePatternResponse | { "type": "arrayGrid" } & ArrayGridResponse | { "type": "timesTable" } & TimesTableResponse | { "type": "placeValueTable" } & PlaceValueTableResponse | { "type": "groupObjects" } & GroupObjectsResponse | { "type": "tenFrame" } & TenFrameResponse | { "type": "fractionBar" } & FractionBarResponse | { "type": "barModel" } & BarModelResponse | { "type": "letterTiles", word: string, tiles: Array<string>, distractors: Array<string>, order: Array<number>, interactive: boolean, } | { "type": "soundSort", groups: Array<string>, words: Array<SortedWordResponse>, interactive: boolean, };

export type SortedWordResponse = { word: string, group: number, start: number, length: number, };

export type LessonOperator = "Add" | "Subtract" | "Multiply";

export type SubtractionMethod = "Cassage" | "Compensation";

export type WrittenOperationResponse = { method: SubtractionMethod | null, 
/**
 * From the units to the left.
 */
columns: Array<WrittenColumnResponse>, };

export type WrittenColumnResponse = { 
/**
 * The digit of each number, from top to bottom (null: the number has no digit here).
 */
digits: Array<number | null>, 
/**
 * Addition: carry above the column. Compensation: 1 ten added to the bottom number here.
 */
carry: number, 
/**
 * Cassage: the top digit crossed out and rewritten, with the column computed at that moment.
 */
rewrites: Array<RewriteResponse>, 
/**
 * Compensation: 10 units added to the top digit.
 */
plusTen: boolean, 
/**
 * The numbers the column adds (carry first), or the top value and the value removed.
 */
terms: Array<number>, value: number, 
/**
 * The digit written in the result.
 */
digit: number, 
/**
 * False for a zero on the left of the result, not written.
 */
written: boolean, };

export type RewriteResponse = { atColumn: number, value: number, };

export type EquationTerm = "Left" | "Right" | "Result";

export type EqualityResponse = { left: number, operator: LessonOperator, right: number, result: number, };

export type BlocksResponse = { hundreds: number, tens: number, units: number, value: number, };

export type BlockStepName = "Start" | "Join" | "ExchangeUnits" | "ExchangeTens" | "BreakTen" | "BreakHundred" | "TakeAway" | "Share";

export type BlockStepResponse = { kind: BlockStepName, blocks: BlocksResponse, highlight: BlocksResponse, };

export type BlockTask = "Build" | "Add" | "Remove" | "Share";

export type RankName = "Units" | "Tens" | "Hundreds" | "Thousands";

export type RankDigitsResponse = { rank: RankName, a: number | null, b: number | null, };

export type CompareNumbersResponse = { a: number, b: number, sign: ComparisonSign, 
/**
 * From the highest rank (on the left) to the units.
 */
ranks: Array<RankDigitsResponse>, 
/**
 * The first rank, from the left, where the digits differ (null: equal numbers).
 */
decisiveRank: RankName | null, 
/**
 * The child picks the sign.
 */
interactive: boolean, };

export type RankCountResponse = { rank: RankName, count: number, };

export type SpellingsResponse = { traditional: string, rectified: string, };

export type PlaceValueTableResponse = { value: number, 
/**
 * The columns, from the left.
 */
ranks: Array<RankName>, 
/**
 * The digit in each column (null: nothing on the left of the number).
 */
digits: Array<number | null>, focus: RankName | null, countOf: RankCountResponse | null, 
/**
 * 300 + 40 + 7 (empty when not shown).
 */
decomposition: Array<number>, words: SpellingsResponse | null, 
/**
 * The child writes the number, digit by digit from the left.
 */
interactive: boolean, };

export type SequenceMode = "Count" | "Order";

export type SequencePatternResponse = { mode: SequenceMode, 
/**
 * Count: the sequence. Order: the numbers as given (not in order).
 */
items: Array<number>, 
/**
 * Count: the sequence. Order: the numbers in the order asked.
 */
ordered: Array<number>, 
/**
 * Count: the positions (from 0) hidden in a box, then revealed.
 */
hidden: Array<number>, 
/**
 * Count: the step (negative: backwards).
 */
step: number, 
/**
 * Order: from the largest.
 */
descending: boolean, 
/**
 * Order: the child clicks the numbers in order.
 */
interactive: boolean, };

export type GridPartResponse = { rows: number, cols: number, product: number, };

export type GridTask = "Build" | "Split";

export type ArrayGridResponse = { rows: number, cols: number, total: number, 
/**
 * The rows after which the grid is cut.
 */
split: number | null, 
/**
 * From the top: one part, or two when cut.
 */
parts: Array<GridPartResponse>, 
/**
 * The rows added one by one (5 + 5 + 5); empty when not shown.
 */
repeated: Array<number>, rotate: boolean, removed: number, remaining: number, task: GridTask | null, };

export type TableHighlightKind = "Row" | "Doubles" | "Result";

export type TableHighlightResponse = { kind: TableHighlightKind, value: number | null, };

export type TableCellResponse = { row: number, col: number, value: number, 
/**
 * Index of the highlight group (in `highlights`) that lights the cell.
 */
group: number | null, };

export type TimesTableResponse = { operator: LessonOperator, size: number, 
/**
 * Rows from 1 to `size`.
 */
cells: Array<Array<TableCellResponse>>, highlights: Array<TableHighlightResponse>, };

export type GroupModeName = "Join" | "Remove" | "GroupBy" | "Share" | "SortParity";

export type RaceObjectName = "Car" | "Helmet" | "Tire" | "Flag" | "Cone";

export type GroupingResponse = { count: number, groups: number, rest: number, };

export type GroupObjectsResponse = { mode: GroupModeName, object: RaceObjectName, 
/**
 * Join: the two groups. GroupBy: the collections.
 */
counts: Array<number>, 
/**
 * Remove, share: the objects at the start.
 */
total: number, removed: number, 
/**
 * GroupBy: size of a group. Share: number of parts.
 */
size: number, 
/**
 * The total, what remains, one part.
 */
result: number, groupings: Array<GroupingResponse>, 
/**
 * SortParity: the numbers and whether each one is even.
 */
numbers: Array<number>, parities: Array<boolean>, 
/**
 * Join: the groups swap places.
 */
commute: boolean, interactive: boolean, 
/**
 * Interactive join or removal: proposals for the result.
 */
choices: Array<number>, };

export type FrameSlotName = "Empty" | "Filled" | "Added" | "Removed";

export type FriendPairResponse = { a: number, b: number, };

export type TenFrameResponse = { frames: number, 
/**
 * The places, garage after garage, in reading order (2 rows of 5).
 */
slots: Array<FrameSlotName>, filled: number, added: number, removed: number, 
/**
 * The cars in the garages at the end.
 */
total: number, friends: Array<FriendPairResponse>, 
/**
 * The child parks the added cars, or makes the removed ones leave.
 */
interactive: boolean, };

export type FractionStripResponse = { denominator: number, 
/**
 * Colored parts, one color per number.
 */
numerators: Array<number>, colored: number, 
/**
 * « 3/4 ».
 */
label: string, 
/**
 * « trois quarts ».
 */
words: string, };

export type FractionBarResponse = { bars: Array<FractionStripResponse>, 
/**
 * Two bars: the sign between their colored fractions.
 */
comparison: ComparisonSign | null, unequal: boolean, notation: boolean, 
/**
 * The child colors the parts of the only bar.
 */
interactive: boolean, };

export type BarKindName = "Parts" | "Compare" | "Groups";

export type BarValueResponse = { part: number | null, value: number, 
/**
 * Shown as « ? » until solved.
 */
unknown: boolean, };

export type BarModelResponse = { kind: BarKindName, 
/**
 * The whole first, then the parts.
 */
values: Array<BarValueResponse>, 
/**
 * Groups: the number of equal parts.
 */
groups: number, labels: Array<string>, answer: number, 
/**
 * The calculation that finds the « ? ».
 */
solution: EqualityResponse, 
/**
 * For « 4 × ? = 24 »: the term hidden in the solution.
 */
solutionMissing: EquationTerm | null, solve: boolean, 
/**
 * The child places the known numbers on the diagram.
 */
interactive: boolean, };

export type ChainMode = "Show" | "Match" | "Pick";

export type ChainResponse = { start: number, steps: Array<JumpResponse>, 
/**
 * What the chain adds in all (+ 20 − 1 → 19): the calculation is `start + total`.
 */
total: number, result: number, 
/**
 * A classic mistake, shown crossed out.
 */
crossed: boolean, };

export type JumpResponse = { delta: number, value: number, };

export type QuizQuestionResponse = { index: number, question: string, 
/**
 * Read aloud (send it to `speak`).
 */
spoken: string, choices: Array<ChoiceResponse> | null, };

export type ParentGateResponse = { 
/**
 * To send back with the answer (`check_parent_gate`).
 */
id: string, left: number, right: number, };

export type ParentProfileResponse = { name: string, characterId: string, };

export type UpdateParentProfileRequest = { name?: string, characterId?: string, };

export type MasteryStateName = "TooEarly" | "Mastered" | "InProgress" | "ToReview";

export type TrendName = "Unknown" | "Up" | "Stable" | "Down";

export type ProfileStatisticsResponse = { profileId: string, summary: StatisticsSummaryResponse, 
/**
 * The last 14 days, today last.
 */
days: Array<DayActivityResponse>, 
/**
 * The last 8 weeks (Monday to Sunday), the current week last.
 */
weeks: Array<WeekActivityResponse>, 
/**
 * Every exercise practised, the most recently practised first.
 */
skills: Array<SkillStatisticsResponse>, 
/**
 * Up to 3 exercises to review first (ids of `skills`), the weakest first.
 */
toReview: Array<string>, 
/**
 * Exams and timed challenges, the most recent first.
 */
tests: Array<TestResultResponse>, frequentMistakes: Array<FrequentMistakeResponse>, };

export type StatisticsSummaryResponse = { sessionCount: number, 
/**
 * Time spent in the exercises, exams, challenges and lesson quizzes.
 */
totalSeconds: number, pointsEarned: number, answerCount: number, correctCount: number, successRate: number | null, 
/**
 * Days with an activity among the last 14.
 */
activeDays: number, lastActivityAt: string | null, };

export type DayActivityResponse = { date: string, seconds: number, sessionCount: number, };

export type WeekActivityResponse = { 
/**
 * The Monday.
 */
start: string, seconds: number, sessionCount: number, answerCount: number, correctCount: number, successRate: number | null, pointsEarned: number, };

export type SkillStatisticsResponse = { skillId: string, 
/**
 * Title of the skill (its id when the skill no longer exists).
 */
title: string, subject: Subject, 
/**
 * Grade of the skill, when it still exists.
 */
grade: Grade | null, 
/**
 * Series answered to the end.
 */
completedCount: number, 
/**
 * Series stopped before the end.
 */
abandonedCount: number, 
/**
 * Every answer, exams and timed challenges included.
 */
answerCount: number, correctCount: number, 
/**
 * Share of right answers among the last 20 (the score shown to the child).
 */
mastery: number, state: MasteryStateName, 
/**
 * Points of the series of this exercise (an exam mixes exercises: its points are not split).
 */
pointsEarned: number, averagePoints: number | null, bestPoints: number | null, trend: TrendName, lastPracticedAt: string, 
/**
 * The last 20 completed series, oldest first: the points chart.
 */
history: Array<SeriesPointResponse>, };

export type SeriesPointResponse = { startedAt: string, date: string, sessionType: SessionType, questionCount: number, correctCount: number, pointsEarned: number, };

export type TestResultResponse = { sessionType: SessionType, subject: Subject, startedAt: string, date: string, questionCount: number, correctCount: number, pointsEarned: number, timeLimitSeconds: number | null, abandoned: boolean, };

export type FrequentMistakeResponse = { skillId: string, skillTitle: string, 
/**
 * The question as written, `?` for the answer.
 */
prompt: string, expected: string, mistakeCount: number, attemptCount: number, usualAnswer: string, lastAt: string, };

export type LevelResponse = { level: number, points: number, 
/**
 * Points from which the current level was reached.
 */
levelStart: number, 
/**
 * Points from which the next level is reached.
 */
nextLevelAt: number, 
/**
 * Way done to the next level, 0 to 100.
 */
percent: number, };

export type DailyGoalResponse = { 
/**
 * Series asked today; 0 when the parents turned the goal off.
 */
target: number, done: number, reached: boolean, 
/**
 * Days in a row the goal was reached.
 */
streak: number, bonusPoints: number, };

export type ProfileOverviewResponse = { level: LevelResponse, goal: DailyGoalResponse, badgesEarned: number, badgesTotal: number, 
/**
 * Items of « Mes révisions » due today, by subject (SUB001509).
 */
reviews: Array<ReviewCountResponse>, };

export type ReviewCountResponse = { subject: Subject, due: number, };

export type BadgeResponse = { id: string, title: string, 
/**
 * The condition said simply (shown under a badge to win).
 */
description: string, 
/**
 * SVG source of the icon.
 */
icon: string, earned: boolean, 
/**
 * ISO 8601 instant in UTC.
 */
earnedAt: string | null, };

export type AccessorySlotName = "Head" | "Eyes" | "Neck" | "Companion";

export type WornAccessoryResponse = { accessoryId: string, colorId: string, };

export type AccessoryColorResponse = { id: string, name: string, 
/**
 * `#RRGGBB`, for the color button.
 */
main: string, };

export type UnlockResponse = { level: number | null, badgeId: string | null, badgeTitle: string | null, };

export type AccessoryResponse = { id: string, name: string, slot: AccessorySlotName, colors: Array<AccessoryColorResponse>, unlocked: boolean, unlock: UnlockResponse, };

export type UpdateAvatarRequest = { characterId?: string, accessories: Array<WornAccessoryResponse>, };

export type SessionRewardsResponse = { 
/**
 * Stars of this series or quiz; `null` for an exam, a challenge, revisions.
 */
stars: number | null, previousBestStars: number, levelBefore: LevelResponse, level: LevelResponse, goal: DailyGoalResponse, 
/**
 * Bonus of the goal of the day given now (0 otherwise).
 */
goalBonus: number, newBadges: Array<BadgeResponse>, newAccessories: Array<AccessoryResponse>, };

export type AdviceReasonName = "Start" | "Continue" | "Consolidate" | "Next";

export type AdviceResponse = { skillId: string, title: string, reason: AdviceReasonName, 
/**
 * Consolidate: the title of the skill it prepares.
 */
prepares: string | null, };

export type VoiceSpeedName = "Slower" | "Normal" | "Faster";

export type TextSizeName = "Normal" | "Large" | "Larger";

export type ProfileSettingsResponse = { sound: boolean, mascot: boolean, voiceSpeed: VoiceSpeedName, textSize: TextSizeName, dyslexiaFont: boolean, theme: string, 
/**
 * Series a day; 0: no goal.
 */
dailyGoal: number, timedEnabled: boolean, 
/**
 * Duration of the timed challenge; 0: the child chooses.
 */
timedSeconds: number, 
/**
 * Advised length of a session before a break reminder; 0: none.
 */
sessionMinutes: number, };

export type UpdateProfileSettingsRequest = { sound?: boolean, mascot?: boolean, voiceSpeed?: VoiceSpeedName, textSize?: TextSizeName, dyslexiaFont?: boolean, theme?: string, dailyGoal?: number, timedEnabled?: boolean, timedSeconds?: number, sessionMinutes?: number, };

export type ThemeResponse = { id: string, name: string, description: string, };

export type SettingsOptionsResponse = { themes: Array<ThemeResponse>, goalChoices: Array<number>, 
/**
 * Durations of the timed challenge the parents may fix (0: the child chooses).
 */
timedChoices: Array<number>, sessionChoices: Array<number>, };

export type BackupResponse = { 
/**
 * Profiles in the file.
 */
profiles: number, 
/**
 * Where the file was written or read.
 */
path: string, };

export type CommandError = { status: number, title: string, detail: string | null, 
/**
 * Request field in error (`name`, `grade`, `characterId`), for the profile errors.
 */
field: string | null, };
