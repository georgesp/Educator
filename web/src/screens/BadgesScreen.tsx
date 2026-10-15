import { useQuery } from "@tanstack/react-query";
import { useNavigate } from "react-router";
import type { Profile } from "../api/client";
import { useApi } from "../app/BackendContext";
import { queryKeys } from "../app/queries";
import { BadgeIcon } from "../components/child/Rewards";
import { SpeakButton } from "../components/child/SpeakButton";
import { RequireProfile } from "../components/RequireProfile";
import { fr } from "../i18n/fr";

/**
 * « Mes badges » (SUB001503): the badges earned in color first, then those to win in grey with their
 * condition said simply.
 */
export function BadgesScreen() {
  return <RequireProfile>{(profile) => <Badges profile={profile} />}</RequireProfile>;
}

function Badges({ profile }: { profile: Profile }) {
  const api = useApi();
  const navigate = useNavigate();
  const badges = useQuery({ queryKey: queryKeys.badges(profile.id), queryFn: () => api.getBadges(profile.id), staleTime: 0 });
  const earned = badges.data?.filter((b) => b.earned) ?? [];
  const toWin = badges.data?.filter((b) => !b.earned) ?? [];

  return (
    <main className="screen badges-screen">
      <header className="screen-header">
        <button type="button" className="back-button" onClick={() => void navigate("/menu")}>
          ← {fr.series.menu}
        </button>
        <h1 className="title">{fr.badges.title}</h1>
        <SpeakButton text={fr.badges.spoken(earned.length)} />
      </header>
      {badges.isLoading && <p className="message">{fr.app.loading}</p>}
      {badges.isError && <p className="message">{fr.app.error}</p>}

      {badges.data && (
        <>
          <section className="badges-section" aria-labelledby="badges-earned">
            <h2 id="badges-earned" className="section-title">
              {fr.badges.earned(earned.length)}
            </h2>
            {earned.length === 0 ? (
              <p className="notice">{fr.badges.noneYet}</p>
            ) : (
              <ul className="plain-list badge-grid">
                {earned.map((badge) => (
                  <li key={badge.id} className="badge-card badge-card-earned">
                    <BadgeIcon badge={badge} size={96} />
                    <span className="badge-title">{badge.title}</span>
                    {badge.earnedAt && <span className="badge-date">{fr.badges.earnedOn(badge.earnedAt)}</span>}
                  </li>
                ))}
              </ul>
            )}
          </section>
          {toWin.length > 0 && (
            <section className="badges-section" aria-labelledby="badges-to-win">
              <h2 id="badges-to-win" className="section-title">
                {fr.badges.toWin}
              </h2>
              <ul className="plain-list badge-grid">
                {toWin.map((badge) => (
                  <li key={badge.id} className="badge-card">
                    <BadgeIcon badge={badge} size={72} />
                    <span className="badge-title">{badge.title}</span>
                    <span className="badge-condition">{badge.description}</span>
                  </li>
                ))}
              </ul>
            </section>
          )}
        </>
      )}
    </main>
  );
}
