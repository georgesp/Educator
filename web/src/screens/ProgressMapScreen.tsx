import { useState } from "react";
import { useNavigate, useParams } from "react-router";
import type { LessonSummary, Profile, Skill } from "../api/client";
import { useLessons, useSkills } from "../app/queries";
import { SUBJECT_SLUGS, SUBJECTS, subjectOfSlug } from "../app/subjects";
import { familyClass, FamilySymbol } from "../components/child/Family";
import { Stars } from "../components/child/Rewards";
import { BookIcon, PencilIcon } from "../components/icons";
import { RequireProfile } from "../components/RequireProfile";
import { fr } from "../i18n/fr";

/** Steps per row of the map; the rows go left to right, then right to left, like a game board. */
const COLUMNS = 6;

/** Height of a row (px): a step, its title on two lines and its stars. */
const ROW_HEIGHT = 140;

/**
 * My map (SUB001506): the path of a subject and a grade, one step per skill (with its stars), in the
 * order of the periods. The steps of the current period stand out; those to come are shown « À
 * venir », never locked. A step opens its lesson or its exercises. Drawn from the skills: no map is
 * drawn by hand.
 */
export function ProgressMapScreen() {
  return <RequireProfile>{(profile) => <ProgressMap profile={profile} />}</RequireProfile>;
}

/** Centre of step `index` in the grid, in units of cells (x: column + 0.5, y: row + 0.5). */
function place(index: number): { x: number; y: number } {
  const row = Math.floor(index / COLUMNS);
  const column = index % COLUMNS;
  return { x: (row % 2 === 0 ? column : COLUMNS - 1 - column) + 0.5, y: row + 0.5 };
}

function ProgressMap({ profile }: { profile: Profile }) {
  const navigate = useNavigate();
  const subject = subjectOfSlug(useParams().subject);
  const slug = SUBJECT_SLUGS[subject];
  const skills = useSkills(profile.grade, subject, profile.id);
  const lessons = useLessons(profile.grade, subject, profile.id);
  const [chosen, setChosen] = useState<string | null>(null);
  const data = skills.data;
  // The grade's own skills, in the order of the periods (the revisions are not steps of the path).
  const steps = data?.skills.filter((s) => s.revisionFrom === null) ?? [];
  const rows = Math.max(1, Math.ceil(steps.length / COLUMNS));
  const selected = steps.find((s) => s.id === chosen);
  const lessonOf = (skill: Skill): LessonSummary | undefined => lessons.data?.lessons.find((l) => l.skillId === skill.id);
  const usesFallback = data !== undefined && data.skillsGrade !== data.grade;
  const path = steps
    .map((_, i) => place(i))
    .map((p, i) => `${i === 0 ? "M" : "L"} ${p.x * 100} ${p.y * 100}`)
    .join(" ");

  return (
    <main className="screen map-screen">
      <header className="screen-header">
        <button type="button" className="back-button" onClick={() => void navigate("/menu")}>
          ← {fr.series.menu}
        </button>
        <h1 className="title">{fr.map.title}</h1>
        <div className="map-subjects" role="group" aria-label={fr.map.subjects}>
          {SUBJECTS.map((s) => (
            <button key={s} type="button" className="family-filter-button" aria-pressed={s === subject} onClick={() => void navigate(`/carte/${SUBJECT_SLUGS[s]}`)}>
              {fr.subjects.names[s]}
            </button>
          ))}
        </div>
      </header>
      {skills.isLoading && <p className="message">{fr.app.loading}</p>}
      {skills.isError && <p className="message">{fr.app.error}</p>}
      {usesFallback && <p className="notice">{fr.skills.noSkills}</p>}

      {data && steps.length > 0 && (
        <div className="map-board" style={{ height: `${rows * ROW_HEIGHT}px` }}>
          <svg className="map-road" viewBox={`0 0 ${COLUMNS * 100} ${rows * 100}`} preserveAspectRatio="none" aria-hidden="true">
            <path d={path} className="map-road-edge" />
            <path d={path} className="map-road-line" />
          </svg>
          <ol className="plain-list map-steps" aria-label={fr.map.stepsOf(fr.subjects.names[subject], fr.grades[data.skillsGrade])}>
            {steps.map((skill, i) => {
              const { x, y } = place(i);
              const future = skill.period > data.currentPeriod;
              const current = skill.period === data.currentPeriod;
              const newPeriod = i === 0 || steps[i - 1].period !== skill.period;
              return (
                <li key={skill.id} className="map-step-slot" style={{ left: `${(x / COLUMNS) * 100}%`, top: `${(y / rows) * 100}%` }}>
                  {newPeriod && <span className="map-period">{fr.map.period(skill.period)}</span>}
                  <button
                    type="button"
                    className={["map-step", current ? "map-step-current" : "", future ? "map-step-future" : "", chosen === skill.id ? "map-step-chosen" : ""].join(" ")}
                    aria-pressed={chosen === skill.id}
                    aria-label={fr.map.stepLabel(skill.title, skill.stars, current, future)}
                    onClick={() => setChosen(skill.id)}
                  >
                    <FamilySymbol category={skill.category} className={`map-step-family ${familyClass(skill.category)}`} />
                  </button>
                  <span className="map-step-title" aria-hidden="true">
                    {skill.title}
                  </span>
                  {future ? (
                    <span className="map-step-soon" aria-hidden="true">
                      {fr.map.soon}
                    </span>
                  ) : (
                    <Stars count={skill.stars} size={14} className="map-step-stars" />
                  )}
                </li>
              );
            })}
          </ol>
        </div>
      )}

      {selected && (
        <section className="map-detail" aria-labelledby="map-detail-title">
          <h2 id="map-detail-title" className="section-title">
            {selected.title}
          </h2>
          <Stars count={selected.stars} size={22} />
          <p className="parent-hint">
            {selected.period > (data?.currentPeriod ?? 0) ? fr.map.soonHint : fr.skills.period(selected.period)}
            {selected.mastery !== null && ` · ${fr.skills.mastery(selected.mastery)}`}
          </p>
          <div className="actions">
            {lessonOf(selected) && (
              <button type="button" className="button" onClick={() => void navigate(`/lecons/${slug}/${encodeURIComponent(lessonOf(selected)?.id ?? "")}`)}>
                <BookIcon size={20} /> {fr.map.lesson}
              </button>
            )}
            <button type="button" className="button button-primary" onClick={() => void navigate(`/exercices/${slug}/${encodeURIComponent(selected.id)}`)} autoFocus>
              <PencilIcon size={20} /> {fr.map.exercises}
            </button>
          </div>
        </section>
      )}
    </main>
  );
}
