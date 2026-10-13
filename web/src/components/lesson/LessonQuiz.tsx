import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useEffect, useRef, useState, type FormEvent } from "react";
import type { AnswerPayload, Lesson, Series, SeriesAnswer, SeriesSummary } from "../../api/client";
import { playVoice, useSpeech } from "../../audio/useSpeech";
import { useApi } from "../../app/BackendContext";
import { queryKeys } from "../../app/queries";
import { fr } from "../../i18n/fr";
import { ChoiceGrid } from "../child/ChoiceGrid";
import { Feedback } from "../child/Feedback";
import { NumberPad } from "../child/NumberPad";
import { SpeakButton } from "../child/SpeakButton";
import { SpellingAnswer } from "../spelling/SpellingAnswer";
import { WordCorrection } from "../spelling/WordCorrection";

const MAX_ANSWER_LENGTH = 6;

/** Same answer: same kind and same values. */
const sameAnswer = (a: AnswerPayload, b: AnswerPayload) => JSON.stringify(a) === JSON.stringify(b);

type LessonQuizProps = {
  lesson: Lesson;
  profileId: string;
  /** The summary of the quiz (right answers, points), once the last explanation is read. */
  onFinished(summary: SeriesSummary): void;
};

/**
 * End-of-lesson quiz (SUB001411): 3 to 5 questions, typed (a number, a word) or picked. The quiz is
 * a series recorded by the core (session `LessonQuiz`, points); the explanation is shown and read
 * after every answer, right or wrong.
 */
export function LessonQuiz({ lesson, profileId, onFinished }: LessonQuizProps) {
  const api = useApi();
  const speak = useSpeech();
  const [quiz, setQuiz] = useState<Series | null>(null);
  const [index, setIndex] = useState(0);
  const [value, setValue] = useState("");
  const [answered, setAnswered] = useState<{ outcome: SeriesAnswer; given: AnswerPayload } | null>(null);
  const input = useRef<HTMLInputElement | null>(null);
  const shownAt = useRef(Date.now());
  const started = useRef(false);
  const queryClient = useQueryClient();
  // The quiz in progress, recorded as abandoned when the child leaves before the last answer
  // (SUB001401): the right answers already given count, the masteries take them into account.
  const open = useRef<string | null>(null);
  useEffect(
    () => () => {
      const seriesId = open.current;
      if (seriesId) {
        open.current = null;
        void api
          .abandonSeries(seriesId)
          .then(() => queryClient.invalidateQueries({ queryKey: queryKeys.profiles }))
          .catch(() => undefined);
      }
    },
    [api, queryClient],
  );

  const start = useMutation({
    mutationFn: () => api.startLessonQuiz(profileId, lesson.id),
    onSuccess: (created) => {
      setQuiz(created);
      open.current = created.seriesId;
      shownAt.current = Date.now();
    },
  });
  const { mutate: startQuiz } = start;
  useEffect(() => {
    // Once, even when React runs the effect twice (StrictMode).
    if (!started.current) {
      started.current = true;
      startQuiz();
    }
  }, [startQuiz]);

  const question = quiz?.exercises[index];
  const last = quiz !== null && index === quiz.exercises.length - 1;

  const check = useMutation({
    mutationFn: (given: AnswerPayload) => api.answerSeries(quiz!.seriesId, index, given, Math.max(0, Math.round(Date.now() - shownAt.current))),
    onSuccess: (outcome, given) => {
      if (outcome.summary) {
        // The last answer closed the quiz.
        open.current = null;
      }
      setAnswered({ outcome, given });
      void speak(outcome.explanation);
    },
  });

  useEffect(() => {
    if (!answered && question?.input.kind === "number") {
      input.current?.focus();
    }
  }, [answered, question]);

  if (start.isError || check.isError) {
    return <p className="message">{fr.app.error}</p>;
  }

  if (!quiz || !question) {
    return <p className="message">{fr.app.loading}</p>;
  }

  const submit = (given: AnswerPayload) => {
    if (!answered && !check.isPending) {
      check.mutate(given);
    }
  };

  const submitTyped = () => {
    if (value !== "") {
      submit({ kind: "number", value: Number.parseInt(value, 10) });
    }
  };

  const onSubmitForm = (event: FormEvent) => {
    event.preventDefault();
    submitTyped();
  };

  const next = () => {
    if (last) {
      if (answered?.outcome.summary) {
        onFinished(answered.outcome.summary);
      }
      return;
    }

    setIndex((i) => i + 1);
    setValue("");
    setAnswered(null);
    shownAt.current = Date.now();
  };

  const choices = question.choices;
  const picked = answered && choices ? choices.findIndex((c) => sameAnswer(c.answer, answered.given)) : null;
  const expected = answered && choices ? choices.findIndex((c) => sameAnswer(c.answer, answered.outcome.expected)) : null;
  const praise = fr.series.correct[index % fr.series.correct.length];
  const given = answered?.given.kind === "text" ? answered.given.value : null;

  return (
    <section className="exercise-card lesson-quiz" aria-live="polite">
      <div className="exercise-instruction">
        <span>{fr.lessons.quizProgress(index + 1, quiz.exercises.length)}</span>
        <SpeakButton key={index} text={question.spokenPrompt} autoPlay compact />
        {question.word && (
          <button type="button" className="button" onClick={() => void playVoice(() => api.speakSeriesWord(quiz.seriesId, index))}>
            {fr.spelling.listenWord}
          </button>
        )}
      </div>

      <p className="lesson-quiz-question">{question.question}</p>

      {choices ? (
        <ChoiceGrid
          labels={choices.map((c) => c.label)}
          onPick={(i) => submit(choices[i].answer)}
          picked={picked}
          expected={expected}
          disabled={answered !== null || check.isPending}
          label={fr.series.choices}
        />
      ) : question.input.kind === "spelling" ? (
        <SpellingAnswer
          key={`quiz-${index}`}
          before={null}
          after={null}
          maxLength={question.input.maxLength}
          disabled={check.isPending}
          onSubmit={(text) => submit({ kind: "text", value: text })}
          given={answered ? (given ?? "") : null}
          correct={answered ? answered.outcome.correct : null}
        />
      ) : (
        <form className="equation-form" onSubmit={onSubmitForm} key={`quiz-${index}`}>
          <input
            ref={input}
            className={answered ? `answer ${answered.outcome.correct ? "slot-right" : "slot-wrong"}` : "answer"}
            aria-label={fr.series.answerLabel}
            inputMode="numeric"
            autoComplete="off"
            maxLength={MAX_ANSWER_LENGTH}
            value={value}
            readOnly={answered !== null}
            onChange={(e) => setValue(e.target.value.replace(/\D/g, "").slice(0, MAX_ANSWER_LENGTH))}
          />
          {!answered && (
            <NumberPad value={value} onChange={setValue} onSubmit={submitTyped} disabled={check.isPending} maxLength={MAX_ANSWER_LENGTH} />
          )}
        </form>
      )}

      {answered && (
        <div className="answer-feedback">
          <Feedback
            kind={answered.outcome.correct ? "correct" : "incorrect"}
            message={answered.outcome.correct ? praise : fr.lessons.notQuite}
            detail={answered.outcome.explanation}
          />
          {answered.outcome.correction && !answered.outcome.correct && <WordCorrection correction={answered.outcome.correction} />}
          <button type="button" className="button button-primary" onClick={next} autoFocus>
            {fr.lessons.next}
          </button>
        </div>
      )}
    </section>
  );
}
