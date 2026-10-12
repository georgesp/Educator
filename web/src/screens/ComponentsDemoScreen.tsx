import { useState } from "react";
import { BigButton } from "../components/child/BigButton";
import { CharacterImage } from "../components/child/CharacterImage";
import { ChoiceGrid } from "../components/child/ChoiceGrid";
import { Feedback } from "../components/child/Feedback";
import { NumberPad } from "../components/child/NumberPad";
import { PointsBadge } from "../components/child/PointsBadge";
import { RaceTrack } from "../components/child/RaceTrack";
import { SpeakButton } from "../components/child/SpeakButton";
import { LockIcon, PencilIcon } from "../components/icons";
import { useCharacters } from "../app/queries";

/**
 * Development page (route /dev/composants, never in the built application): every child component of
 * SUB000105, to check the look and the behaviour in one place. Texts here are samples, not product copy.
 */
export function ComponentsDemoScreen() {
  const [value, setValue] = useState("");
  const [picked, setPicked] = useState<number | null>(null);
  const [points, setPoints] = useState(120);
  const [done, setDone] = useState(3);
  const characters = useCharacters();

  return (
    <main className="screen demo">
      <h1 className="title">Composants enfant</h1>

      <section>
        <h2 className="section-title">BigButton</h2>
        <div className="demo-row">
          <BigButton number="01" label="Exercices" hint="Nombres et calcul" icon={<PencilIcon />} variant="primary" />
          <BigButton number="02" label="Leçons" hint="Au stand, bientôt" icon={<LockIcon />} locked />
          <BigButton label="Bouton contour" variant="outline" />
        </div>
      </section>

      <section>
        <h2 className="section-title">NumberPad</h2>
        <p className="message">Saisi : {value || "—"}</p>
        <NumberPad value={value} onChange={setValue} onSubmit={() => setValue("")} />
      </section>

      <section>
        <h2 className="section-title">ChoiceGrid</h2>
        <ChoiceGrid labels={["11", "12", "13", "22"]} onPick={setPicked} picked={picked} expected={picked === null ? null : 1} label="Exemple de QCM" />
      </section>

      <section>
        <h2 className="section-title">SpeakButton, Feedback, PointsBadge</h2>
        <div className="demo-row">
          <SpeakButton text="Combien font 7 plus 5 ?" />
          <SpeakButton text="Bonjour" compact />
          <PointsBadge points={points} />
          <button type="button" className="button" onClick={() => setPoints((p) => p + 10)}>
            +10 points
          </button>
        </div>
        <div className="demo-row">
          <Feedback kind="correct" message="Bravo !" />
          <Feedback kind="incorrect" message="Presque ! Regarde bien :" detail="7 + 5 = 12" />
        </div>
      </section>

      <section>
        <h2 className="section-title">RaceTrack</h2>
        <RaceTrack done={done} total={10} results={Array.from({ length: 10 }, (_, i) => (i < done ? i !== 1 : null))} label="Progression" />
        <button type="button" className="button" onClick={() => setDone((d) => (d + 1) % 11)}>
          Avancer
        </button>
      </section>

      <section>
        <h2 className="section-title">Personnages</h2>
        <div className="demo-row">
          {characters.data?.map((c) => <CharacterImage key={c.id} id={c.id} size={96} alt={c.name} />)}
        </div>
      </section>
    </main>
  );
}
