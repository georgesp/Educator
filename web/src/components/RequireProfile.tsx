import type { ReactNode } from "react";
import { Navigate } from "react-router";
import type { Profile } from "../api/client";
import { useCurrentProfile } from "../app/queries";
import { fr } from "../i18n/fr";

/** Renders its children with the session's profile; without one, back to the profile choice. */
export function RequireProfile({ children }: { children: (profile: Profile) => ReactNode }) {
  const { profile, isLoading } = useCurrentProfile();
  if (isLoading) {
    return (
      <main className="centered">
        <p className="message">{fr.app.loading}</p>
      </main>
    );
  }

  return profile ? children(profile) : <Navigate to="/" replace />;
}
