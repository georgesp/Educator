import { useNavigate } from "react-router";
import { SUBJECT_SLUGS, SUBJECTS } from "../app/subjects";
import { BigButton } from "../components/child/BigButton";
import { SpeakButton } from "../components/child/SpeakButton";
import { BookIcon, PencilIcon } from "../components/icons";
import { RequireProfile } from "../components/RequireProfile";
import { fr } from "../i18n/fr";

type SubjectScreenProps = {
  /** Exercises (`/exercices/<matière>`) or lessons (`/lecons/<matière>`). */
  kind: "exercises" | "lessons";
};

/** Exercises → or Lessons → the subject (SUB001416): mathematics or spelling. */
export function SubjectScreen({ kind }: SubjectScreenProps) {
  return <RequireProfile>{() => <SubjectChoice kind={kind} />}</RequireProfile>;
}

function SubjectChoice({ kind }: SubjectScreenProps) {
  const navigate = useNavigate();
  const base = kind === "exercises" ? "/exercices" : "/lecons";
  const title = kind === "exercises" ? fr.subjects.exercisesTitle : fr.subjects.lessonsTitle;
  const spoken = kind === "exercises" ? fr.subjects.exercisesSpoken : fr.subjects.lessonsSpoken;

  return (
    <main className="screen">
      <header className="screen-header">
        <button type="button" className="back-button" onClick={() => void navigate("/menu")}>
          ← {fr.series.menu}
        </button>
        <h1 className="title">{title}</h1>
        <SpeakButton text={spoken} autoPlay />
      </header>
      <div className="mode-choice">
        <div className="screen-instruction">
          <h2 className="section-title">{fr.subjects.choose}</h2>
        </div>
        {SUBJECTS.map((subject, i) => (
          <BigButton
            key={subject}
            label={fr.subjects.names[subject]}
            hint={fr.subjects.hints[subject]}
            icon={kind === "lessons" ? <BookIcon /> : subject === "Math" ? <PencilIcon /> : <span className="choice-icon">{fr.common.spellingIcon}</span>}
            variant="primary"
            number={String(i + 1).padStart(2, "0")}
            autoFocus={i === 0}
            onClick={() => void navigate(`${base}/${SUBJECT_SLUGS[subject]}`)}
          />
        ))}
      </div>
    </main>
  );
}
