import { useQuery } from "@tanstack/react-query";
import { useNavigate, useParams } from "react-router";
import type { Grade, LessonSummary, Profile, Subject } from "../api/client";
import { useApi } from "../app/BackendContext";
import { SUBJECT_SLUGS, subjectOfSlug } from "../app/subjects";
import { BigButton } from "../components/child/BigButton";
import { FamilyFilter, useFamilyFilter } from "../components/child/Family";
import { SpeakButton } from "../components/child/SpeakButton";
import { RequireProfile } from "../components/RequireProfile";
import { fr } from "../i18n/fr";
import "../components/lesson/lesson.css";

export const lessonsQueryKey = (grade: Grade, subject: Subject) => ["lessons", grade, subject] as const;

/** Lessons of a subject offered to a grade (the core falls back to CE1 for the grades without lessons yet). */
export function useLessons(grade: Grade, subject: Subject) {
  const api = useApi();
  return useQuery({ queryKey: lessonsQueryKey(grade, subject), queryFn: () => api.listLessons(grade, subject) });
}

/**
 * Lessons → Mathematics or Spelling (SUB001410): the lessons of the profile's grade, the current period first.
 * Every card shows the color and the symbol of the family of its main skill; the buttons under the title
 * keep one family.
 */
export function LessonsScreen() {
  return <RequireProfile>{(profile) => <LessonList profile={profile} />}</RequireProfile>;
}

function LessonList({ profile }: { profile: Profile }) {
  const navigate = useNavigate();
  const subject = subjectOfSlug(useParams().subject);
  const lessons = useLessons(profile.grade, subject);
  const data = lessons.data;
  const usesFallback = data !== undefined && data.lessonsGrade !== data.grade;
  const categories = data?.lessons.map((l) => l.category) ?? [];
  // The family chosen above the lists (every family when none).
  const [family, setFamily] = useFamilyFilter(categories);
  const shown = data?.lessons.filter((l) => family === null || l.category === family) ?? [];
  const current = shown.filter((l) => l.isCurrentPeriod);
  const others = shown.filter((l) => !l.isCurrentPeriod);
  const open = (lesson: LessonSummary) => void navigate(`/lecons/${SUBJECT_SLUGS[subject]}/${encodeURIComponent(lesson.id)}`);

  return (
    <main className="screen">
      <header className="screen-header">
        <button type="button" className="back-button" onClick={() => void navigate("/lecons")}>
          ← {fr.common.back}
        </button>
        <h1 className="title">{fr.lessons.titles[subject]}</h1>
        <SpeakButton text={usesFallback ? `${fr.lessons.fallback} ${fr.lessons.spoken}` : fr.lessons.spoken} />
      </header>

      {data && <FamilyFilter categories={categories} selected={family} onSelect={setFamily} />}
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
                category={lesson.category}
                variant="outline"
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
              <BigButton key={lesson.id} label={lesson.title} hint={fr.lessons.hint(lesson.period, lesson.durationMinutes)} category={lesson.category} onClick={() => open(lesson)} />
            ))}
          </div>
        </section>
      )}
    </main>
  );
}
