import { useMutation } from "@tanstack/react-query";
import { useEffect, useRef, useState, type FormEvent } from "react";
import type { AnswerPayload, Lesson, QuizAnswer } from "../../api/client";
import { useSpeech } from "../../audio/useSpeech";
import { useApi } from "../../app/BackendContext";
import { fr } from "../../i18n/fr";
import { ChoiceGrid } from "../child/ChoiceGrid";
import { Feedback } from "../child/Feedback";
import { NumberPad } from "../child/NumberPad";
import { SpeakButton } from "../child/SpeakButton";

const MAX_ANSWER_LENGTH = 6;

/** Same answer: same kind and same values. */
const sameAnswer = (a: AnswerPayload, b: AnswerPayload) => JSON.stringify(a) === JSON.stringify(b);

type LessonQuizProps = {
  lesson: Lesson;
  /** Number of right answers, once the last question is answered and its explanation read. */
  onFinished(correct: number): void;
};

/**
 * End-of-lesson quiz (SUB001411): 3 to 5 questions, typed or picked. The core checks each answer;
 * the explanation is shown and read after every answer, right or wrong.
 */
export function LessonQuiz({ lesson, onFinished }: LessonQuizProps) {
  const api = useApi();
  const speak = useSpeech();
  const [index, setIndex] = useState(0);
  const [value, setValue] = useState("");
  const [answered, setAnswered] = useState<{ outcome: QuizAnswer; given: AnswerPayload } | null>(null);
  const [correct, setCorrect] = useState(0);
  const input = useRef<HTMLInputElement | null>(null);
  const question = lesson.quiz[index];
  const last = index === lesson.quiz.length - 1;

  const check = useMutation({
    mutationFn: (given: AnswerPayload) => api.answerLessonQuiz(lesson.id, index, given),
    onSuccess: (outcome, given) => {
      setAnswered({ outcome, given });
      if (outcome.correct) {
        setCorrect((c) => c + 1);
      }
      void speak(outcome.explanation);
    },
  });

  useEffect(() => {
    if (!answered && !question.choices) {
      input.current?.focus();
    }
  }, [answered, question]);

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
      onFinished(correct);
      return;
    }

    setIndex((i) => i + 1);
    setValue("");
    setAnswered(null);
  };

  const choices = question.choices;
  const picked = answered && choices ? choices.findIndex((c) => sameAnswer(c.answer, answered.given)) : null;
  const expected = answered && choices ? choices.findIndex((c) => sameAnswer(c.answer, answered.outcome.expected)) : null;
  const praise = fr.series.correct[index % fr.series.correct.length];

  return (
    <section className="exercise-card lesson-quiz" aria-live="polite">
      <div className="exercise-instruction">
        <span>{fr.lessons.quizProgress(index + 1, lesson.quiz.length)}</span>
        <SpeakButton key={index} text={question.spoken} autoPlay compact />
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
          <button type="button" className="button button-primary" onClick={next} autoFocus>
            {fr.lessons.next}
          </button>
        </div>
      )}

      {check.isError && <p className="message">{fr.app.error}</p>}
    </section>
  );
}
