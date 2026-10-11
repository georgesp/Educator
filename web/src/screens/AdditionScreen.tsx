import { useMutation } from "@tanstack/react-query";
import { AnimatePresence, motion } from "motion/react";
import { useEffect, useRef, useState, type FormEvent } from "react";
import { Link } from "react-router";
import type { AdditionExercise, AnswerResponse } from "../api/client";
import { useApi } from "../app/BackendContext";
import { fr } from "../i18n/fr";

/**
 * Lot 0 proof of concept (SUB006007): the host generates an addition, the child types the answer,
 * the host checks it; the "Écouter" button reads the statement with Piper.
 * The real exercise screens arrive with lot 1.
 */
export function AdditionScreen() {
  const api = useApi();
  const [exercise, setExercise] = useState<AdditionExercise | null>(null);
  const [answer, setAnswer] = useState("");
  const [result, setResult] = useState<AnswerResponse | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  const create = useMutation({
    mutationFn: () => api.createAddition(),
    onSuccess: (created) => {
      setExercise(created);
      setAnswer("");
      setResult(null);
    },
  });
  const check = useMutation({
    mutationFn: (value: number) => api.answerAddition(exercise!.id, value),
    onSuccess: setResult,
  });
  const listen = useMutation({
    mutationFn: async () => playWav(await api.readAddition(exercise!.id)),
  });

  const { mutate: createExercise } = create;
  useEffect(() => createExercise(), [createExercise]);
  useEffect(() => {
    if (exercise && !result) {
      inputRef.current?.focus();
    }
  }, [exercise, result]);

  const submit = (event: FormEvent) => {
    event.preventDefault();
    const value = Number.parseInt(answer, 10);
    if (!Number.isNaN(value) && !result) {
      check.mutate(value);
    }
  };

  const failed = create.isError || check.isError;

  return (
    <main className="centered">
      <Link className="back" to="/">
        ← {fr.addition.back}
      </Link>
      <h1 className="title">{fr.addition.title}</h1>

      {!exercise ? (
        <p className="message">{create.isError ? fr.addition.error : fr.addition.loading}</p>
      ) : (
        <form className="exercise" onSubmit={submit}>
          <div className="statement" aria-live="polite">
            <span>{exercise.statement}</span>
            <span>=</span>
            <input
              ref={inputRef}
              className="answer"
              aria-label={fr.addition.answerLabel}
              inputMode="numeric"
              autoComplete="off"
              maxLength={3}
              value={answer}
              disabled={result !== null}
              onChange={(e) => setAnswer(e.target.value.replace(/\D/g, ""))}
            />
          </div>

          <div className="actions">
            <button type="button" className="button" onClick={() => listen.mutate()} disabled={listen.isPending}>
              🔊 {fr.addition.listen}
            </button>
            {result ? (
              <button type="button" className="button button-primary" onClick={() => create.mutate()}>
                {fr.addition.next}
              </button>
            ) : (
              <button type="submit" className="button button-primary" disabled={answer === "" || check.isPending}>
                {fr.addition.check}
              </button>
            )}
          </div>

          <AnimatePresence>
            {result && (
              <motion.p
                key={result.isCorrect ? "correct" : "incorrect"}
                role="status"
                className={result.isCorrect ? "feedback feedback-correct" : "feedback feedback-incorrect"}
                initial={{ opacity: 0, scale: 0.8 }}
                animate={{ opacity: 1, scale: 1 }}
                exit={{ opacity: 0 }}
              >
                {result.isCorrect ? fr.addition.correct : fr.addition.incorrect(result.expectedAnswer)}
              </motion.p>
            )}
          </AnimatePresence>
          {failed && exercise && <p className="message">{fr.addition.error}</p>}
        </form>
      )}
    </main>
  );
}

async function playWav(wav: Blob): Promise<void> {
  const url = URL.createObjectURL(wav);
  const audio = new Audio(url);
  audio.addEventListener("ended", () => URL.revokeObjectURL(url), { once: true });
  try {
    await audio.play();
  } catch {
    URL.revokeObjectURL(url);
  }
}
