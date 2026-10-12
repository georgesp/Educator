import { useQuery } from "@tanstack/react-query";
import { AnimatePresence, motion } from "motion/react";
import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router";
import type { Lesson } from "../api/client";
import { playSound } from "../audio/sounds";
import { stopSpeech, useSpeech } from "../audio/useSpeech";
import { useApi } from "../app/BackendContext";
import { BigButton } from "../components/child/BigButton";
import { RaceTrack } from "../components/child/RaceTrack";
import { SpeakButton } from "../components/child/SpeakButton";
import { BookIcon, PencilIcon } from "../components/icons";
import { LessonAnimation } from "../components/lesson/LessonAnimation";
import { LessonQuiz } from "../components/lesson/LessonQuiz";
import { useMove } from "../components/lesson/useSteps";
import { RequireProfile } from "../components/RequireProfile";
import { fr } from "../i18n/fr";
import "../components/lesson/lesson.css";

/** A lesson (SUB001410): one slide at a time, then the quiz (SUB001411), then « Je m'entraîne ». */
export function LessonScreen() {
  return <RequireProfile>{() => <LessonLoader />}</RequireProfile>;
}

function LessonLoader() {
  const api = useApi();
  const navigate = useNavigate();
  const { lessonId = "" } = useParams();
  const lesson = useQuery({ queryKey: ["lesson", lessonId], queryFn: () => api.getLesson(lessonId), staleTime: Infinity });

  if (lesson.isError) {
    return (
      <main className="centered">
        <p className="message">{fr.app.error}</p>
        <button type="button" className="button" onClick={() => void navigate("/lecons")}>
          {fr.common.back}
        </button>
      </main>
    );
  }

  if (!lesson.data) {
    return (
      <main className="centered">
        <p className="message">{fr.app.loading}</p>
      </main>
    );
  }

  return <LessonPlayer key={lesson.data.id} lesson={lesson.data} />;
}

type Phase = { kind: "slides"; index: number } | { kind: "quiz" } | { kind: "finished"; correct: number };

function LessonPlayer({ lesson }: { lesson: Lesson }) {
  const navigate = useNavigate();
  const speak = useSpeech();
  const move = useMove();
  const [phase, setPhase] = useState<Phase>({ kind: "slides", index: 0 });
  // Interactive slides already done: going back to one does not block « Suivant » again.
  const [done, setDone] = useState<ReadonlySet<number>>(new Set());

  useEffect(() => () => stopSpeech(), []);

  const total = lesson.slides.length;
  const steps = phase.kind === "slides" ? phase.index : total;
  const progress = fr.lessons.progress(Math.min(steps + 1, total), total);

  const finish = (correct: number) => {
    setPhase({ kind: "finished", correct });
    playSound(correct === lesson.quiz.length ? "celebration" : "finished");
    void speak(fr.lessons.finishedSpoken(correct, lesson.quiz.length));
  };

  // « Je m'entraîne »: the exercises of the main skill (answer mode chosen on the next screen).
  const practice = () => void navigate(`/exercices/${encodeURIComponent(lesson.skillIds[0])}`);

  return (
    <main className="series-screen lesson-screen">
      <header className="series-header">
        <button type="button" className="back-button" onClick={() => void navigate("/lecons")}>
          {phase.kind === "finished" ? `← ${fr.common.back}` : fr.lessons.quit}
        </button>
        <span className="series-title">{lesson.title}</span>
        <span className="lesson-progress">{phase.kind === "slides" ? progress : phase.kind === "quiz" ? fr.lessons.quizTitle : fr.lessons.finished}</span>
      </header>

      <RaceTrack done={steps} total={total} results={lesson.slides.map(() => null)} label={progress} />

      {phase.kind === "slides" && (
        <SlideView
          lesson={lesson}
          index={phase.index}
          completed={done.has(phase.index)}
          onComplete={() => setDone((d) => new Set(d).add(phase.index))}
          onPrevious={() => setPhase({ kind: "slides", index: Math.max(phase.index - 1, 0) })}
          onNext={() => setPhase(phase.index + 1 < total ? { kind: "slides", index: phase.index + 1 } : { kind: "quiz" })}
          move={move}
        />
      )}

      {phase.kind === "quiz" && <LessonQuiz lesson={lesson} onFinished={finish} />}

      {phase.kind === "finished" && (
        <section className="summary lesson-summary">
          <h2 className="title">{fr.lessons.finished}</h2>
          <p className="summary-score">{fr.series.score(phase.correct, lesson.quiz.length)}</p>
          {phase.correct === lesson.quiz.length && <p className="summary-perfect">{fr.series.allRight}</p>}
          <div className="lesson-summary-actions">
            <BigButton label={fr.lessons.practice} icon={<PencilIcon />} variant="primary" onClick={practice} autoFocus />
            <BigButton
              label={fr.lessons.again}
              icon={<BookIcon />}
              onClick={() => {
                setDone(new Set());
                setPhase({ kind: "slides", index: 0 });
              }}
            />
          </div>
        </section>
      )}
    </main>
  );
}

type SlideViewProps = {
  lesson: Lesson;
  index: number;
  completed: boolean;
  onComplete(): void;
  onPrevious(): void;
  onNext(): void;
  move: { duration: number };
};

function SlideView({ lesson, index, completed, onComplete, onPrevious, onNext, move }: SlideViewProps) {
  const slide = lesson.slides[index];
  const last = index === lesson.slides.length - 1;
  const blocked = slide.interactive && !completed;

  return (
    <AnimatePresence mode="wait">
      <motion.section
        key={index}
        className="lesson-slide"
        aria-labelledby={`slide-${index}`}
        initial={{ opacity: 0, x: 40 }}
        animate={{ opacity: 1, x: 0 }}
        exit={{ opacity: 0, x: -40 }}
        transition={move}
      >
        <div className="lesson-slide-head">
          <h2 id={`slide-${index}`} className="lesson-slide-title">
            {slide.title}
          </h2>
          <SpeakButton text={slide.speech} autoPlay label={fr.lessons.replay} />
        </div>
        <p className="lesson-slide-text">{slide.text}</p>

        <div className="lesson-stage">
          <LessonAnimation animation={slide.animation} onComplete={onComplete} />
        </div>

        <nav className="lesson-nav" aria-label={fr.lessons.progress(index + 1, lesson.slides.length)}>
          <button type="button" className="button" onClick={onPrevious} disabled={index === 0}>
            ← {fr.lessons.previous}
          </button>
          {blocked && <span className="lesson-your-turn">{fr.lessons.yourTurn}</span>}
          <button type="button" className="button button-primary" onClick={onNext} disabled={blocked}>
            {last ? fr.lessons.toQuiz : `${fr.lessons.next} →`}
          </button>
        </nav>
      </motion.section>
    </AnimatePresence>
  );
}
