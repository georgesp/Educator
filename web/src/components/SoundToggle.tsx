import { setMuted, useMuted } from "../audio/sounds";
import { stopSpeech } from "../audio/useSpeech";
import { fr } from "../i18n/fr";
import { MutedIcon, SpeakerIcon } from "./icons";

/** Mutes the feedback sounds (setting kept in this computer until the Options of lot 3). */
export function SoundToggle() {
  const muted = useMuted();
  const toggle = () => {
    setMuted(!muted);
    if (!muted) {
      stopSpeech();
    }
  };

  return (
    <button type="button" className="sound-toggle" onClick={toggle} aria-pressed={muted} aria-label={muted ? fr.common.soundOff : fr.common.soundOn}>
      {muted ? <MutedIcon size={24} /> : <SpeakerIcon size={24} />}
    </button>
  );
}
