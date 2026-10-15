import { useQuery, type QueryClient } from "@tanstack/react-query";
import type { Grade, Profile, Subject, WornAccessory } from "../api/client";
import { useApi } from "./BackendContext";
import { useProfileSession } from "./ProfileContext";

export const queryKeys = {
  status: ["status"] as const,
  profiles: ["profiles"] as const,
  characters: ["characters"] as const,
  characterSvg: (id: string) => ["character-svg", id] as const,
  skills: (grade: Grade, subject: Subject = "Math", profileId?: string) => ["skills", grade, subject, profileId ?? null] as const,
  parentProfile: ["parent-profile"] as const,
  /** The level assessment of a profile (which subjects remain). */
  placement: (profileId: string) => ["placement", profileId] as const,
  /** Statistics of a profile in the parents' space. */
  statistics: (profileId: string) => ["profile-statistics", profileId] as const,
  /** Lot 3: level, goal of the day, badges, reviews due (the menu). */
  overview: (profileId: string) => ["profile-overview", profileId] as const,
  badges: (profileId: string) => ["badges", profileId] as const,
  accessories: (profileId: string) => ["accessories", profileId] as const,
  avatarSvg: (characterId: string, accessories: readonly WornAccessory[]) =>
    ["avatar-svg", characterId, accessories.map((a) => `${a.accessoryId}:${a.colorId}`).join(",")] as const,
  advice: (profileId: string, subject: Subject) => ["advice", profileId, subject] as const,
  settings: (profileId: string) => ["profile-settings", profileId] as const,
  settingsOptions: ["settings-options"] as const,
  lessons: (grade: Grade, subject: Subject, profileId?: string) => ["lessons", grade, subject, profileId ?? null] as const,
};

/**
 * After a session: the points, the level, the stars, the goal of the day, the badges, the
 * accessories and the advice may have changed.
 */
export function invalidateProgress(queryClient: QueryClient): Promise<void> {
  return Promise.all(
    [["profiles"], ["profile-overview"], ["skills"], ["lessons"], ["badges"], ["accessories"], ["advice"]].map((queryKey) =>
      queryClient.invalidateQueries({ queryKey }),
    ),
  ).then(() => undefined);
}

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

/** The level assessment of a profile: the subjects still to do. Always read again (staleTime 0). */
export function usePlacementStatus(profileId: string | undefined) {
  const api = useApi();
  return useQuery({
    queryKey: queryKeys.placement(profileId ?? ""),
    queryFn: () => api.getPlacementStatus(profileId!),
    enabled: profileId !== undefined,
    staleTime: 0,
  });
}

/**
 * Whether the voice can read the words (SUB026704): false only once the status says it is
 * unavailable, so that nothing is greyed while the status loads.
 */
export function useVoiceAvailable(): boolean {
  const api = useApi();
  const { data } = useQuery({ queryKey: queryKeys.status, queryFn: () => api.getStatus() });
  return data?.speech.isAvailable ?? true;
}

/** The parents' card of the profile choice (created by the core the first time). */
export function useParentProfile() {
  const api = useApi();
  return useQuery({ queryKey: queryKeys.parentProfile, queryFn: () => api.getParentProfile() });
}

/** Level, goal of the day, badges and reviews due of a profile; always read again. */
export function useProfileOverview(profileId: string | undefined) {
  const api = useApi();
  return useQuery({
    queryKey: queryKeys.overview(profileId ?? ""),
    queryFn: () => api.getProfileOverview(profileId!),
    enabled: profileId !== undefined,
    staleTime: 0,
  });
}

/** The settings of a child (sounds, voice, text, theme, goal of the day…). */
export function useProfileSettings(profileId: string | null | undefined) {
  const api = useApi();
  return useQuery({
    queryKey: queryKeys.settings(profileId ?? ""),
    queryFn: () => api.getProfileSettings(profileId!),
    enabled: !!profileId,
  });
}

/** Themes, goals and durations offered by the settings. */
export function useSettingsOptions() {
  const api = useApi();
  return useQuery({ queryKey: queryKeys.settingsOptions, queryFn: () => api.getSettingsOptions(), staleTime: Infinity });
}

/** Lessons of a subject offered to a grade, with the profile's stars when given. */
export function useLessons(grade: Grade, subject: Subject, profileId?: string) {
  const api = useApi();
  return useQuery({ queryKey: queryKeys.lessons(grade, subject, profileId), queryFn: () => api.listLessons(grade, subject, profileId) });
}
