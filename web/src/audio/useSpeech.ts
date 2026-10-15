import { useCallback, useEffect } from "react";
import { useApi } from "../app/BackendContext";
import { isMuted } from "./sounds";

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
let current: { audio: HTMLAudioElement; url: string; done(): void } | null = null;

/** Readings waiting for the current one to end ({@link queueVoice}); a stop cancels them. */
let queue: Promise<void> = Promise.resolve();
let generation = 0;

export function stopSpeech(): void {
  generation += 1;
  if (current) {
    current.audio.pause();
    URL.revokeObjectURL(current.url);
    current.done();
    current = null;
  }
}

// The voice stops when the application goes to the background (iPad, SUB023806) or the window is
// hidden: a reading never goes on behind another application.
if (typeof document !== "undefined") {
  document.addEventListener("visibilitychange", () => {
    if (document.visibilityState === "hidden") {
      stopSpeech();
    }
  });
}

async function play(load: () => Promise<Blob>, startedAt: number): Promise<void> {
  // « Couper les sons » (Options) silences the voice too.
  if (!voiceEnabled || isMuted()) {
    return;
  }

  try {
    const wav = await load();
    if (startedAt !== generation) {
      // Stopped (or replaced) while the audio was being prepared.
      return;
    }
    const url = URL.createObjectURL(wav);
    const audio = new Audio(url);
    const ended = new Promise<void>((resolve) => {
      current = { audio, url, done: resolve };
      audio.addEventListener("ended", () => resolve(), { once: true });
      audio.addEventListener("error", () => resolve(), { once: true });
    });
    audio.addEventListener(
      "ended",
      () => {
        if (current?.audio === audio) {
          URL.revokeObjectURL(url);
          current = null;
        }
      },
      { once: true },
    );
    await audio.play();
    await ended;
  } catch {
    // Voice unavailable or audio blocked: nothing to do, the text is on screen.
  }
}

/**
 * Plays WAV bytes fetched by `load`, stopping what was playing (and what was waiting). Resolves when
 * the audio ends, fails or is stopped. Never throws: without a voice, or while the voice is switched
 * off, nothing is played.
 */
export function playVoice(load: () => Promise<Blob>): Promise<void> {
  stopSpeech();
  return play(load, generation);
}

/**
 * Plays readings one after the other (a word, then its sentence: SUB001406), stopping what was
 * playing; a stop cancels the readings not started yet.
 */
export async function playVoices(loads: Array<() => Promise<Blob>>): Promise<void> {
  stopSpeech();
  const startedAt = generation;
  for (const load of loads) {
    if (startedAt !== generation) {
      return;
    }
    await play(load, startedAt);
  }
}

/** Plays after what is being read; a stop cancels it. */
export function queueVoice(load: () => Promise<Blob>): Promise<void> {
  const startedAt = generation;
  queue = queue.then(() => (startedAt === generation ? play(load, startedAt) : undefined));
  return queue;
}

/**
 * Reads a French text aloud with the local voice (Piper, through the core). Never throws: without a
 * voice, or while the voice is switched off, the screen simply stays silent.
 */
export function useSpeech(): (text: string) => Promise<void> {
  const api = useApi();
  return useCallback((text: string) => playVoice(() => api.speak(text)), [api]);
}

/** Reads a text after what is being read. */
export function useQueuedSpeech(): (text: string) => Promise<void> {
  const api = useApi();
  return useCallback((text: string) => queueVoice(() => api.speak(text)), [api]);
}

/**
 * The voice on while a screen is shown (the French exercises and lessons, SUB001406): switched back
 * off when the screen goes.
 */
export function useVoiceWhile(enabled: boolean): void {
  // Turned on during the render, so that the first reading of the screen is not lost.
  if (enabled && !voiceEnabled) {
    voiceEnabled = true;
  }

  useEffect(() => {
    if (!enabled) {
      return;
    }

    setVoiceEnabled(true);
    return () => setVoiceEnabled(false);
  }, [enabled]);
}
