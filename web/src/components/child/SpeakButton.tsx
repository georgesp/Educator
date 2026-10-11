import { useEffect, useRef } from "react";
import { useSpeech } from "../../audio/useSpeech";
import { fr } from "../../i18n/fr";
import { SpeakerIcon } from "../icons";

type SpeakButtonProps = {
  text: string;
  /** Reads the text once when the button appears (instructions of a new screen). */
  autoPlay?: boolean;
  label?: string;
  compact?: boolean;
};

/** Reads a French text aloud with the local voice. */
export function SpeakButton({ text, autoPlay = false, label = fr.common.listen, compact = false }: SpeakButtonProps) {
  const speak = useSpeech();
  const played = useRef<string | null>(null);

  useEffect(() => {
    if (autoPlay && played.current !== text) {
      played.current = text;
      void speak(text);
    }
  }, [autoPlay, speak, text]);

  return (
    <button type="button" className={compact ? "speak-button speak-compact" : "speak-button"} onClick={() => void speak(text)} aria-label={compact ? label : undefined}>
      <SpeakerIcon size={24} />
      {!compact && <span>{label}</span>}
    </button>
  );
}
