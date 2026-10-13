import { useSyncExternalStore } from "react";

/**
 * Short feedback sounds, synthesized with the Web Audio API: no audio file to ship, no licence to track.
 * Gentle for a mistake (two soft notes), never a buzzer. Can be muted (stored in localStorage until the
 * Options of lot 3).
 */
export type SoundName = "correct" | "incorrect" | "finished" | "celebration";

const MUTED_KEY = "educator.sounds.muted";

const melodies: Record<SoundName, Array<[frequency: number, start: number, duration: number]>> = {
  correct: [
    [659.25, 0, 0.12], // E5
    [987.77, 0.1, 0.22], // B5
  ],
  incorrect: [
    [392.0, 0, 0.18], // G4
    [329.63, 0.16, 0.26], // E4
  ],
  finished: [
    [523.25, 0, 0.12], // C5
    [659.25, 0.12, 0.12], // E5
    [783.99, 0.24, 0.12], // G5
    [1046.5, 0.36, 0.35], // C6
  ],
  // Fireworks (SUB000114): a rising fanfare, a little longer than « finished ».
  celebration: [
    [523.25, 0, 0.1], // C5
    [659.25, 0.1, 0.1], // E5
    [783.99, 0.2, 0.1], // G5
    [1046.5, 0.3, 0.18], // C6
    [783.99, 0.5, 0.1], // G5
    [1046.5, 0.6, 0.12], // C6
    [1318.51, 0.72, 0.5], // E6
  ],
};

let context: AudioContext | null = null;

export function isMuted(): boolean {
  try {
    return localStorage.getItem(MUTED_KEY) === "1";
  } catch {
    return false;
  }
}

const listeners = new Set<() => void>();

export function setMuted(muted: boolean): void {
  try {
    localStorage.setItem(MUTED_KEY, muted ? "1" : "0");
  } catch {
    // Storage unavailable: the setting only lasts for this session.
  }
  listeners.forEach((listener) => listener());
}

/** Whether the sounds are muted, updated wherever the setting is changed (header button, Options). */
export function useMuted(): boolean {
  return useSyncExternalStore(
    (listener) => {
      listeners.add(listener);
      return () => listeners.delete(listener);
    },
    isMuted,
    isMuted,
  );
}

export function playSound(name: SoundName): void {
  if (isMuted() || typeof window === "undefined" || !("AudioContext" in window)) {
    return;
  }

  try {
    context ??= new AudioContext();
    const now = context.currentTime;
    for (const [frequency, start, duration] of melodies[name]) {
      const oscillator = context.createOscillator();
      const gain = context.createGain();
      oscillator.type = "triangle";
      oscillator.frequency.value = frequency;
      gain.gain.setValueAtTime(0.0001, now + start);
      gain.gain.exponentialRampToValueAtTime(0.25, now + start + 0.02);
      gain.gain.exponentialRampToValueAtTime(0.0001, now + start + duration);
      oscillator.connect(gain).connect(context.destination);
      oscillator.start(now + start);
      oscillator.stop(now + start + duration + 0.05);
    }
  } catch {
    // No audio device: the animation alone gives the feedback.
  }
}
