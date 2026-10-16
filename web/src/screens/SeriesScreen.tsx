import { useMutation, useQueryClient } from "@tanstack/react-query";
import { AnimatePresence, motion } from "motion/react";
import { useEffect, useRef, useState, type FormEvent, type ReactNode } from "react";
import { useNavigate, useParams, useSearchParams } from "react-router";
import type { AnswerMode, AnswerPayload, Exercise, Profile, Series, SeriesAnswer, SeriesSummary, SpellingFormat } from "../api/client";
import { playSound } from "../audio/sounds";
import { playVoice, stopSpeech, useVoiceWhile } from "../audio/useSpeech";
import { useApi } from "../app/BackendContext";
import { answerInputMode } from "../app/device";
import { invalidateProgress } from "../app/queries";
import { ChoiceGrid } from "../components/child/ChoiceGrid";
import { Feedback } from "../components/child/Feedback";
import { Fireworks } from "../components/child/Fireworks";
import { NumberPad } from "../components/child/NumberPad";
import { PointsBadge } from "../components/child/PointsBadge";
import { RaceTrack } from "../components/child/RaceTrack";
import { SpeakButton } from "../components/child/SpeakButton";
import { MultiChoiceInput, OrderInput, SignButtons, YesNoButtons } from "../components/exercise/AnswerInputs";
import { FractionText } from "../components/exercise/AnswerText";
import { ExercisePicture } from "../components/exercise/ExercisePicture";
import { Statement } from "../components/exercise/Statement";
import { TortoiseIcon } from "../components/icons";
import { RequireProfile } from "../components/RequireProfile";
import { SpellingAnswer } from "../components/spelling/SpellingAnswer";
import { WordCorrection } from "../components/spelling/WordCorrection";
import { QuestionTimer } from "../components/child/QuestionTimer";
import { TimerBar } from "../components/child/TimerBar";
import { Mascot } from "../components/child/Mascot";
import { RewardsPanel } from "../components/child/Rewards";
import { fr } from "../i18n/fr";

/** Pause on a right answer before the next exercise comes by itself. */
export const AUTO_NEXT_DELAY_MS = 900;

/** Timed challenge: a wrong answer shows the right one briefly, then the next question comes (no penalty). */
export const TIMED_WRONG_DELAY_MS = 1500;

const MAX_ANSWER_LENGTH = 10;

/** The core reads numbers as 32-bit integers: a longer number is simply a wrong answer. */
const toNumber = (text: string) => Math.min(Number.parseInt(text, 10), 2_147_483_647);

/** Same answer (a picked proposal, the expected one): same kind and same values. */
const sameAnswer = (a: AnswerPayload, b: AnswerPayload) => JSON.stringify(a) === JSON.stringify(b);

const SPELLING_FORMATS: readonly SpellingFormat[] = ["Dictation", "MissingLetters", "ChooseSpelling"];

/**
 * A series of 10 exercises (SUB000109, SUB000110, SUB000111; CE1 families SUB000118 to SUB000124;
 * spelling SUB001406 to SUB001408, the kind of exercise in `?format=`).
 */
export function SeriesScreen() {
  return <RequireProfile>{(profile) => <SkillSeries profile={profile} />}</RequireProfile>;
}

function SkillSeries({ profile }: { profile: Profile }) {
  const api = useApi();
  const { skillId = "", mode = "Keyboard" } = useParams();
  const [search] = useSearchParams();
  const answerMode: AnswerMode = mode === "Choice" ? "Choice" : "Keyboard";
  const format = SPELLING_FORMATS.find((f) => f === search.get("format"));
  return (
    <SeriesRun
      profile={profile}
      answerMode={answerMode}
      start={() => (format ? api.startSeries(profile.id, skillId, answerMode, format) : api.startSeries(profile.id, skillId, answerMode))}
    />
  );
}

type Phase =
  | { kind: "answering" }
  | { kind: "answered"; outcome: SeriesAnswer; given: AnswerPayload }
  | { kind: "finished"; summary: SeriesSummary };

/** Number of typed fields of an exercise: one number, two numbers (framing), a fraction. */
function typedSlots(exercise: Exercise): number {
  switch (exercise.input.kind) {
    case "number":
      return exercise.input.slots;
    case "fraction":
      return 2;
    default:
      return 0;
  }
}

type SeriesRunProps = {
  profile: Profile;
  answerMode: AnswerMode;
  /** Creates the series (exercises of a skill, an exam, a timed challenge). */
  start(): Promise<Series>;
  /** The end of the series, instead of the usual summary (exam, timed challenge). */
  renderSummary?(summary: SeriesSummary, again: () => void): ReactNode;
  /** What the speaker of the end reads, instead of the usual sentence. */
  spokenSummary?(summary: SeriesSummary): string;
};

/** The instruction above the questions: the skill's one (content), the screen's one for an exam or a
 * timed challenge. */
function instructionOf(series: Series): string {
  switch (series.sessionType) {
    case "Exam":
      return fr.exam.instruction;
    case "Timed":
      return fr.exam.timedInstruction;
    case "Placement":
      return fr.placement.instruction;
    case "Review":
      return fr.reviews.instruction;
    default:
      return series.instruction;
  }
}

/** Plays the word of a spelling exercise, alone or in its sentence, or very slowly (SUB026702). */
function useSeriesWord() {
  const api = useApi();
  return (seriesId: string, index: number, example = false, slow = false) =>
    playVoice(() => api.speakSeriesWord(seriesId, index, example, slow));
}

export function SeriesRun({ profile, answerMode, start: create, renderSummary, spokenSummary }: SeriesRunProps) {
  const api = useApi();
  const queryClient = useQueryClient();
  const navigate = useNavigate();
  const playWord = useSeriesWord();

  const [attempt, setAttempt] = useState(0);
  const [series, setSeries] = useState<Series | null>(null);
  const [index, setIndex] = useState(0);
  const [values, setValues] = useState<string[]>([]);
  const [active, setActive] = useState(0);
  const [results, setResults] = useState<(boolean | null)[]>([]);
  const [phase, setPhase] = useState<Phase>({ kind: "answering" });
  const [confirmQuit, setConfirmQuit] = useState(false);
  const [points, setPoints] = useState(profile.points);
  // The timer of a question (level assessment, exam with a timer) runs once the question is read.
  const [timerRunning, setTimerRunning] = useState(false);
  // Which question the timer belongs to: a late end of reading must not start the next one's.
  const turn = useRef(0);
  const started = useRef(-1);
  const inputRefs = useRef<(HTMLInputElement | null)[]>([]);
  // When the exercise was shown, for the time taken to answer (SUB001401).
  const shownAt = useRef(Date.now());
  // The voice is on for the French exercises.
  useVoiceWhile(series?.subject === "Spelling");

  const timed = series?.sessionType === "Timed";
  const questionSeconds = series?.questionSeconds ?? null;
  const questionTimed = questionSeconds !== null && questionSeconds > 0;
  // With a timer on each question, the next one comes by itself, as in a timed challenge.
  const autoNext = timed || questionTimed;

  const startTimer = (token: number) => {
    if (turn.current === token) {
      setTimerRunning(true);
    }
  };

  const resetAnswer = () => {
    setValues([]);
    setActive(0);
  };

  const start = useMutation({
    mutationFn: () => create(),
    onSuccess: async (created) => {
      setSeries(created);
      setIndex(0);
      resetAnswer();
      setResults(created.exercises.map(() => null));
      setPhase({ kind: "answering" });
      shownAt.current = Date.now();
      const token = ++turn.current;
      // Nothing is read by itself (decision of 2026-10-15): the child clicks « Écouter le mot » or
      // the speakers. The time of a question starts as soon as it is shown.
      startTimer(token);
    },
  });

  const { mutate: startSeries } = start;
  useEffect(() => {
    // Once per attempt, even when React runs the effect twice (StrictMode).
    if (started.current !== attempt) {
      started.current = attempt;
      startSeries();
    }
  }, [attempt, startSeries]);

  useEffect(() => () => stopSpeech(), []);

  /** The sentence of the end, read on click of its speaker. */
  const spokenEnd = (summary: SeriesSummary) => {
    const perfect = summary.exerciseCount > 0 && summary.correctCount === summary.exerciseCount;
    const spoken = spokenSummary ? spokenSummary(summary) : fr.series.finishedSpoken(summary.correctCount, summary.exerciseCount, summary.pointsEarned);
    return perfect ? `${fr.series.allRight} ${spoken}` : spoken;
  };

  const finish = (summary: SeriesSummary) => {
    setPoints(summary.profilePoints);
    setPhase({ kind: "finished", summary });
    // Everything right: fireworks, a fanfare and « Bravo, tout est juste ! » (SUB000114).
    const perfect = summary.exerciseCount > 0 && summary.correctCount === summary.exerciseCount;
    playSound(perfect ? "celebration" : "finished");
    void invalidateProgress(queryClient);
  };

  const goNext = (outcome: SeriesAnswer) => {
    if (outcome.summary) {
      finish(outcome.summary);
      return;
    }

    const next = index + 1;
    setIndex(next);
    resetAnswer();
    setPhase({ kind: "answering" });
    shownAt.current = Date.now();
    const token = ++turn.current;
    setTimerRunning(false);
    // The word is heard on click of « Écouter le mot »: the time starts at once.
    stopSpeech();
    startTimer(token);
  };

  const check = useMutation({
    mutationFn: (given: AnswerPayload) => api.answerSeries(series!.seriesId, index, given, Math.max(0, Math.round(Date.now() - shownAt.current))),
    onSuccess: (outcome, given) => {
      setResults((current) => current.map((r, i) => (i === index ? outcome.correct : r)));
      // The time of a challenge may run out while the answer is on its way: the end stays shown.
      setPhase((current) => (current.kind === "finished" ? current : { kind: "answered", outcome, given }));
    },
  });

  // A right answer moves on by itself; after a mistake the child reads the correction and clicks
  // « Suivant » — in a timed challenge, or with a timer on each question, the next question comes by
  // itself too.
  useEffect(() => {
    if (phase.kind !== "answered" || (!phase.outcome.correct && !autoNext)) {
      return;
    }

    const timer = window.setTimeout(() => goNext(phase.outcome), phase.outcome.correct ? AUTO_NEXT_DELAY_MS : TIMED_WRONG_DELAY_MS);
    return () => window.clearTimeout(timer);
    // goNext only uses state setters: the phase alone decides.
  }, [phase]);

  useEffect(() => {
    if (phase.kind === "answering") {
      inputRefs.current[active]?.focus();
    }
  }, [phase, index, active]);

  // The end of a timed challenge: the time is over.
  const finishTimed = useMutation({
    mutationFn: () => api.finishSeries(series!.seriesId),
    onSuccess: finish,
  });

  const abandon = useMutation({
    mutationFn: () => api.abandonSeries(series!.seriesId),
    onSettled: async () => {
      await invalidateProgress(queryClient);
      await navigate("/menu");
    },
  });

  const submit = (given: AnswerPayload) => {
    if (phase.kind === "answering" && !check.isPending) {
      check.mutate(given);
    }
  };

  const again = () => {
    setSeries(null);
    setAttempt((a) => a + 1);
  };

  if (start.isError) {
    return (
      <main className="centered">
        <p className="message">{fr.app.error}</p>
        <button type="button" className="button" onClick={() => void navigate(-1)}>
          {fr.common.back}
        </button>
      </main>
    );
  }

  if (!series) {
    return (
      <main className="centered">
        <p className="message">{fr.series.starting}</p>
      </main>
    );
  }

  const exercise = series.exercises[Math.min(index, series.exercises.length - 1)];
  const total = series.exercises.length;
  const answeredCount = results.filter((r) => r !== null).length;
  const answered = phase.kind === "answered" ? phase : null;
  const slots = typedSlots(exercise);
  const typing = slots > 0 && phase.kind === "answering";
  const value = (slot: number) => values[slot] ?? "";
  const setValue = (slot: number, text: string) =>
    setValues((current) => {
      const next = [...current];
      next[slot] = text.replace(/\D/g, "").slice(0, MAX_ANSWER_LENGTH);
      return next;
    });

  /** The typed answer, once every field is filled; otherwise the next empty field gets the focus. */
  const submitTyped = () => {
    const empty = Array.from({ length: slots }, (_, i) => i).find((i) => value(i) === "");
    if (empty !== undefined) {
      setActive(empty);
      return;
    }

    if (exercise.input.kind === "fraction") {
      submit({ kind: "fraction", numerator: toNumber(value(0)), denominator: toNumber(value(1)) });
    } else if (slots === 2) {
      submit({ kind: "sequence", values: [toNumber(value(0)), toNumber(value(1))] });
    } else {
      submit({ kind: "number", value: toNumber(value(0)) });
    }
  };

  const onSubmitForm = (event: FormEvent) => {
    event.preventDefault();
    if (value(active) !== "") {
      if (active + 1 < slots && value(active + 1) === "") {
        setActive(active + 1);
      } else {
        submitTyped();
      }
    }
  };

  const field = (slot: number, label: string) => (
    <input
      ref={(element) => {
        inputRefs.current[slot] = element;
      }}
      className={slots === 1 ? "answer" : "answer answer-small"}
      aria-label={label}
      inputMode={answerInputMode("numeric")}
      autoComplete="off"
      maxLength={MAX_ANSWER_LENGTH}
      value={value(slot)}
      onFocus={() => setActive(slot)}
      onChange={(e) => setValue(slot, e.target.value)}
    />
  );

  /** What the child gave, written in a slot of the statement. */
  const givenInSlot = (slot: number): ReactNode => {
    if (!answered) {
      return null;
    }
    const given = answered.given;
    switch (given.kind) {
      case "number":
        return given.value;
      case "sequence":
        return given.values[slot];
      case "fraction":
        return <FractionText numerator={given.numerator} denominator={given.denominator} />;
      case "comparison":
        return given.sign;
      default:
        return null;
    }
  };

  const renderSlot = (slot: number): ReactNode => {
    if (answered) {
      return <span className={`equation-slot ${answered.outcome.correct ? "slot-right" : "slot-wrong"}`}>{givenInSlot(slot) ?? "?"}</span>;
    }

    if (typing && exercise.input.kind === "fraction") {
      return (
        <span className="fraction fraction-input">
          {field(0, fr.series.numerator)}
          <span className="fraction-bar" aria-hidden="true" />
          {field(1, fr.series.denominator)}
        </span>
      );
    }

    if (typing) {
      return field(slot, slots === 1 ? fr.series.answerLabel : fr.series.slot(slot));
    }

    return <span className="equation-slot">?</span>;
  };

  const choiceLabels = exercise.choices?.map((c) => (c.label === "" ? fr.spelling.nothing : c.label)) ?? [];
  const word = exercise.word;
  const blank = word !== null && (word.before !== null || word.after !== null);
  const given = answered?.given.kind === "text" ? answered.given.value : null;
  const pickedChoice = answered && exercise.choices ? exercise.choices.findIndex((c) => sameAnswer(c.answer, answered.given)) : null;
  const expectedChoice = answered && exercise.choices ? exercise.choices.findIndex((c) => sameAnswer(c.answer, answered.outcome.expected)) : null;
  const expected = answered?.outcome.expected ?? null;
  const disabled = phase.kind !== "answering" || check.isPending;

  return (
    <main className="series-screen">
      <header className="series-header">
        <button type="button" className="back-button" onClick={() => (phase.kind === "finished" ? void navigate("/menu") : setConfirmQuit(true))}>
          {phase.kind === "finished" ? `← ${fr.series.menu}` : fr.series.quit}
        </button>
        <span className="series-title">{timed ? fr.exam.timedTitle : fr.series.progress(Math.min(index + 1, total), total)}</span>
        <PointsBadge points={points} />
      </header>

      {timed && series.timeLimitSeconds !== null ? (
        phase.kind !== "finished" && (
          <TimerBar
            seconds={series.timeLimitSeconds}
            rightCount={results.filter((r) => r === true).length}
            onOver={() => {
              if (!finishTimed.isPending && !finishTimed.isSuccess) {
                finishTimed.mutate();
              }
            }}
          />
        )
      ) : (
        <RaceTrack done={answeredCount} total={total} results={results} label={fr.series.progress(Math.min(index + 1, total), total)} />
      )}

      {questionSeconds !== null && questionTimed && phase.kind === "answering" && (
        <QuestionTimer
          key={`timer-${index}`}
          seconds={questionSeconds}
          running={timerRunning && !confirmQuit && !check.isPending}
          onOver={() => submit({ kind: "timedOut" })}
        />
      )}

      {confirmQuit && phase.kind !== "finished" && (
        <div className="quit-confirm" role="alertdialog" aria-labelledby="quit-question">
          <p id="quit-question">{series.sessionType === "Placement" ? fr.placement.quitQuestion : fr.series.quitQuestion}</p>
          <div className="actions">
            <button type="button" className="button" onClick={() => setConfirmQuit(false)} autoFocus>
              {fr.series.quitNo}
            </button>
            <button type="button" className="button button-primary" onClick={() => abandon.mutate()} disabled={abandon.isPending}>
              {fr.series.quitYes}
            </button>
          </div>
        </div>
      )}

      {phase.kind === "finished" ? (
        <>
          {renderSummary ? (
            renderSummary(phase.summary, again)
          ) : (
            <Summary summary={phase.summary} onAgain={again} onMenu={() => void navigate("/menu")} />
          )}
          {/* The summary is read on click only. */}
          <SpeakButton text={spokenEnd(phase.summary)} label={fr.series.listenSummary} />
        </>
      ) : (
        <section className="exercise-card" aria-live="polite">
          <div className="exercise-instruction">
            <span>{instructionOf(series)}</span>
            {word ? (
              <span className="word-listen">
                <SpeakButton text={instructionOf(series)} label={fr.series.listenInstruction} compact />
                <button type="button" className="button" onClick={() => void playWord(series.seriesId, index)}>
                  {fr.spelling.listenWord}
                </button>
                <button type="button" className="button" onClick={() => void playWord(series.seriesId, index, false, true)}>
                  <TortoiseIcon size={24} />
                  {fr.spelling.listenSlowly}
                </button>
                {word.hasExample && (
                  <button type="button" className="button" onClick={() => void playWord(series.seriesId, index, true)}>
                    {fr.spelling.listenExample}
                  </button>
                )}
              </span>
            ) : (
              <SpeakButton text={exercise.spokenPrompt} compact />
            )}
          </div>

          {exercise.question && <p className={exercise.question.length > 60 ? "exercise-question exercise-problem" : "exercise-question"}>{exercise.question}</p>}

          {exercise.picture && (
            <ExercisePicture
              picture={exercise.picture}
              onPickTick={exercise.input.kind === "numberLine" ? (n) => submit({ kind: "number", value: n }) : undefined}
              picked={answered?.given.kind === "number" ? answered.given.value : null}
              expected={answered && expected?.kind === "number" ? expected.value : null}
              disabled={disabled}
            />
          )}

          <form className="equation-form" onSubmit={onSubmitForm} key={`form-${index}`}>
            <Statement tokens={exercise.statement} renderSlot={renderSlot} label={exercise.prompt} />
            {/* Enter validates the typed answer. */}
            <button type="submit" hidden tabIndex={-1} aria-hidden="true" />
            {typing && answerMode === "Keyboard" && (
              <NumberPad
                value={value(active)}
                onChange={(text) => setValue(active, text)}
                onSubmit={() => (value(active) !== "" ? (active + 1 < slots && value(active + 1) === "" ? setActive(active + 1) : submitTyped()) : undefined)}
                disabled={check.isPending}
                maxLength={MAX_ANSWER_LENGTH}
              />
            )}
          </form>

          {exercise.input.kind === "spelling" && (
            <SpellingAnswer
              key={`spelling-${index}`}
              before={word?.before ?? null}
              after={word?.after ?? null}
              missing={word?.missing ?? null}
              article={word?.article ?? null}
              maxLength={exercise.input.maxLength}
              disabled={disabled}
              onSubmit={(text) => submit({ kind: "text", value: text })}
              given={answered ? (given ?? "") : null}
              correct={answered ? answered.outcome.correct : null}
            />
          )}

          {exercise.input.kind === "pick" && blank && word && (
            <p className="spelling-word spelling-word-blank" aria-label={exercise.prompt}>
              {word.article && <span className="spelling-article">{word.article}</span>}
              <span className="spelling-around">{word.before}</span>
              <span className={answered ? `spelling-given ${answered.outcome.correct ? "slot-right" : "slot-wrong"}` : "spelling-hole"}>
                {answered ? (given === "" ? fr.spelling.nothing : given) : "_".repeat(word.missing ?? 1)}
              </span>
              <span className="spelling-around">{word.after}</span>
            </p>
          )}

          {exercise.input.kind === "pick" && exercise.choices && (
            <ChoiceGrid
              labels={choiceLabels}
              onPick={(i) => submit(exercise.choices![i].answer)}
              picked={pickedChoice}
              expected={expectedChoice}
              disabled={disabled}
              label={fr.series.choices}
            />
          )}

          {exercise.input.kind === "comparison" && (
            <SignButtons
              key={`SignButtons-${index}`}
              onPick={(sign) => submit({ kind: "comparison", sign })}
              disabled={disabled}
              picked={answered?.given.kind === "comparison" ? answered.given.sign : null}
              expected={expected?.kind === "comparison" ? expected.sign : null}
            />
          )}

          {exercise.input.kind === "yesNo" && (
            <YesNoButtons
              key={`YesNoButtons-${index}`}
              onPick={(v) => submit({ kind: "yesNo", value: v })}
              disabled={disabled}
              picked={answered?.given.kind === "yesNo" ? answered.given.value : null}
              expected={expected?.kind === "yesNo" ? expected.value : null}
            />
          )}

          {exercise.input.kind === "order" && (
            <OrderInput
              key={`OrderInput-${index}`}
              items={exercise.input.items}
              ascending={exercise.input.ascending}
              onSubmit={(order) => submit({ kind: "sequence", values: order })}
              disabled={disabled}
              correct={answered ? answered.outcome.correct : null}
            />
          )}

          {exercise.input.kind === "multiChoice" && (
            <MultiChoiceInput
              key={`MultiChoiceInput-${index}`}
              items={exercise.input.items}
              onSubmit={(picked) => submit({ kind: "set", values: picked })}
              disabled={disabled}
              expected={expected?.kind === "set" ? expected.values : null}
            />
          )}

          {exercise.input.kind === "numberLine" && phase.kind === "answering" && <p className="exercise-hint">{fr.series.numberLine}</p>}

          <AnimatePresence mode="wait">
            {answered && (
              <motion.div key={`${index}-${answered.outcome.correct}`} className="answer-feedback" exit={{ opacity: 0 }}>
                {answered.outcome.correct ? (
                  <Feedback kind="correct" message={fr.series.correct[index % fr.series.correct.length]} />
                ) : (
                  <>
                    <Feedback
                      kind="incorrect"
                      message={answered.given.kind === "timedOut" ? fr.placement.timeUp : fr.series.incorrect}
                      detail={answered.outcome.explanation}
                    />
                    <SpeakButton text={answered.outcome.explanation} label={fr.series.listenExplanation} compact />
                    {answered.outcome.correction && <WordCorrection correction={answered.outcome.correction} />}
                    {answered.outcome.help && (
                      <figure className="answer-help">
                        <figcaption>{fr.series.help}</figcaption>
                        <ExercisePicture picture={answered.outcome.help} />
                      </figure>
                    )}
                    {!autoNext && (
                      <button type="button" className="button button-primary" onClick={() => goNext(answered.outcome)} autoFocus>
                        {fr.series.next}
                      </button>
                    )}
                  </>
                )}
              </motion.div>
            )}
          </AnimatePresence>
          {check.isError && <p className="message">{fr.app.error}</p>}
        </section>
      )}
    </main>
  );
}

export function Summary({ summary, onAgain, onMenu, againLabel = fr.series.again }: { summary: SeriesSummary; onAgain(): void; onMenu(): void; againLabel?: string }) {
  const perfect = summary.exerciseCount > 0 && summary.correctCount === summary.exerciseCount;
  return (
    <motion.section className="summary" role="status" initial={{ opacity: 0, y: 12 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.3 }}>
      <h1 className="title">{fr.series.finished}</h1>
      <Mascot situation={summary.mascotSituation} />
      <p className="summary-score">{fr.series.score(summary.correctCount, summary.exerciseCount)}</p>
      {perfect && <Fireworks label={fr.series.allRight} />}
      {perfect && <p className="summary-perfect">{fr.series.allRight}</p>}
      <motion.p
        className="summary-points"
        initial={{ scale: 0.6, opacity: 0 }}
        animate={{ scale: 1, opacity: 1 }}
        transition={{ delay: 0.25, type: "spring", stiffness: 260, damping: 14 }}
      >
        {fr.series.pointsEarned(summary.pointsEarned)}
      </motion.p>
      {summary.pointsPercent > 100 && <p className="summary-perfect summary-bonus">{fr.series.aheadBonus(summary.pointsPercent)}</p>}
      <RewardsPanel rewards={summary.rewards} />
      <div className="actions">
        <button type="button" className="button" onClick={onMenu}>
          {fr.series.menu}
        </button>
        <button type="button" className="button button-primary" onClick={onAgain} autoFocus>
          {againLabel}
        </button>
      </div>
    </motion.section>
  );
}
