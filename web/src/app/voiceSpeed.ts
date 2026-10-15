import type { VoiceSpeed } from "../api/client";

/**
 * Speed of the voice chosen by the child in Options (SUB001512), applied by the core to every
 * reading. Set when the profile's settings are loaded ({@link ProfileSettingsEffect}).
 */
let voiceSpeed: VoiceSpeed = "Normal";

export function currentVoiceSpeed(): VoiceSpeed {
  return voiceSpeed;
}

export function setVoiceSpeed(speed: VoiceSpeed): void {
  voiceSpeed = speed;
}
