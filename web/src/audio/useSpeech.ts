import { useCallback } from "react";
import { useApi } from "../app/BackendContext";

/** The audio element playing now: a new reading stops the previous one (one voice at a time). */
let current: { audio: HTMLAudioElement; url: string } | null = null;

export function stopSpeech(): void {
  if (current) {
    current.audio.pause();
    URL.revokeObjectURL(current.url);
    current = null;
  }
}

/**
 * Reads a French text aloud with the local voice (Piper, through the host). Never throws: without a
 * voice, the screen simply stays silent.
 */
export function useSpeech(): (text: string) => Promise<void> {
  const api = useApi();
  return useCallback(
    async (text: string) => {
      stopSpeech();
      try {
        const wav = await api.speak(text);
        stopSpeech();
        const url = URL.createObjectURL(wav);
        const audio = new Audio(url);
        current = { audio, url };
        audio.addEventListener("ended", () => current?.audio === audio && stopSpeech(), { once: true });
        await audio.play();
      } catch {
        // Voice unavailable or audio blocked: nothing to do, the text is on screen.
      }
    },
    [api],
  );
}
