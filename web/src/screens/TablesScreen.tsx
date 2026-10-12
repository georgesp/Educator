import { useQuery } from "@tanstack/react-query";
import { useState } from "react";
import { useNavigate } from "react-router";
import { useApi } from "../app/BackendContext";
import { SpeakButton } from "../components/child/SpeakButton";
import { RequireProfile } from "../components/RequireProfile";
import { fr } from "../i18n/fr";

/** Tables shown in « Mes tables » (the programme of CE1: A × B with A and B from 0 to 10). */
export const TABLES = [2, 3, 4, 5, 6, 7, 8, 9, 10];

/** « Mes tables » (SUB000119): one multiplication table in large, read aloud. */
export function TablesScreen() {
  return <RequireProfile>{() => <Tables />}</RequireProfile>;
}

function Tables() {
  const api = useApi();
  const navigate = useNavigate();
  const [number, setNumber] = useState(2);
  const table = useQuery({ queryKey: ["table", number], queryFn: () => api.getMultiplicationTable(number), staleTime: Infinity });

  return (
    <main className="screen">
      <header className="screen-header">
        <button type="button" className="back-button" onClick={() => void navigate("/exercices")}>
          ← {fr.common.back}
        </button>
        <h1 className="title">{fr.tables.title}</h1>
        <SpeakButton text={fr.tables.spoken} autoPlay />
      </header>

      <div className="table-choice" role="group" aria-label={fr.tables.choose}>
        {TABLES.map((n) => (
          <button key={n} type="button" className="grade-button" aria-pressed={n === number} onClick={() => setNumber(n)}>
            {n}
          </button>
        ))}
      </div>

      {table.isError && <p className="message">{fr.app.error}</p>}
      {table.data && (
        <section className="table-card" aria-label={fr.tables.table(table.data.number)}>
          <div className="table-heading">
            <h2 className="section-title">{fr.tables.table(table.data.number)}</h2>
            <SpeakButton text={table.data.lines.map((l) => l.spoken).join(" ")} label={fr.tables.listen} />
          </div>
          <ol className="table-lines">
            {table.data.lines.map((line) => (
              <li key={line.right} className="table-line">
                <span>{line.left}</span>
                <span className="equation-sign">×</span>
                <span>{line.right}</span>
                <span className="equation-sign">=</span>
                <strong>{line.result}</strong>
              </li>
            ))}
          </ol>
        </section>
      )}
    </main>
  );
}
