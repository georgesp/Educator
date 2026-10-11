import { useNavigate } from "react-router";
import type { Profile, Skill } from "../api/client";
import { useSkills } from "../app/queries";
import { BigButton } from "../components/child/BigButton";
import { SpeakButton } from "../components/child/SpeakButton";
import { PencilIcon } from "../components/icons";
import { RequireProfile } from "../components/RequireProfile";
import { fr } from "../i18n/fr";

/** Exercises → Mathematics (SUB000109): the skills of the profile's grade, the current period first. */
export function SkillsScreen() {
  return <RequireProfile>{(profile) => <Skills profile={profile} />}</RequireProfile>;
}

function Skills({ profile }: { profile: Profile }) {
  const navigate = useNavigate();
  const skills = useSkills(profile.grade);
  const data = skills.data;
  // Grades without skills yet (CE2 to CM2 in lot 1) receive the CE1 ones from the host (SUB000108).
  const usesFallback = data !== undefined && data.skillsGrade !== data.grade;

  const current = data?.skills.filter((s) => s.isCurrentPeriod) ?? [];
  const others = data?.skills.filter((s) => !s.isCurrentPeriod) ?? [];
  const open = (skill: Skill) => void navigate(`/exercices/${encodeURIComponent(skill.id)}`);

  return (
    <main className="screen">
      <header className="screen-header">
        <button type="button" className="back-button" onClick={() => void navigate("/menu")}>
          ← {fr.series.menu}
        </button>
        <h1 className="title">{fr.skills.title}</h1>
        <SpeakButton text={usesFallback ? `${fr.skills.noSkills} ${fr.skills.spoken}` : fr.skills.spoken} autoPlay />
      </header>

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
              <BigButton key={skill.id} label={skill.title} hint={fr.skills.period(skill.period)} icon={<PencilIcon />} variant="primary" onClick={() => open(skill)} />
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
              <BigButton key={skill.id} label={skill.title} hint={fr.skills.period(skill.period)} onClick={() => open(skill)} />
            ))}
          </div>
        </section>
      )}
    </main>
  );
}
