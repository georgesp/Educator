import { useMutation, useQuery } from "@tanstack/react-query";
import { useState } from "react";
import { useLocation, useNavigate } from "react-router";
import { useApi } from "../app/BackendContext";
import { lockParentSpace, parentOrigin, unlockParentSpace } from "../app/parentAccess";
import { LockIcon } from "../components/icons";
import { fr } from "../i18n/fr";

/**
 * The « adult » gate of the parents' space (SUB001510): a multiplication drawn by the core at every
 * opening, and drawn again after a wrong answer. Not a password: a guard against a child's click.
 */
export function ParentGateScreen() {
  const api = useApi();
  const navigate = useNavigate();
  // Where the parent came from: the profile choice ("/") or the options (by default).
  const from = parentOrigin(useLocation().state);
  // Coming back here closes the parents' space.
  useState(() => lockParentSpace());
  const [attempt, setAttempt] = useState(0);
  const [typed, setTyped] = useState("");
  const [wrong, setWrong] = useState(false);
  const gate = useQuery({
    queryKey: ["parent-gate", attempt],
    queryFn: () => api.getParentGate(),
    staleTime: Infinity,
    gcTime: 0,
  });

  const check = useMutation({
    mutationFn: ({ id, answer }: { id: string; answer: number }) => api.checkParentGate(id, answer),
    onSuccess: async (opened) => {
      if (opened) {
        unlockParentSpace();
        await navigate("/parent/espace", { state: { from } });
        return;
      }
      setWrong(true);
      setTyped("");
      setAttempt((a) => a + 1);
    },
  });

  const answer = Number.parseInt(typed, 10);
  return (
    <main className="screen parent-screen">
      <header className="screen-header">
        <button type="button" className="back-button" onClick={() => void navigate(from)}>
          ← {fr.parent.backToOptions}
        </button>
        <h1 className="title">{fr.parent.title}</h1>
      </header>

      <form
        className="parent-gate"
        onSubmit={(e) => {
          e.preventDefault();
          if (gate.data && Number.isFinite(answer)) {
            check.mutate({ id: gate.data.id, answer });
          }
        }}
      >
        <LockIcon size={40} />
        <p>{fr.parent.gateIntro}</p>
        {gate.data && (
          <label className="parent-gate-question">
            <span>{fr.parent.gateQuestion(gate.data.left, gate.data.right)}</span>
            <input
              className="name-input"
              aria-label={fr.parent.gateAnswer}
              inputMode="numeric"
              autoComplete="off"
              value={typed}
              onChange={(e) => setTyped(e.target.value.replace(/\D/g, "").slice(0, 4))}
              autoFocus
            />
          </label>
        )}
        {gate.isError && <p className="message">{fr.app.error}</p>}
        {wrong && (
          <p className="parent-gate-wrong" role="status">
            {fr.parent.gateWrong}
          </p>
        )}
        <button type="submit" className="button button-primary" disabled={!gate.data || !Number.isFinite(answer) || check.isPending}>
          {fr.parent.gateEnter}
        </button>
      </form>
    </main>
  );
}
