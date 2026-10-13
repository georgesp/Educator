import { useQuery } from "@tanstack/react-query";
import type { Grade, Profile, Subject } from "../api/client";
import { useApi } from "./BackendContext";
import { useProfileSession } from "./ProfileContext";

export const queryKeys = {
  profiles: ["profiles"] as const,
  characters: ["characters"] as const,
  characterSvg: (id: string) => ["character-svg", id] as const,
  skills: (grade: Grade, subject: Subject = "Math", profileId?: string) => ["skills", grade, subject, profileId ?? null] as const,
};

export function useProfiles() {
  const api = useApi();
  return useQuery({ queryKey: queryKeys.profiles, queryFn: () => api.listProfiles() });
}

/** The profile chosen for this session, once loaded; undefined while loading or when none is chosen. */
export function useCurrentProfile(): { profile: Profile | undefined; isLoading: boolean } {
  const { profileId } = useProfileSession();
  const profiles = useProfiles();
  return { profile: profiles.data?.find((p) => p.id === profileId), isLoading: profiles.isLoading };
}

export function useCharacters() {
  const api = useApi();
  return useQuery({ queryKey: queryKeys.characters, queryFn: () => api.getCharacters(), staleTime: Infinity });
}

/**
 * Skills of a subject offered to a grade (the core falls back to CE1 for the grades without skills
 * yet), with the profile's mastery when `profileId` is given.
 */
export function useSkills(grade: Grade, subject: Subject = "Math", profileId?: string) {
  const api = useApi();
  return useQuery({ queryKey: queryKeys.skills(grade, subject, profileId), queryFn: () => api.getSkills(grade, subject, profileId) });
}
