import { Link, useNavigate } from "react-router";
import { setMuted, useMuted } from "../audio/sounds";
import { stopSpeech } from "../audio/useSpeech";
import { useProfiles } from "../app/queries";
import { setMascotHidden, useMascotHidden } from "../app/settings";
import { CharacterImage } from "../components/child/CharacterImage";
import { LockIcon, PencilIcon } from "../components/icons";
import { RequireProfile } from "../components/RequireProfile";
import { fr } from "../i18n/fr";

/**
 * Options (SUB001416): the sounds, the mascot, the profiles (change one) and the way to the parents'
 * space, where a profile is deleted (SUB001510).
 */
export function OptionsScreen() {
  return <RequireProfile>{() => <Options />}</RequireProfile>;
}

function Options() {
  const navigate = useNavigate();
  const profiles = useProfiles();
  const muted = useMuted();
  const mascotHidden = useMascotHidden();

  const toggleSounds = () => {
    setMuted(!muted);
    if (!muted) {
      stopSpeech();
    }
  };

  return (
    <main className="screen options-screen">
      <header className="screen-header">
        <button type="button" className="back-button" onClick={() => void navigate("/menu")}>
          ← {fr.series.menu}
        </button>
        <h1 className="title">{fr.options.title}</h1>
      </header>

      <section className="options-section" aria-labelledby="options-sounds">
        <h2 id="options-sounds" className="section-title">
          {fr.options.sounds}
        </h2>
        <label className="option-switch">
          <input type="checkbox" checked={muted} onChange={toggleSounds} />
          <span>{fr.options.mute}</span>
        </label>
        <label className="option-switch">
          <input type="checkbox" checked={mascotHidden} onChange={() => setMascotHidden(!mascotHidden)} />
          <span>{fr.options.hideMascot}</span>
        </label>
      </section>

      <section className="options-section" aria-labelledby="options-profiles">
        <h2 id="options-profiles" className="section-title">
          {fr.options.profiles}
        </h2>
        <ul className="plain-list options-profiles">
          {profiles.data?.map((profile) => (
            <li key={profile.id} className="options-profile">
              <CharacterImage id={profile.characterId} size={56} />
              <span className="options-profile-name">
                {profile.name} · {fr.grades[profile.grade]}
              </span>
              <Link className="button" to={`/profil/${encodeURIComponent(profile.id)}?retour=options`} aria-label={fr.profiles.editLabel(profile.name)}>
                <PencilIcon size={18} /> {fr.profiles.edit}
              </Link>
            </li>
          ))}
        </ul>
      </section>

      <section className="options-section" aria-labelledby="options-parent">
        <h2 id="options-parent" className="section-title">
          {fr.options.parentSpace}
        </h2>
        <p className="parent-hint">{fr.options.parentSpaceHint}</p>
        <div>
          <Link className="button" to="/parent">
            <LockIcon size={22} /> {fr.options.parentSpace}
          </Link>
        </div>
      </section>
    </main>
  );
}
