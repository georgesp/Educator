import { useQuery } from "@tanstack/react-query";
import { useApi } from "../app/BackendContext";
import { fr } from "../i18n/fr";

/** Discreet line at the bottom: version, voice and database state (diagnostics for the parent). */
export function StatusBar() {
  const api = useApi();
  const { data } = useQuery({ queryKey: ["status"], queryFn: () => api.getStatus() });
  if (!data) {
    return null;
  }

  const { speech } = data;
  return (
    <footer className="status-bar" aria-label="État de l'application">
      <span>{fr.status.version(data.version)}</span>
      <span title={speech.problem ?? undefined}>
        {speech.isAvailable && speech.voice
          ? fr.status.voiceReady(speech.voice)
          : speech.voiceToInstall
            ? fr.status.voiceToInstall
            : fr.status.voiceUnavailable}
      </span>
      {!data.database.isReady && <span>{fr.status.databaseNotReady}</span>}
    </footer>
  );
}
