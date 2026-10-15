import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useEffect, useState } from "react";
import type { ProfileSettings, UpdateProfileSettings } from "../api/client";
import { setMuted } from "../audio/sounds";
import { stopSpeech } from "../audio/useSpeech";
import { fr } from "../i18n/fr";
import { useApi } from "./BackendContext";
import { useProfileSession } from "./ProfileContext";
import { queryKeys, useProfileSettings } from "./queries";
import { setMascotHidden } from "./settings";
import { setVoiceSpeed } from "./voiceSpeed";

/** The theme of the screens without a profile (the profile choice). */
const DEFAULT_THEME = "sport";

/** Applies the settings of a child to the whole application (or the defaults, without a child). */
export function applySettings(settings: ProfileSettings | null): void {
  const root = document.documentElement;
  root.dataset.theme = settings?.theme ?? DEFAULT_THEME;
  root.dataset.textSize = (settings?.textSize ?? "Normal").toLowerCase();
  if (settings?.dyslexiaFont) {
    root.dataset.font = "dyslexia";
  } else {
    delete root.dataset.font;
  }
  setVoiceSpeed(settings?.voiceSpeed ?? "Normal");
  if (settings) {
    setMuted(!settings.sound);
    setMascotHidden(!settings.mascot);
  }
}

/**
 * Settings of the chosen child (SUB001512, SUB001513): theme, size of the text, font, sounds, mascot,
 * speed of the voice, applied as soon as they are read or changed; and the gentle reminder of a
 * break after the advised length of a session.
 */
export function ProfileSettingsEffect() {
  const { profileId } = useProfileSession();
  const settings = useProfileSettings(profileId).data;
  const active = profileId ? (settings ?? null) : null;

  useEffect(() => {
    applySettings(active);
  }, [active]);

  return active && active.sessionMinutes > 0 ? <BreakReminder key={`${profileId}-${active.sessionMinutes}`} minutes={active.sessionMinutes} /> : null;
}

/** After `minutes` with the same child: « C'est le moment d'une petite pause ». Never blocking. */
function BreakReminder({ minutes }: { minutes: number }) {
  const [shown, setShown] = useState(false);
  const [round, setRound] = useState(0);

  useEffect(() => {
    const timer = window.setTimeout(() => setShown(true), minutes * 60_000);
    return () => window.clearTimeout(timer);
  }, [minutes, round]);

  if (!shown) {
    return null;
  }
  return (
    <div className="break-reminder" role="status" aria-live="polite">
      <p className="break-reminder-title">{fr.breakReminder.title}</p>
      <p>{fr.breakReminder.text(minutes)}</p>
      <button
        type="button"
        className="button button-primary"
        onClick={() => {
          setShown(false);
          setRound((r) => r + 1);
        }}
      >
        {fr.breakReminder.ok}
      </button>
    </div>
  );
}

/** Changes settings of the chosen child; the screen follows at once. */
export function useUpdateSettings(profileId: string | null | undefined) {
  const api = useApi();
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (changes: UpdateProfileSettings) => api.updateProfileSettings(profileId!, changes),
    onMutate: (changes) => {
      if (!profileId) {
        return;
      }
      const next = queryClient.setQueryData<ProfileSettings>(queryKeys.settings(profileId), (current) =>
        current ? { ...current, ...definedOnly(changes) } : current,
      );
      // Shown at once, before the core answers.
      if (next) {
        applySettings(next);
      }
      if (changes.sound === false) {
        stopSpeech();
      }
    },
    onSuccess: (saved) => {
      if (profileId) {
        queryClient.setQueryData(queryKeys.settings(profileId), saved);
        // The goal of the day may have changed.
        void queryClient.invalidateQueries({ queryKey: queryKeys.overview(profileId) });
      }
    },
    onError: () => {
      if (profileId) {
        void queryClient.invalidateQueries({ queryKey: queryKeys.settings(profileId) });
      }
    },
  });
}

function definedOnly(changes: UpdateProfileSettings): Partial<ProfileSettings> {
  return Object.fromEntries(Object.entries(changes).filter(([, v]) => v !== undefined)) as Partial<ProfileSettings>;
}
