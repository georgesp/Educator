import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { Link, useNavigate } from "react-router";
import type { Profile } from "../api/client";
import { setMuted, useMuted } from "../audio/sounds";
import { stopSpeech } from "../audio/useSpeech";
import { useApi } from "../app/BackendContext";
import { useProfileSession } from "../app/ProfileContext";
import { queryKeys, useProfiles } from "../app/queries";
import { setMascotHidden, useMascotHidden } from "../app/settings";
import { CharacterImage } from "../components/child/CharacterImage";
import { PencilIcon, TrashIcon } from "../components/icons";
import { RequireProfile } from "../components/RequireProfile";
import { fr } from "../i18n/fr";

/**
 * Options, first version (SUB001416): the sounds, the mascot, the profiles (change one, or delete it
 * with an explicit two-step confirmation: never by an accidental click).
 */
export function OptionsScreen() {
  return <RequireProfile>{(profile) => <Options current={profile} />}</RequireProfile>;
}

function Options({ current }: { current: Profile }) {
  const navigate = useNavigate();
  const profiles = useProfiles();
  const muted = useMuted();
  const mascotHidden = useMascotHidden();
  const [deleting, setDeleting] = useState<Profile | null>(null);

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
              <button type="button" className="button button-danger" onClick={() => setDeleting(profile)} aria-label={fr.options.deleteLabel(profile.name)}>
                <TrashIcon size={18} /> {fr.options.delete}
              </button>
            </li>
          ))}
        </ul>
      </section>

      {deleting && <DeleteProfile key={deleting.id} profile={deleting} isCurrent={deleting.id === current.id} onClose={() => setDeleting(null)} />}
    </main>
  );
}

/**
 * Deleting a profile, in two steps: the question first, then the profile's name to type before the
 * button deletes anything.
 */
function DeleteProfile({ profile, isCurrent, onClose }: { profile: Profile; isCurrent: boolean; onClose(): void }) {
  const api = useApi();
  const queryClient = useQueryClient();
  const navigate = useNavigate();
  const { setProfileId } = useProfileSession();
  const [step, setStep] = useState<1 | 2>(1);
  const [typed, setTyped] = useState("");
  const matches = typed.trim().toLocaleLowerCase("fr") === profile.name.toLocaleLowerCase("fr");

  const remove = useMutation({
    mutationFn: () => api.deleteProfile(profile.id),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: queryKeys.profiles });
      onClose();
      if (isCurrent) {
        setProfileId(null);
        await navigate("/");
      }
    },
  });

  return (
    <div className="quit-confirm options-delete" role="alertdialog" aria-labelledby="delete-question" aria-describedby="delete-warning">
      <p id="delete-question">{fr.options.deleteQuestion(profile.name)}</p>
      <p id="delete-warning" className="message">
        {fr.options.deleteWarning}
      </p>
      {step === 1 ? (
        <div className="actions">
          <button type="button" className="button" onClick={onClose} autoFocus>
            {fr.options.keep}
          </button>
          <button type="button" className="button button-danger" onClick={() => setStep(2)}>
            {fr.options.continue}
          </button>
        </div>
      ) : (
        <form
          className="options-delete-confirm"
          onSubmit={(e) => {
            e.preventDefault();
            if (matches) {
              remove.mutate();
            }
          }}
        >
          <label>
            {fr.options.typeName(profile.name)}
            <input className="name-input" value={typed} onChange={(e) => setTyped(e.target.value)} autoComplete="off" autoFocus />
          </label>
          <div className="actions">
            <button type="button" className="button" onClick={onClose}>
              {fr.options.keep}
            </button>
            <button type="submit" className="button button-danger" disabled={!matches || remove.isPending}>
              {fr.options.deleteForever}
            </button>
          </div>
          {remove.isError && <p className="message">{fr.app.error}</p>}
        </form>
      )}
    </div>
  );
}
