import { useEffect, useRef, type KeyboardEvent, type ReactNode } from "react";
import { Link, useNavigate } from "react-router";
import type { Profile } from "../api/client";
import { useSpeech } from "../audio/useSpeech";
import { BigButton } from "../components/child/BigButton";
import { CharacterImage } from "../components/child/CharacterImage";
import { PointsBadge } from "../components/child/PointsBadge";
import { SpeakButton } from "../components/child/SpeakButton";
import { BookIcon, FlagIcon, GearIcon, LockIcon, PencilIcon } from "../components/icons";
import { RequireProfile } from "../components/RequireProfile";
import { fr } from "../i18n/fr";

type Entry = { key: string; label: string; hint: string; icon: ReactNode; to?: string };

/** Every entry is open since lot 2 (SUB001416); an entry without `to` stays visible, locked, and says so aloud. */
const ENTRIES: Entry[] = [
  { key: "lessons", label: fr.menu.lessons, hint: fr.menu.lessonsHint, icon: <BookIcon />, to: "/lecons" },
  { key: "exercises", label: fr.menu.exercises, hint: fr.menu.exercisesHint, icon: <PencilIcon />, to: "/exercices" },
  { key: "exam", label: fr.menu.exam, hint: fr.menu.examHint, icon: <FlagIcon />, to: "/examen" },
  { key: "options", label: fr.menu.options, hint: fr.menu.optionsHint, icon: <GearIcon />, to: "/options" },
];

const HOVER_READ_DELAY_MS = 600;

/** Home menu (SUB000104): vertical menu on the left, the profile panel on the right. */
export function MenuScreen() {
  return <RequireProfile>{(profile) => <Menu profile={profile} />}</RequireProfile>;
}

function Menu({ profile }: { profile: Profile }) {
  const navigate = useNavigate();
  const speak = useSpeech();
  const items = useRef<(HTMLButtonElement | null)[]>([]);
  const hoverTimer = useRef<number | undefined>(undefined);
  // No reading after leaving the screen.
  useEffect(() => () => window.clearTimeout(hoverTimer.current), []);

  const activate = (entry: Entry) => {
    if (entry.to) {
      void navigate(entry.to);
    } else {
      void speak(fr.menu.soonSpoken);
    }
  };

  // Arrow keys move between the entries (Enter / Space activate them, as any button).
  const onKeyDown = (event: KeyboardEvent<HTMLElement>) => {
    if (event.key !== "ArrowDown" && event.key !== "ArrowUp") {
      return;
    }

    event.preventDefault();
    const current = items.current.findIndex((item) => item === document.activeElement);
    const step = event.key === "ArrowDown" ? 1 : -1;
    const next = (current + step + ENTRIES.length) % ENTRIES.length;
    items.current[next]?.focus();
  };

  const readOnHover = (entry: Entry) => {
    window.clearTimeout(hoverTimer.current);
    hoverTimer.current = window.setTimeout(() => void speak(entry.to ? entry.label : `${entry.label}. ${fr.menu.soonSpoken}`), HOVER_READ_DELAY_MS);
  };

  return (
    <main className="menu-screen">
      <nav className="menu-nav" aria-label={fr.app.title} onKeyDown={onKeyDown}>
        <div className="menu-heading">
          <h1 className="title">{fr.menu.greeting(profile.name)}</h1>
          <SpeakButton text={fr.menu.spoken(profile.name)} autoPlay compact />
        </div>
        {ENTRIES.map((entry, index) => (
          <BigButton
            key={entry.key}
            ref={(element: HTMLButtonElement | null) => {
              items.current[index] = element;
            }}
            number={String(index + 1).padStart(2, "0")}
            label={entry.label}
            hint={entry.hint}
            icon={entry.to ? entry.icon : <LockIcon />}
            variant={entry.to ? "primary" : "default"}
            locked={!entry.to}
            autoFocus={index === 0}
            onClick={() => activate(entry)}
            onMouseEnter={() => readOnHover(entry)}
            onMouseLeave={() => window.clearTimeout(hoverTimer.current)}
          />
        ))}
      </nav>

      <aside className="profile-panel" aria-label={profile.name}>
        <div className="profile-panel-avatar">
          <CharacterImage id={profile.characterId} size={136} />
        </div>
        <div className="profile-panel-name">
          <span className="profile-panel-title">{profile.name}</span>
          <Link to="/">{fr.menu.changeProfile}</Link>
          <Link className="profile-edit" to={`/profil/${encodeURIComponent(profile.id)}?retour=menu`} aria-label={fr.profiles.editLabel(profile.name)}>
            <PencilIcon size={20} />
            {fr.profiles.edit}
          </Link>
        </div>
        <dl className="profile-stats">
          <div className="profile-stat">
            <dt>{fr.menu.grade}</dt>
            <dd>{fr.grades[profile.grade]}</dd>
          </div>
          <div className="profile-stat">
            <dt>{fr.menu.points}</dt>
            <dd className="profile-points">
              <PointsBadge points={profile.points} />
            </dd>
          </div>
        </dl>
        <div className="profile-level">
          <span className="profile-level-label">{fr.menu.level}</span>
          <span className="profile-level-soon">{fr.menu.levelSoon}</span>
        </div>
      </aside>
    </main>
  );
}
