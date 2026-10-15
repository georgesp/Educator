import { useNavigate, useParams } from "react-router";
import type { Profile, Skill } from "../api/client";
import { useSkills, useVoiceAvailable } from "../app/queries";
import { SUBJECT_SLUGS, subjectOfSlug } from "../app/subjects";
import { BigButton } from "../components/child/BigButton";
import { FamilyFilter, useFamilyFilter } from "../components/child/Family";
import { SpeakButton } from "../components/child/SpeakButton";
import { RequireProfile } from "../components/RequireProfile";
import { fr } from "../i18n/fr";

/**
 * Exercises → Mathematics or Spelling (SUB000109, SUB001416): the skills of the profile's grade, the
 * current period first, then the revisions of the grade before (SUB000113), then the other periods.
 * Every card shows the color and the symbol of its family; the buttons under the title keep one family.
 */
export function SkillsScreen() {
  return <RequireProfile>{(profile) => <Skills profile={profile} />}</RequireProfile>;
}

function Skills({ profile }: { profile: Profile }) {
  const navigate = useNavigate();
  const subject = subjectOfSlug(useParams().subject);
  const slug = SUBJECT_SLUGS[subject];
  const skills = useSkills(profile.grade, subject, profile.id);
  const voice = useVoiceAvailable();
  const data = skills.data;
  // Grades without skills yet (CE2 to CM2 in lot 1) receive the CE1 ones from the host (SUB000108).
  const usesFallback = data !== undefined && data.skillsGrade !== data.grade;

  const categories = data?.skills.map((s) => s.category) ?? [];
  // The family chosen above the lists (every family when none).
  const [family, setFamily] = useFamilyFilter(categories);
  const shown = data?.skills.filter((s) => family === null || s.category === family) ?? [];

  const current = shown.filter((s) => s.isCurrentPeriod);
  // Skills of the grade before, offered again at the start of the year (SUB000113).
  const revisions = shown.filter((s) => s.revisionFrom !== null);
  const others = shown.filter((s) => !s.isCurrentPeriod && s.revisionFrom === null);
  // A single way to answer (and a single kind of exercise): straight to the series, no question asked
  // — unless that exercise needs the voice and there is none: the next screen says why (SUB026704).
  const open = (skill: Skill) =>
    void navigate(
      skill.answerModes.length === 1 && skill.spellingFormats.length <= 1 && (voice || skill.listeningFormats.length === 0)
        ? `/serie/${encodeURIComponent(skill.id)}/${skill.answerModes[0]}`
        : `/exercices/${slug}/${encodeURIComponent(skill.id)}`,
    );
  const hint = (skill: Skill, text: string) => (skill.mastery === null ? text : `${text} · ${fr.skills.mastery(skill.mastery)}`);
  // « Mes tables » next to the multiplications (SUB000119).
  const hasTables = data?.skills.some((s) => s.family === "Multiplication") ?? false;

  return (
    <main className="screen">
      <header className="screen-header">
        <button type="button" className="back-button" onClick={() => void navigate("/exercices")}>
          ← {fr.common.back}
        </button>
        <h1 className="title">{fr.skills.titles[subject]}</h1>
        <SpeakButton text={usesFallback ? `${fr.skills.noSkills} ${fr.skills.spoken}` : fr.skills.spoken} />
        {hasTables && (
          <button type="button" className="button tables-button" onClick={() => void navigate("/tables")}>
            {fr.skills.tables}
          </button>
        )}
      </header>

      {data && <FamilyFilter categories={categories} selected={family} onSelect={setFamily} />}
      {skills.isError && <p className="message">{fr.app.error}</p>}
      {usesFallback && <p className="notice">{fr.skills.noSkills}</p>}
      {skills.isLoading && <p className="message">{fr.app.loading}</p>}

      {current.length > 0 && (
        <section className="skill-section skill-section-current" aria-labelledby="skills-current">
          <h2 id="skills-current" className="section-title">
            {fr.skills.current}
          </h2>
          <div className="skill-list">
            {current.map((skill) => (
              <BigButton key={skill.id} label={skill.title} hint={hint(skill, fr.skills.period(skill.period))} category={skill.category} variant="outline" onClick={() => open(skill)} />
            ))}
          </div>
        </section>
      )}

      {revisions.length > 0 && (
        <section className="skill-section skill-section-revisions" aria-labelledby="skills-revisions">
          <h2 id="skills-revisions" className="section-title">
            {fr.skills.revisions}
          </h2>
          <div className="skill-list skill-list-compact">
            {revisions.map((skill) => (
              <BigButton
                key={skill.id}
                label={skill.title}
                hint={hint(skill, fr.skills.revisionFrom(skill.revisionFrom ?? ""))}
                number={skill.revisionFrom ?? undefined}
                category={skill.category}
                onClick={() => open(skill)}
              />
            ))}
          </div>
        </section>
      )}

      {others.length > 0 && (
        <section className="skill-section" aria-labelledby="skills-others">
          <h2 id="skills-others" className="section-title">
            {fr.skills.others}
          </h2>
          <div className="skill-list skill-list-compact">
            {others.map((skill) => (
              <BigButton key={skill.id} label={skill.title} hint={hint(skill, fr.skills.period(skill.period))} category={skill.category} onClick={() => open(skill)} />
            ))}
          </div>
        </section>
      )}
    </main>
  );
}
