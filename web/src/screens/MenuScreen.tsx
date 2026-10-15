import { useRef, type KeyboardEvent, type ReactNode } from "react";
import { Link, useNavigate } from "react-router";
import type { Profile } from "../api/client";
import { useSpeech } from "../audio/useSpeech";
import { usePlacementStatus, useProfileOverview } from "../app/queries";
import { ProfileAvatar } from "../components/child/AvatarImage";
import { BigButton } from "../components/child/BigButton";
import { PointsBadge } from "../components/child/PointsBadge";
import { DailyGoalCard, LevelGauge } from "../components/child/Rewards";
import { SpeakButton } from "../components/child/SpeakButton";
import { BookIcon, FlagIcon, GearIcon, LockIcon, MapIcon, MedalIcon, PencilIcon } from "../components/icons";
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

/** Home menu (SUB000104): vertical menu on the left, the profile panel on the right. */
export function MenuScreen() {
  return <RequireProfile>{(profile) => <Menu profile={profile} />}</RequireProfile>;
}

function Menu({ profile }: { profile: Profile }) {
  const navigate = useNavigate();
  const speak = useSpeech();
  const items = useRef<(HTMLButtonElement | null)[]>([]);
  // The level assessment stays offered until it is done (or the child leaves it for later).
  const placement = usePlacementStatus(profile.id);
  const placementToDo = placement.data?.subjects.some((s) => s.available) ?? false;
  // Lot 3: level, goal of the day, badges.
  const overview = useProfileOverview(profile.id).data;

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

  return (
    <main className="menu-screen">
      <nav className="menu-nav" aria-label={fr.app.title} onKeyDown={onKeyDown}>
        <div className="menu-heading">
          <h1 className="title">{fr.menu.greeting(profile.name)}</h1>
          <SpeakButton text={fr.menu.spoken(profile.name)} compact />
        </div>
        {placementToDo && (
          <BigButton
            className="menu-banner"
            label={fr.placement.menuBanner}
            hint={fr.placement.menuBannerHint}
            icon={<FlagIcon />}
            variant="outline"
            onClick={() => void navigate("/evaluation")}
          />
        )}
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
          />
        ))}
      </nav>

      <aside className="profile-panel" aria-label={profile.name}>
        <Link className="profile-panel-avatar" to="/avatar" aria-label={fr.avatar.open}>
          <ProfileAvatar profile={profile} size={124} />
          <span className="profile-panel-avatar-hint" aria-hidden="true">
            {fr.avatar.dress}
          </span>
        </Link>
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
        {overview && <LevelGauge level={overview.level} />}
        {overview && <DailyGoalCard goal={overview.goal} />}
        <div className="profile-links">
          <Link className="button profile-link" to="/carte">
            <MapIcon size={22} /> {fr.menu.map}
          </Link>
          <Link className="button profile-link" to="/badges">
            <MedalIcon size={22} /> {overview ? fr.menu.badgesCount(overview.badgesEarned, overview.badgesTotal) : fr.menu.badges}
          </Link>
        </div>
      </aside>
    </main>
  );
}
