import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useState, type FormEvent } from "react";
import { useNavigate, useParams, useSearchParams } from "react-router";
import { ApiError, type Grade, type Profile } from "../api/client";
import { useApi } from "../app/BackendContext";
import { queryKeys, useCharacters, useProfiles } from "../app/queries";
import { CharacterImage } from "../components/child/CharacterImage";
import { SpeakButton } from "../components/child/SpeakButton";
import { DiceIcon } from "../components/icons";
import { fr } from "../i18n/fr";
import { GRADES } from "./ProfilesScreen";

/** Where the sheet was opened from, to go back there (`?retour=menu`, otherwise the profile choice). */
const backPath = (from: string | null) => (from === "menu" ? "/menu" : "/");

/**
 * Profile sheet (SUB000115): change the name, the grade or the character of a profile. Points and
 * history stay. Nothing is saved before « Enregistrer »; « Annuler » leaves everything as it was.
 */
export function EditProfileScreen() {
  const { profileId = "" } = useParams();
  const [search] = useSearchParams();
  const profiles = useProfiles();
  const profile = profiles.data?.find((p) => p.id === profileId);
  const back = backPath(search.get("retour"));

  if (profiles.isLoading) {
    return (
      <main className="centered">
        <p className="message">{fr.app.loading}</p>
      </main>
    );
  }

  if (!profile) {
    return (
      <main className="centered">
        <p className="message">{fr.app.error}</p>
      </main>
    );
  }

  return <EditProfile key={profile.id} profile={profile} back={back} />;
}

function EditProfile({ profile, back }: { profile: Profile; back: string }) {
  const api = useApi();
  const queryClient = useQueryClient();
  const navigate = useNavigate();
  const characters = useCharacters();
  const [name, setName] = useState(profile.name);
  const [grade, setGrade] = useState<Grade>(profile.grade);
  const [characterId, setCharacterId] = useState(profile.characterId);

  // Drawn on screen only: saved with the rest, so that « Annuler » changes nothing.
  const draw = useMutation({ mutationFn: () => api.drawCharacter(characterId), onSuccess: (character) => setCharacterId(character.id) });

  const save = useMutation({
    mutationFn: () => api.updateProfile(profile.id, { name, grade, characterId }),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: queryKeys.profiles });
      await navigate(back);
    },
  });

  const submit = (event: FormEvent) => {
    event.preventDefault();
    save.mutate();
  };

  const error = save.error instanceof ApiError && save.error.status === 400 ? save.error.detail : save.isError ? fr.app.error : null;

  return (
    <main className="screen">
      <header className="screen-header">
        <h1 className="title">{fr.profiles.editTitle(profile.name)}</h1>
        <SpeakButton text={fr.profiles.editSpoken} autoPlay />
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

          <fieldset className="character-picker">
            <legend className="field-label">{fr.profiles.characterLabel}</legend>
            <div className="character-choices">
              {(characters.data ?? []).map((character) => (
                <button
                  key={character.id}
                  type="button"
                  className="character-choice"
                  aria-pressed={characterId === character.id}
                  aria-label={character.name}
                  onClick={() => setCharacterId(character.id)}
                >
                  <CharacterImage id={character.id} size={72} />
                </button>
              ))}
            </div>
          </fieldset>
        </div>

        <div className="create-character">
          <CharacterImage id={characterId} size={208} alt={characters.data?.find((c) => c.id === characterId)?.name ?? ""} />
          <button type="button" className="button" onClick={() => draw.mutate()} disabled={draw.isPending}>
            <DiceIcon size={24} />
            {fr.profiles.randomCharacter}
          </button>
        </div>

        <div className="create-actions">
          <button type="button" className="button" onClick={() => void navigate(back)}>
            {fr.profiles.cancel}
          </button>
          <button type="submit" className="button button-primary" disabled={save.isPending}>
            {fr.profiles.save}
          </button>
        </div>
      </form>
    </main>
  );
}
