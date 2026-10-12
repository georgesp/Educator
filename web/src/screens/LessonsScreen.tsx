import { useQuery } from "@tanstack/react-query";
import { useNavigate } from "react-router";
import type { Grade, LessonSummary, Profile } from "../api/client";
import { useApi } from "../app/BackendContext";
import { BigButton } from "../components/child/BigButton";
import { SpeakButton } from "../components/child/SpeakButton";
import { BookIcon } from "../components/icons";
import { RequireProfile } from "../components/RequireProfile";
import { fr } from "../i18n/fr";
import "../components/lesson/lesson.css";

export const lessonsQueryKey = (grade: Grade) => ["lessons", grade] as const;

/** Lessons offered to a grade (the core falls back to CE1 for the grades without lessons yet). */
export function useLessons(grade: Grade) {
  const api = useApi();
  return useQuery({ queryKey: lessonsQueryKey(grade), queryFn: () => api.listLessons(grade) });
}

/** Lessons → Mathematics (SUB001410): the lessons of the profile's grade, the current period first. */
export function LessonsScreen() {
  return <RequireProfile>{(profile) => <LessonList profile={profile} />}</RequireProfile>;
}

function LessonList({ profile }: { profile: Profile }) {
  const navigate = useNavigate();
  const lessons = useLessons(profile.grade);
  const data = lessons.data;
  const usesFallback = data !== undefined && data.lessonsGrade !== data.grade;
  const current = data?.lessons.filter((l) => l.isCurrentPeriod) ?? [];
  const others = data?.lessons.filter((l) => !l.isCurrentPeriod) ?? [];
  const open = (lesson: LessonSummary) => void navigate(`/lecons/${encodeURIComponent(lesson.id)}`);

  return (
    <main className="screen">
      <header className="screen-header">
        <button type="button" className="back-button" onClick={() => void navigate("/menu")}>
          ← {fr.series.menu}
        </button>
        <h1 className="title">{fr.lessons.title}</h1>
        <SpeakButton text={usesFallback ? `${fr.lessons.fallback} ${fr.lessons.spoken}` : fr.lessons.spoken} autoPlay />
      </header>

      {lessons.isError && <p className="message">{fr.app.error}</p>}
      {usesFallback && <p className="notice">{fr.lessons.fallback}</p>}
      {lessons.isLoading && <p className="message">{fr.app.loading}</p>}
      {data && data.lessons.length === 0 && <p className="notice">{fr.lessons.none}</p>}

      {current.length > 0 && (
        <section className="skill-section skill-section-current" aria-labelledby="lessons-current">
          <h2 id="lessons-current" className="section-title">
            {fr.lessons.current}
          </h2>
          <div className="skill-list">
            {current.map((lesson) => (
              <BigButton
                key={lesson.id}
                label={lesson.title}
                hint={fr.lessons.hint(lesson.period, lesson.durationMinutes)}
                icon={<BookIcon />}
                variant="primary"
                onClick={() => open(lesson)}
              />
            ))}
          </div>
        </section>
      )}

      {others.length > 0 && (
        <section className="skill-section" aria-labelledby="lessons-others">
          <h2 id="lessons-others" className="section-title">
            {fr.lessons.others}
          </h2>
          <div className="skill-list skill-list-compact">
            {others.map((lesson) => (
              <BigButton key={lesson.id} label={lesson.title} hint={fr.lessons.hint(lesson.period, lesson.durationMinutes)} icon={<BookIcon />} onClick={() => open(lesson)} />
            ))}
          </div>
        </section>
      )}
    </main>
  );
}
