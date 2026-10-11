import { createContext, useContext, useMemo, useState, type ReactNode } from "react";

type ProfileSession = {
  /** Id of the profile chosen on the first screen, for this session only. */
  profileId: string | null;
  setProfileId(id: string | null): void;
};

const ProfileContext = createContext<ProfileSession | null>(null);

export function ProfileProvider({ children, initialProfileId = null }: { children: ReactNode; initialProfileId?: string | null }) {
  const [profileId, setProfileId] = useState<string | null>(initialProfileId);
  const value = useMemo(() => ({ profileId, setProfileId }), [profileId]);
  return <ProfileContext.Provider value={value}>{children}</ProfileContext.Provider>;
}

export function useProfileSession(): ProfileSession {
  const session = useContext(ProfileContext);
  if (!session) {
    throw new Error("useProfileSession must be used below ProfileProvider.");
  }

  return session;
}
