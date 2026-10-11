import { useCallback } from "react";
import { useApi } from "../app/BackendContext";

/**
 * Voice switch. The voice is off for now: it adds nothing to the home and the maths exercises, and is
 * kept for the French exercises (dictation, words), which will turn it back on. While it is off, nothing is
 * read, `speak` is never called and the speaker buttons are hidden. The core (Piper, `speak` command,
 * cache, text normalisation) is untouched.
 */
let voiceEnabled = false;

export const isVoiceEnabled = (): boolean => voiceEnabled;

/** Turns the voice on or off (the French exercises will turn it on; tests do too). */
export function setVoiceEnabled(enabled: boolean): void {
  voiceEnabled = enabled;
  if (!enabled) {
    stopSpeech();
  }
}

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
 * voice, or while the voice is switched off, the screen simply stays silent.
 */
export function useSpeech(): (text: string) => Promise<void> {
  const api = useApi();
  return useCallback(
    async (text: string) => {
      if (!voiceEnabled) {
        return;
      }

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
