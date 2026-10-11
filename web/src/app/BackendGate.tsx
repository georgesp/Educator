import { useEffect, useMemo, useState, type ReactNode } from "react";
import { waitForBackend, type BackendStatus } from "../api/backend";
import { createUiApi } from "../api/ui";
import { fr } from "../i18n/fr";
import { BackendProvider } from "./BackendContext";

/** Waits for the .NET backend started by the Tauri shell, then renders the application. */
export function BackendGate({ children }: { children: ReactNode }) {
  const [status, setStatus] = useState<BackendStatus>({ state: "starting" });
  const [attempt, setAttempt] = useState(0);

  useEffect(() => {
    const controller = new AbortController();
    waitForBackend(controller.signal)
      .then((result) => !controller.signal.aborted && setStatus(result))
      .catch((error: unknown) =>
        setStatus({ state: "failed", message: error instanceof Error ? error.message : String(error) }),
      );
    return () => controller.abort();
  }, [attempt]);

  const api = useMemo(() => (status.state === "ready" ? createUiApi(status) : null), [status]);

  if (status.state === "failed") {
    return (
      <main className="centered">
        <p className="message">{fr.app.startFailed}</p>
        <pre className="details">{status.message}</pre>
        <button
          className="button"
          onClick={() => {
            setStatus({ state: "starting" });
            setAttempt((n) => n + 1);
          }}
        >
          {fr.app.retry}
        </button>
      </main>
    );
  }

  if (!api) {
    return (
      <main className="centered">
        <p className="message">{fr.app.starting}</p>
      </main>
    );
  }

  return <BackendProvider api={api}>{children}</BackendProvider>;
}
