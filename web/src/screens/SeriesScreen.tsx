import { useMutation, useQueryClient } from "@tanstack/react-query";
import { AnimatePresence, motion } from "motion/react";
import { useEffect, useRef, useState, type FormEvent, type ReactNode } from "react";
import { useNavigate, useParams } from "react-router";
import type { AnswerMode, Exercise, Profile, Series, SeriesAnswer, SeriesSummary } from "../api/client";
import { playSound } from "../audio/sounds";
import { stopSpeech, useSpeech } from "../audio/useSpeech";
import { useApi } from "../app/BackendContext";
import { queryKeys } from "../app/queries";
import { ChoiceGrid } from "../components/child/ChoiceGrid";
import { Feedback } from "../components/child/Feedback";
import { Fireworks } from "../components/child/Fireworks";
import { NumberPad } from "../components/child/NumberPad";
import { PointsBadge } from "../components/child/PointsBadge";
import { RaceTrack } from "../components/child/RaceTrack";
import { SpeakButton } from "../components/child/SpeakButton";
import { RequireProfile } from "../components/RequireProfile";
import { fr } from "../i18n/fr";

/** Pause on a right answer before the next exercise comes by itself. */
export const AUTO_NEXT_DELAY_MS = 900;

const MAX_ANSWER_LENGTH = 10;

/** The host reads answers as 32-bit integers: a longer number is simply a wrong answer. */
const toAnswer = (text: string) => Math.min(Number.parseInt(text, 10), 2_147_483_647);

/** A series of 10 exercises (SUB000109, SUB000110, SUB000111). */
export function SeriesScreen() {
  return <RequireProfile>{(profile) => <SeriesRun profile={profile} />}</RequireProfile>;
}

type Phase =
  | { kind: "answering" }
  | { kind: "answered"; outcome: SeriesAnswer; answer: number }
  | { kind: "finished"; summary: SeriesSummary };

function SeriesRun({ profile }: { profile: Profile }) {
  const api = useApi();
  const queryClient = useQueryClient();
  const navigate = useNavigate();
  const speak = useSpeech();
  const { skillId = "", mode = "Keyboard" } = useParams();
  const answerMode: AnswerMode = mode === "Choice" ? "Choice" : "Keyboard";

  const [attempt, setAttempt] = useState(0);
  const [series, setSeries] = useState<Series | null>(null);
  const [index, setIndex] = useState(0);
  const [answer, setAnswer] = useState("");
  const [results, setResults] = useState<(boolean | null)[]>([]);
  const [phase, setPhase] = useState<Phase>({ kind: "answering" });
  const [confirmQuit, setConfirmQuit] = useState(false);
  const [points, setPoints] = useState(profile.points);
  const started = useRef(-1);
  const inputRef = useRef<HTMLInputElement>(null);

  const start = useMutation({
    mutationFn: () => api.startSeries(profile.id, skillId, answerMode),
    onSuccess: (created) => {
      setSeries(created);
      setIndex(0);
      setAnswer("");
      setResults(created.exercises.map(() => null));
      setPhase({ kind: "answering" });
      void speak(`${created.instruction} ${created.exercises[0]?.spokenPrompt ?? ""}`);
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

  const finish = (summary: SeriesSummary) => {
    setPoints(summary.profilePoints);
    setPhase({ kind: "finished", summary });
    // Everything right: fireworks, a fanfare and « Bravo, tout est juste ! » (SUB000114).
    const perfect = summary.correctCount === summary.exerciseCount;
    playSound(perfect ? "celebration" : "finished");
    const spoken = fr.series.finishedSpoken(summary.correctCount, summary.exerciseCount, summary.pointsEarned);
    void speak(perfect ? `${fr.series.allRight} ${spoken}` : spoken);
    void queryClient.invalidateQueries({ queryKey: queryKeys.profiles });
  };

  const goNext = (outcome: SeriesAnswer) => {
    if (outcome.summary) {
      finish(outcome.summary);
      return;
    }

    setIndex((i) => i + 1);
    setAnswer("");
    setPhase({ kind: "answering" });
  };

  const check = useMutation({
    mutationFn: (value: number) => api.answerSeries(series!.seriesId, index, value),
    onSuccess: (outcome, value) => {
      setResults((current) => current.map((r, i) => (i === index ? outcome.correct : r)));
      setPhase({ kind: "answered", outcome, answer: value });
      if (!outcome.correct) {
        void speak(outcome.explanation);
      }
    },
  });

  // A right answer moves on by itself; after a mistake the child reads the correction and clicks « Suivant ».
  useEffect(() => {
    if (phase.kind !== "answered" || !phase.outcome.correct) {
      return;
    }

    const timer = window.setTimeout(() => goNext(phase.outcome), AUTO_NEXT_DELAY_MS);
    return () => window.clearTimeout(timer);
    // goNext only uses state setters: the phase alone decides.
  }, [phase]);

  useEffect(() => {
    if (phase.kind === "answering") {
      inputRef.current?.focus();
    }
  }, [phase, index]);

  const abandon = useMutation({
    mutationFn: () => api.abandonSeries(series!.seriesId),
    onSettled: async () => {
      await queryClient.invalidateQueries({ queryKey: queryKeys.profiles });
      await navigate("/menu");
    },
  });

  const submit = (value: number) => {
    if (phase.kind === "answering" && !check.isPending) {
      check.mutate(value);
    }
  };

  const onSubmitForm = (event: FormEvent) => {
    event.preventDefault();
    if (answer !== "") {
      submit(toAnswer(answer));
    }
  };

  if (start.isError) {
    return (
      <main className="centered">
        <p className="message">{fr.app.error}</p>
        <button type="button" className="button" onClick={() => void navigate("/exercices")}>
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

  const exercise = series.exercises[index];
  const total = series.exercises.length;
  const answeredCount = results.filter((r) => r !== null).length;

  return (
    <main className="series-screen">
      <header className="series-header">
        <button type="button" className="back-button" onClick={() => (phase.kind === "finished" ? void navigate("/menu") : setConfirmQuit(true))}>
          {phase.kind === "finished" ? `← ${fr.series.menu}` : fr.series.quit}
        </button>
        <span className="series-title">{fr.series.progress(Math.min(index + 1, total), total)}</span>
        <PointsBadge points={points} />
      </header>

      <RaceTrack done={answeredCount} total={total} results={results} label={fr.series.progress(Math.min(index + 1, total), total)} />

      {confirmQuit && phase.kind !== "finished" && (
        <div className="quit-confirm" role="alertdialog" aria-labelledby="quit-question">
          <p id="quit-question">{fr.series.quitQuestion}</p>
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
        <Summary
          summary={phase.summary}
          onAgain={() => {
            setSeries(null);
            setAttempt((a) => a + 1);
          }}
          onMenu={() => void navigate("/menu")}
        />
      ) : (
        <section className="exercise-card" aria-live="polite">
          <div className="exercise-instruction">
            <span>{series.instruction}</span>
            <SpeakButton text={exercise.spokenPrompt} compact />
          </div>

          <form className="equation-form" onSubmit={onSubmitForm}>
            <Equation
              exercise={exercise}
              hiddenSlot={
                phase.kind === "answered" ? (
                  <span className={`equation-slot ${phase.outcome.correct ? "slot-right" : "slot-wrong"}`}>{phase.answer}</span>
                ) : answerMode === "Keyboard" ? (
                  <input
                    ref={inputRef}
                    className="answer"
                    aria-label={fr.series.answerLabel}
                    inputMode="numeric"
                    autoComplete="off"
                    maxLength={MAX_ANSWER_LENGTH}
                    value={answer}
                    onChange={(e) => setAnswer(e.target.value.replace(/\D/g, "").slice(0, MAX_ANSWER_LENGTH))}
                  />
                ) : (
                  <span className="equation-slot">?</span>
                )
              }
            />

            {phase.kind === "answering" && answerMode === "Keyboard" && (
              <NumberPad value={answer} onChange={setAnswer} onSubmit={() => answer !== "" && submit(toAnswer(answer))} disabled={check.isPending} maxLength={MAX_ANSWER_LENGTH} />
            )}
          </form>

          {answerMode === "Choice" && exercise.choices && (
            <ChoiceGrid
              choices={exercise.choices}
              onPick={submit}
              picked={phase.kind === "answered" ? phase.answer : null}
              expected={phase.kind === "answered" ? phase.outcome.expected : null}
              disabled={phase.kind !== "answering" || check.isPending}
              label={fr.series.choices}
            />
          )}

          <AnimatePresence mode="wait">
            {phase.kind === "answered" && (
              <motion.div key={`${index}-${phase.outcome.correct}`} className="answer-feedback" exit={{ opacity: 0 }}>
                {phase.outcome.correct ? (
                  <Feedback kind="correct" message={fr.series.correct[index % fr.series.correct.length]} />
                ) : (
                  <>
                    <Feedback kind="incorrect" message={fr.series.incorrect} detail={phase.outcome.explanation} />
                    <button type="button" className="button button-primary" onClick={() => goNext(phase.outcome)} autoFocus>
                      {fr.series.next}
                    </button>
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

function Equation({ exercise, hiddenSlot }: { exercise: Exercise; hiddenSlot: ReactNode }) {
  const part = (value: number | null) => (value === null ? hiddenSlot : <span>{value}</span>);
  return (
    <div className="equation" aria-label={exercise.prompt}>
      {part(exercise.left)}
      <span className="equation-sign">{exercise.operator}</span>
      {part(exercise.right)}
      <span className="equation-sign">=</span>
      {part(exercise.result)}
    </div>
  );
}

function Summary({ summary, onAgain, onMenu }: { summary: SeriesSummary; onAgain(): void; onMenu(): void }) {
  const perfect = summary.correctCount === summary.exerciseCount;
  return (
    <motion.section className="summary" role="status" initial={{ opacity: 0, y: 12 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.3 }}>
      <h1 className="title">{fr.series.finished}</h1>
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
      <div className="actions">
        <button type="button" className="button" onClick={onMenu}>
          {fr.series.menu}
        </button>
        <button type="button" className="button button-primary" onClick={onAgain} autoFocus>
          {fr.series.again}
        </button>
      </div>
    </motion.section>
  );
}
