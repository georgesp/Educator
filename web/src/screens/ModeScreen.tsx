import { useNavigate, useParams } from "react-router";
import type { AnswerMode, Profile } from "../api/client";
import { useSkills } from "../app/queries";
import { BigButton } from "../components/child/BigButton";
import { SpeakButton } from "../components/child/SpeakButton";
import { PencilIcon } from "../components/icons";
import { RequireProfile } from "../components/RequireProfile";
import { fr } from "../i18n/fr";

/** Second step (SUB000109): « J'écris la réponse » (keyboard) or « Je choisis la réponse » (QCM). */
export function ModeScreen() {
  return <RequireProfile>{(profile) => <Mode profile={profile} />}</RequireProfile>;
}

function Mode({ profile }: { profile: Profile }) {
  const navigate = useNavigate();
  const { skillId = "" } = useParams();
  const skill = useSkills(profile.grade).data?.skills.find((s) => s.id === skillId);
  const modes: AnswerMode[] = skill ? [...skill.answerModes] : [];
  const start = (mode: AnswerMode) => void navigate(`/serie/${encodeURIComponent(skillId)}/${mode}`);

  return (
    <main className="screen">
      <header className="screen-header">
        <button type="button" className="back-button" onClick={() => void navigate("/exercices")}>
          ← {fr.common.back}
        </button>
        <h1 className="title">{skill?.title ?? fr.app.loading}</h1>
      </header>
      <div className="mode-choice">
        <div className="screen-instruction">
          <h2 className="section-title">{fr.skills.chooseMode}</h2>
          <SpeakButton text={fr.skills.chooseModeSpoken} autoPlay compact />
        </div>
        {modes.includes("Keyboard") && (
          <BigButton label={fr.skills.keyboard} icon={<PencilIcon />} variant="primary" number="01" onClick={() => start("Keyboard")} autoFocus />
        )}
        {modes.includes("Choice") && (
          <BigButton label={fr.skills.choice} icon={<span className="choice-icon">4</span>} variant="primary" number="02" onClick={() => start("Choice")} />
        )}
      </div>
    </main>
  );
}
