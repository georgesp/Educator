import { isVoiceEnabled, useSpeech } from "../../audio/useSpeech";
import { fr } from "../../i18n/fr";
import { SpeakerIcon } from "../icons";

type SpeakButtonProps = {
  text: string;
  label?: string;
  compact?: boolean;
};

/**
 * Reads a French text aloud with the local voice, on click only: nothing is ever read by itself
 * (decision of 2026-10-15). Not shown while the voice is switched off.
 */
export function SpeakButton({ text, label = fr.common.listen, compact = false }: SpeakButtonProps) {
  const speak = useSpeech();

  if (!isVoiceEnabled()) {
    return null;
  }

  return (
    <button type="button" className={compact ? "speak-button speak-compact" : "speak-button"} onClick={() => void speak(text)} aria-label={compact ? label : undefined}>
      <SpeakerIcon size={24} />
      {!compact && <span>{label}</span>}
    </button>
  );
}
