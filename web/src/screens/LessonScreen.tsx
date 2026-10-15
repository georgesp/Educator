import { useQuery, useQueryClient } from "@tanstack/react-query";
import { AnimatePresence, motion } from "motion/react";
import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router";
import type { Grade, Lesson, Profile, SeriesSummary } from "../api/client";
import { playSound } from "../audio/sounds";
import { stopSpeech, useVoiceWhile } from "../audio/useSpeech";
import { useApi } from "../app/BackendContext";
import { invalidateProgress, useSkills } from "../app/queries";
import { SUBJECT_SLUGS, subjectOfSlug } from "../app/subjects";

import { BigButton } from "../components/child/BigButton";
import { Mascot } from "../components/child/Mascot";
import { RewardsPanel } from "../components/child/Rewards";
import { RaceTrack } from "../components/child/RaceTrack";
import { SpeakButton } from "../components/child/SpeakButton";
import { BookIcon, PencilIcon } from "../components/icons";
import { LessonAnimation } from "../components/lesson/LessonAnimation";
import { LessonQuiz } from "../components/lesson/LessonQuiz";
import { useMove } from "../components/lesson/useSteps";
import { RequireProfile } from "../components/RequireProfile";
import { fr } from "../i18n/fr";
import "../components/lesson/lesson.css";

/**
 * A lesson (SUB001410): one slide at a time, then the quiz (SUB001411), then « Je m'entraîne ». A
 * lesson left in the middle starts again where the child stopped.
 */
export function LessonScreen() {
  return <RequireProfile>{(profile) => <LessonLoader profile={profile} />}</RequireProfile>;
}

function LessonLoader({ profile }: { profile: Profile }) {
  const api = useApi();
  const navigate = useNavigate();
  const params = useParams();
  const lessonId = params.lessonId ?? "";
  const back = `/lecons/${SUBJECT_SLUGS[subjectOfSlug(params.subject)]}`;
  const lesson = useQuery({ queryKey: ["lesson", lessonId, profile.id], queryFn: () => api.getLesson(lessonId, profile.id), staleTime: 0, gcTime: 0 });

  if (lesson.isError) {
    return (
      <main className="centered">
        <p className="message">{fr.app.error}</p>
        <button type="button" className="button" onClick={() => void navigate(back)}>
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

  return <LessonPlayer key={lesson.data.id} lesson={lesson.data} profile={profile} back={back} />;
}

type Phase =
  | { kind: "resume"; slide: number }
  | { kind: "slides"; index: number }
  | { kind: "quiz" }
  | { kind: "finished"; summary: SeriesSummary };

function LessonPlayer({ lesson, profile, back }: { lesson: Lesson; profile: Profile; back: string }) {
  const api = useApi();
  const queryClient = useQueryClient();
  const navigate = useNavigate();
  const move = useMove();
  const grade: Grade = profile.grade;
  const [phase, setPhase] = useState<Phase>(() =>
    lesson.resumeSlide !== null && lesson.resumeSlide > 0 ? { kind: "resume", slide: lesson.resumeSlide } : { kind: "slides", index: 0 },
  );
  // Interactive slides already done: going back to one does not block « Suivant » again.
  const [done, setDone] = useState<ReadonlySet<number>>(new Set());
  // The French lessons are read aloud.
  useVoiceWhile(lesson.subject === "Spelling");

  useEffect(() => () => stopSpeech(), []);

  // Where the child is, kept by the core: a lesson left in the middle starts again there.
  const slideIndex = phase.kind === "slides" ? phase.index : null;
  useEffect(() => {
    if (slideIndex !== null) {
      api.saveLessonProgress(profile.id, lesson.id, slideIndex).catch(() => undefined);
    }
  }, [api, profile.id, lesson.id, slideIndex]);

  const total = lesson.slides.length;
  const steps = phase.kind === "slides" ? phase.index : phase.kind === "resume" ? phase.slide : total;
  const progress = fr.lessons.progress(Math.min(steps + 1, total), total);

  const finish = (summary: SeriesSummary) => {
    setPhase({ kind: "finished", summary });
    api.saveLessonProgress(profile.id, lesson.id, null).catch(() => undefined);
    void invalidateProgress(queryClient);
    const perfect = summary.correctCount === summary.exerciseCount;
    playSound(perfect ? "celebration" : "finished");
  };

  // « Je m'entraîne »: the exercises of each skill of the lesson, the main one first (answer mode
  // chosen on the next screen). With several skills, each button names its skill.
  const skills = useSkills(grade, lesson.subject);
  const practice = (skillId: string) => void navigate(`/exercices/${SUBJECT_SLUGS[lesson.subject]}/${encodeURIComponent(skillId)}`);
  const practiceLabel = (skillId: string) => {
    const title = skills.data?.skills.find((s) => s.id === skillId)?.title;
    return lesson.skillIds.length > 1 && title ? fr.lessons.practiceSkill(title) : fr.lessons.practice;
  };

  return (
    <main className="series-screen lesson-screen">
      <header className="series-header">
        <button type="button" className="back-button" onClick={() => void navigate(back)}>
          {phase.kind === "finished" ? `← ${fr.common.back}` : fr.lessons.quit}
        </button>
        <span className="series-title">{lesson.title}</span>
        <span className="lesson-progress">
          {phase.kind === "slides" || phase.kind === "resume" ? progress : phase.kind === "quiz" ? fr.lessons.quizTitle : fr.lessons.finished}
        </span>
      </header>

      <RaceTrack done={steps} total={total} results={lesson.slides.map(() => null)} label={progress} />

      {phase.kind === "resume" && (
        <section className="summary lesson-resume" aria-labelledby="lesson-resume">
          <h2 id="lesson-resume" className="title">
            {fr.lessons.resumeTitle}
          </h2>
          <p className="message">{fr.lessons.resumeAsk}</p>
          <div className="lesson-summary-actions">
            <BigButton label={fr.lessons.resume} icon={<BookIcon />} variant="primary" autoFocus onClick={() => setPhase({ kind: "slides", index: phase.slide })} />
            <BigButton label={fr.lessons.restart} icon={<BookIcon />} onClick={() => setPhase({ kind: "slides", index: 0 })} />
          </div>
        </section>
      )}

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

      {phase.kind === "quiz" && <LessonQuiz lesson={lesson} profileId={profile.id} onFinished={finish} />}

      {phase.kind === "finished" && (
        <LessonSummary
          summary={phase.summary}
          skillIds={lesson.skillIds}
          practiceLabel={practiceLabel}
          onPractice={practice}
          onReview={() => {
            setDone(new Set());
            setPhase({ kind: "slides", index: 0 });
          }}
        />
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
          <SpeakButton text={slide.speech} label={fr.lessons.replay} />
        </div>
        {index === 0 && <Mascot situation="LessonStart" title={lesson.title} size={96} />}
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

type LessonSummaryProps = {
  summary: SeriesSummary;
  skillIds: string[];
  practiceLabel(skillId: string): string;
  onPractice(skillId: string): void;
  onReview(): void;
};

/** The end of the lesson: the score of the quiz, the points, then « Je m'entraîne » — or, below half
 * of right answers, « Revoir la leçon » first. */
function LessonSummary({ summary, skillIds, practiceLabel, onPractice, onReview }: LessonSummaryProps) {
  const review = summary.reviewLesson;
  const reviewButton = <BigButton key="review" label={fr.lessons.again} icon={<BookIcon />} variant={review ? "primary" : "default"} autoFocus={review} onClick={onReview} />;
  const practiceButtons = skillIds.map((skillId, i) => (
    <BigButton
      key={skillId}
      label={practiceLabel(skillId)}
      icon={<PencilIcon />}
      variant={i === 0 && !review ? "primary" : "default"}
      onClick={() => onPractice(skillId)}
      autoFocus={i === 0 && !review}
    />
  ));
  return (
    <section className="summary lesson-summary">
      <h2 className="title">{fr.lessons.finished}</h2>
      <Mascot situation={summary.mascotSituation} />
      <p className="summary-score">{fr.series.score(summary.correctCount, summary.exerciseCount)}</p>
      {summary.correctCount === summary.exerciseCount && <p className="summary-perfect">{fr.series.allRight}</p>}
      <p className="summary-points">{fr.series.pointsEarned(summary.pointsEarned)}</p>
      <RewardsPanel rewards={summary.rewards} />
      {review && <p className="notice">{fr.lessons.review}</p>}
      <div className="lesson-summary-actions">{review ? [reviewButton, ...practiceButtons] : [...practiceButtons, reviewButton]}</div>
    </section>
  );
}
