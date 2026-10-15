import type { ReactNode } from "react";
import { useNavigate, useParams } from "react-router";
import type { AnswerMode, Profile, Skill, SpellingFormat } from "../api/client";
import { useSkills, useVoiceAvailable } from "../app/queries";
import { SUBJECT_SLUGS, subjectOfSlug } from "../app/subjects";
import { BigButton } from "../components/child/BigButton";
import { SpeakButton } from "../components/child/SpeakButton";
import { PencilIcon } from "../components/icons";
import { RequireProfile } from "../components/RequireProfile";
import { fr } from "../i18n/fr";

/**
 * Second step (SUB000109): « J'écris la réponse » (keyboard) or « Je choisis la réponse » (QCM). For a
 * spelling skill, the kind of exercise (SUB001406 to SUB001408): the dictation, the missing letters
 * (written or picked), the right spelling. Without the voice, the kinds where the word is only heard
 * are greyed with a kind message (SUB026704); the missing letters stay playable.
 */
export function ModeScreen() {
  return <RequireProfile>{(profile) => <Mode profile={profile} />}</RequireProfile>;
}

type Choice = {
  key: string;
  label: string;
  hint?: string;
  mode: AnswerMode;
  format?: SpellingFormat;
  icon: ReactNode;
  /** The word is only heard: not playable without the voice. */
  listening?: boolean;
};

/** The kinds of spelling exercise of a skill, each with its answer mode. */
function spellingChoices(skill: Skill): Choice[] {
  const choices: Choice[] = [];
  const has = (mode: AnswerMode) => skill.answerModes.includes(mode);
  for (const format of skill.spellingFormats) {
    const label = fr.skills.kinds[format];
    const listening = skill.listeningFormats.includes(format);
    if (format === "Dictation" && has("Keyboard")) {
      choices.push({ key: format, label, hint: fr.skills.kindHints.Dictation, mode: "Keyboard", format, icon: <PencilIcon />, listening });
    } else if (format === "ChooseSpelling" && has("Choice")) {
      choices.push({
        key: format,
        label,
        hint: fr.skills.kindHints.ChooseSpelling,
        mode: "Choice",
        format,
        icon: <span className="choice-icon">3</span>,
        listening,
      });
    } else if (format === "MissingLetters") {
      if (has("Keyboard")) {
        choices.push({ key: `${format}-k`, label, hint: fr.skills.kindHints.MissingLetters, mode: "Keyboard", format, icon: <PencilIcon /> });
      }
      if (has("Choice")) {
        choices.push({ key: `${format}-c`, label, hint: fr.skills.kindHints.MissingLettersChoice, mode: "Choice", format, icon: <span className="choice-icon">3</span> });
      }
    }
  }

  return choices;
}

function Mode({ profile }: { profile: Profile }) {
  const navigate = useNavigate();
  const params = useParams();
  const subject = subjectOfSlug(params.subject);
  const skillId = params.skillId ?? "";
  const skill = useSkills(profile.grade, subject, profile.id).data?.skills.find((s) => s.id === skillId);
  const voice = useVoiceAvailable();
  const spelling = skill?.family === "Spelling";
  const choices: Choice[] = !skill
    ? []
    : spelling
      ? spellingChoices(skill)
      : [
          ...(skill.answerModes.includes("Keyboard") ? [{ key: "k", label: fr.skills.keyboard, mode: "Keyboard" as const, icon: <PencilIcon /> }] : []),
          ...(skill.answerModes.includes("Choice") ? [{ key: "c", label: fr.skills.choice, mode: "Choice" as const, icon: <span className="choice-icon">4</span> }] : []),
        ];
  const locked = (choice: Choice) => !voice && choice.listening === true;
  const start = (choice: Choice) => {
    if (locked(choice)) {
      return;
    }
    void navigate(`/serie/${encodeURIComponent(skillId)}/${choice.mode}${choice.format ? `?format=${choice.format}` : ""}`);
  };

  return (
    <main className="screen">
      <header className="screen-header">
        <button type="button" className="back-button" onClick={() => void navigate(`/exercices/${SUBJECT_SLUGS[subject]}`)}>
          ← {fr.common.back}
        </button>
        <h1 className="title">{skill?.title ?? fr.app.loading}</h1>
      </header>
      <div className="mode-choice">
        <div className="screen-instruction">
          <h2 className="section-title">{spelling ? fr.skills.chooseKind : fr.skills.chooseMode}</h2>
          <SpeakButton text={spelling ? fr.skills.chooseKindSpoken : fr.skills.chooseModeSpoken} compact />
        </div>
        {choices.some(locked) && (
          <p className="notice" role="status">
            {fr.skills.noVoice}
          </p>
        )}
        {choices.map((choice, i) => (
          <BigButton
            key={choice.key}
            label={choice.label}
            hint={choice.hint}
            icon={choice.icon}
            variant="primary"
            number={String(i + 1).padStart(2, "0")}
            locked={locked(choice)}
            onClick={() => start(choice)}
            autoFocus={i === choices.findIndex((c) => !locked(c))}
          />
        ))}
      </div>
    </main>
  );
}
