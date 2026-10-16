import { useQueryClient } from "@tanstack/react-query";
import { useEffect } from "react";
import { useSyncInterruption } from "../components/parent/SyncSection";
import { useApi } from "./BackendContext";

/** Once per start of the application. */
let attempted = false;

/** Tests: the next profile choice tries again. */
export function resetAutomaticSync(): void {
  attempted = false;
}

/**
 * On the iPad, at the profile choice: a silent synchronisation with the paired computer
 * (SUB027602). Nothing is shown to the child; a failure (computer off, other Wi-Fi) simply waits
 * for the next time, and a question « Est-ce le même enfant ? » waits in the parents' space.
 */
export function useAutomaticSync(): void {
  const api = useApi();
  const queryClient = useQueryClient();
  useSyncInterruption(api);

  useEffect(() => {
    if (attempted) {
      return;
    }
    attempted = true;
    void (async () => {
      try {
        const status = await api.getSyncStatus();
        if (status.role !== "Tablet" || status.devices.length === 0) {
          return;
        }
        const result = await api.syncNow(true);
        if (result.outcome === "Done" && result.received > 0) {
          await queryClient.invalidateQueries();
        }
      } catch {
        // Silent: the parents' space tells what happened when asked.
      }
    })();
  }, [api, queryClient]);
}
