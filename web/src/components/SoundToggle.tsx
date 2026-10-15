import { useProfileSession } from "../app/ProfileContext";
import { useUpdateSettings } from "../app/ProfileSettingsEffect";
import { setMuted, useMuted } from "../audio/sounds";
import { stopSpeech } from "../audio/useSpeech";
import { fr } from "../i18n/fr";
import { MutedIcon, SpeakerIcon } from "./icons";

/** Mutes the sounds and the voice; with a child chosen, kept in its settings (SUB001512). */
export function SoundToggle() {
  const muted = useMuted();
  const { profileId } = useProfileSession();
  const update = useUpdateSettings(profileId);
  const toggle = () => {
    setMuted(!muted);
    if (!muted) {
      stopSpeech();
    }
    if (profileId) {
      update.mutate({ sound: muted });
    }
  };

  return (
    <button type="button" className="sound-toggle" onClick={toggle} aria-pressed={muted} aria-label={muted ? fr.common.soundOff : fr.common.soundOn}>
      {muted ? <MutedIcon size={24} /> : <SpeakerIcon size={24} />}
    </button>
  );
}
