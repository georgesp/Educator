import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useEffect, useState, type FormEvent } from "react";
import { Link, useNavigate } from "react-router";
import { ApiError, type Character, type Grade, type Profile } from "../api/client";
import { useAutomaticSync } from "../app/automaticSync";
import { useApi } from "../app/BackendContext";
import { useProfileSession } from "../app/ProfileContext";
import { queryKeys, useParentProfile, useProfiles } from "../app/queries";
import { ProfileAvatar } from "../components/child/AvatarImage";
import { CharacterImage } from "../components/child/CharacterImage";
import { SpeakButton } from "../components/child/SpeakButton";
import { DiceIcon, LockIcon, PencilIcon, PlusIcon } from "../components/icons";
import { fr } from "../i18n/fr";

export const GRADES: Grade[] = ["CP", "CE1", "CE2", "CM1", "CM2"];

/**
 * First screen (SUB000103): choose a profile, or create one. Without any profile, the creation shows at once.
 * The parents' card, last, leads to the parents' space (SUB001510).
 */
export function ProfilesScreen() {
  const profiles = useProfiles();
  // iPad: a silent synchronisation with the computer at the profile choice (lot tablette 2).
  useAutomaticSync();
  const [creating, setCreating] = useState(false);

  if (profiles.isLoading) {
    return (
      <main className="centered">
        <p className="message">{fr.app.loading}</p>
      </main>
    );
  }

  if (profiles.isError) {
    return (
      <main className="centered">
        <p className="message">{fr.app.error}</p>
      </main>
    );
  }

  const list = profiles.data ?? [];
  if (creating || list.length === 0) {
    return <CreateProfile canCancel={list.length > 0} onCancel={() => setCreating(false)} />;
  }

  return (
    <main className="screen">
      <header className="screen-header">
        <h1 className="title">{fr.profiles.title}</h1>
        <SpeakButton text={fr.profiles.spoken} />
      </header>
      <ul className="profile-grid">
        {list.map((profile) => (
          <li key={profile.id} className="profile-item">
            <ProfileCard profile={profile} />
            <Link className="profile-edit" to={`/profil/${encodeURIComponent(profile.id)}`} aria-label={fr.profiles.editLabel(profile.name)}>
              <PencilIcon size={20} />
              {fr.profiles.edit}
            </Link>
          </li>
        ))}
        <li>
          <button type="button" className="profile-card profile-card-new" onClick={() => setCreating(true)}>
            <span className="profile-new-icon">
              <PlusIcon size={64} />
            </span>
            <span className="profile-name">{fr.profiles.newProfile}</span>
          </button>
        </li>
        <li>
          <ParentCard />
        </li>
      </ul>
    </main>
  );
}

/** The parents' card: the way to the parents' space, through the « adult » gate (SUB001510). */
function ParentCard() {
  const parent = useParentProfile();
  const navigate = useNavigate();
  if (!parent.data) {
    return null;
  }

  return (
    <button
      type="button"
      className="profile-card profile-card-parent"
      onClick={() => void navigate("/parent", { state: { from: "/" } })}
      aria-label={fr.profiles.parentCard(parent.data.name)}
    >
      <CharacterImage id={parent.data.characterId} size={144} />
      <span className="profile-name">{parent.data.name}</span>
      <span className="profile-grade profile-parent-badge">
        <LockIcon size={16} /> {fr.profiles.parentBadge}
      </span>
    </button>
  );
}

function ProfileCard({ profile }: { profile: Profile }) {
  const api = useApi();
  const queryClient = useQueryClient();
  const navigate = useNavigate();
  const { setProfileId } = useProfileSession();
  const select = useMutation({
    mutationFn: () => api.selectProfile(profile.id),
    onSuccess: async () => {
      setProfileId(profile.id);
      await queryClient.invalidateQueries({ queryKey: queryKeys.profiles });
      await navigate("/menu");
    },
  });

  return (
    <button type="button" className="profile-card" onClick={() => select.mutate()} disabled={select.isPending} aria-label={fr.profiles.choose(profile.name)}>
      <ProfileAvatar profile={profile} size={144} />
      <span className="profile-name">{profile.name}</span>
      <span className="profile-grade">{fr.grades[profile.grade]}</span>
    </button>
  );
}

function CreateProfile({ canCancel, onCancel }: { canCancel: boolean; onCancel(): void }) {
  const api = useApi();
  const queryClient = useQueryClient();
  const navigate = useNavigate();
  const { setProfileId } = useProfileSession();
  const [name, setName] = useState("");
  const [grade, setGrade] = useState<Grade>("CP");
  // The host draws the character shown here; « Changer de personnage » asks for another one.
  const [character, setCharacter] = useState<Character | null>(null);
  const draw = useMutation({ mutationFn: (except?: string) => api.drawCharacter(except), onSuccess: setCharacter });
  const { mutate: drawCharacter } = draw;
  useEffect(() => drawCharacter(undefined), [drawCharacter]);

  const create = useMutation({
    mutationFn: () => api.createProfile(name, grade, character?.id),
    onSuccess: async (profile) => {
      setProfileId(profile.id);
      await queryClient.invalidateQueries({ queryKey: queryKeys.profiles });
      // The level assessment is offered first (SUB001418); with nothing to ask, it leads to the menu.
      await navigate("/evaluation");
    },
  });

  const submit = (event: FormEvent) => {
    event.preventDefault();
    create.mutate();
  };

  const error = create.error instanceof ApiError && create.error.status === 400 ? create.error.detail : create.isError ? fr.app.error : null;

  return (
    <main className="screen">
      <header className="screen-header">
        <h1 className="title">{fr.profiles.createTitle}</h1>
        <SpeakButton text={fr.profiles.createSpoken} />
      </header>
      <form className="create-profile" onSubmit={submit}>
        <div className="create-fields">
          <label className="field-label" htmlFor="profile-name">
            {fr.profiles.nameLabel}
          </label>
          <input
            id="profile-name"
            className="text-input"
            value={name}
            maxLength={20}
            autoComplete="off"
            autoFocus
            onChange={(e) => setName(e.target.value)}
            aria-invalid={error ? true : undefined}
            aria-describedby={error ? "profile-error" : undefined}
          />
          {error && (
            <p id="profile-error" className="form-error" role="alert">
              {error}
            </p>
          )}

          <fieldset className="grade-picker">
            <legend className="field-label">{fr.profiles.gradeLabel}</legend>
            <div className="grade-buttons">
              {GRADES.map((g) => (
                <button key={g} type="button" className="grade-button" aria-pressed={grade === g} onClick={() => setGrade(g)}>
                  {fr.grades[g]}
                </button>
              ))}
            </div>
          </fieldset>
        </div>

        <div className="create-character">
          <span className="field-label">{fr.profiles.characterLabel}</span>
          {character ? <CharacterImage id={character.id} size={208} alt={character.name} /> : <span className="character-placeholder" style={{ width: 208, height: 208 }} />}
          <button type="button" className="button" onClick={() => drawCharacter(character?.id)} disabled={draw.isPending}>
            <DiceIcon size={24} />
            {fr.profiles.rerollCharacter}
          </button>
        </div>

        <div className="create-actions">
          {canCancel && (
            <button type="button" className="button" onClick={onCancel}>
              {fr.profiles.cancel}
            </button>
          )}
          <button type="submit" className="button button-primary" disabled={create.isPending}>
            {fr.profiles.start}
          </button>
        </div>
      </form>
    </main>
  );
}
