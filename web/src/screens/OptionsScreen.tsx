import { Link, useNavigate } from "react-router";
import type { ProfileSettings, TextSize, Theme, VoiceSpeed } from "../api/client";
import type { Profile } from "../api/client";
import { useUpdateSettings } from "../app/ProfileSettingsEffect";
import { useProfiles, useProfileSettings, useSettingsOptions } from "../app/queries";
import { ProfileAvatar } from "../components/child/AvatarImage";
import { LockIcon, PencilIcon } from "../components/icons";
import { RequireProfile } from "../components/RequireProfile";
import { fr } from "../i18n/fr";

const VOICE_SPEEDS: readonly VoiceSpeed[] = ["Slower", "Normal", "Faster"];
const TEXT_SIZES: readonly TextSize[] = ["Normal", "Large", "Larger"];

/**
 * Options (SUB001416, SUB001512, SUB001513): the settings of the child (sounds, mascot, voice, text,
 * font, theme), the profiles (change one) and the way to the parents' space. Every change is saved at
 * once in the child's settings and shown at once.
 */
export function OptionsScreen() {
  return <RequireProfile>{(profile) => <Options profile={profile} />}</RequireProfile>;
}

function Options({ profile }: { profile: Profile }) {
  const navigate = useNavigate();
  const profiles = useProfiles();
  const settings = useProfileSettings(profile.id).data;
  const options = useSettingsOptions().data;
  const update = useUpdateSettings(profile.id);

  return (
    <main className="screen options-screen">
      <header className="screen-header">
        <button type="button" className="back-button" onClick={() => void navigate("/menu")}>
          ← {fr.series.menu}
        </button>
        <h1 className="title">{fr.options.title}</h1>
      </header>

      {settings && (
        <>
          <section className="options-section" aria-labelledby="options-sounds">
            <h2 id="options-sounds" className="section-title">
              {fr.options.sounds}
            </h2>
            <label className="option-switch">
              <input type="checkbox" checked={!settings.sound} onChange={() => update.mutate({ sound: !settings.sound })} />
              <span>{fr.options.mute}</span>
            </label>
            <label className="option-switch">
              <input type="checkbox" checked={!settings.mascot} onChange={() => update.mutate({ mascot: !settings.mascot })} />
              <span>{fr.options.hideMascot}</span>
            </label>
            <Choice
              legend={fr.options.voiceSpeed}
              values={VOICE_SPEEDS}
              selected={settings.voiceSpeed}
              label={(v) => fr.options.voiceSpeeds[v]}
              onChoose={(voiceSpeed) => update.mutate({ voiceSpeed })}
            />
          </section>

          <section className="options-section" aria-labelledby="options-reading">
            <h2 id="options-reading" className="section-title">
              {fr.options.reading}
            </h2>
            <Choice
              legend={fr.options.textSize}
              values={TEXT_SIZES}
              selected={settings.textSize}
              label={(v) => fr.options.textSizes[v]}
              onChoose={(textSize) => update.mutate({ textSize })}
            />
            <label className="option-switch">
              <input type="checkbox" checked={settings.dyslexiaFont} onChange={() => update.mutate({ dyslexiaFont: !settings.dyslexiaFont })} />
              <span>{fr.options.dyslexiaFont}</span>
            </label>
            <p className="parent-hint">{fr.options.dyslexiaFontHint}</p>
          </section>

          {options && options.themes.length > 1 && <ThemeChoice themes={options.themes} settings={settings} onChoose={(theme) => update.mutate({ theme })} />}
        </>
      )}

      <section className="options-section" aria-labelledby="options-profiles">
        <h2 id="options-profiles" className="section-title">
          {fr.options.profiles}
        </h2>
        <ul className="plain-list options-profiles">
          {profiles.data?.map((p) => (
            <li key={p.id} className="options-profile">
              <ProfileAvatar profile={p} size={56} />
              <span className="options-profile-name">
                {p.name} · {fr.grades[p.grade]}
              </span>
              <Link className="button" to={`/profil/${encodeURIComponent(p.id)}?retour=options`} aria-label={fr.profiles.editLabel(p.name)}>
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

/** A setting with a few values, as pressed buttons. */
export function Choice<T extends string | number>({
  legend,
  values,
  selected,
  label,
  onChoose,
}: {
  legend: string;
  values: readonly T[];
  selected: T;
  label(value: T): string;
  onChoose(value: T): void;
}) {
  return (
    <div className="option-choice" role="group" aria-label={legend}>
      <span className="option-choice-legend">{legend}</span>
      <div className="option-choice-values">
        {values.map((value) => (
          <button key={value} type="button" className="family-filter-button" aria-pressed={value === selected} onClick={() => onChoose(value)}>
            {label(value)}
          </button>
        ))}
      </div>
    </div>
  );
}

/** « Thème » (SUB001513): a thumbnail of the menu in each theme; applied at once. */
function ThemeChoice({ themes, settings, onChoose }: { themes: Theme[]; settings: ProfileSettings; onChoose(theme: string): void }) {
  return (
    <section className="options-section" aria-labelledby="options-theme">
      <h2 id="options-theme" className="section-title">
        {fr.options.theme}
      </h2>
      <ul className="plain-list theme-list">
        {themes.map((theme) => (
          <li key={theme.id}>
            <button type="button" className="theme-card" aria-pressed={settings.theme === theme.id} onClick={() => onChoose(theme.id)}>
              <span className="theme-thumbnail" data-theme={theme.id} aria-hidden="true">
                <span className="theme-thumbnail-strip" />
                <span className="theme-thumbnail-title">{fr.app.title}</span>
                <span className="theme-thumbnail-button theme-thumbnail-primary" />
                <span className="theme-thumbnail-button" />
                <span className="theme-thumbnail-button" />
              </span>
              <span className="theme-card-name">{theme.name}</span>
              <span className="theme-card-description">{theme.description}</span>
            </button>
          </li>
        ))}
      </ul>
    </section>
  );
}
