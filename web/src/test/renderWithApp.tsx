import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { render } from "@testing-library/react";
import { createMemoryRouter, RouterProvider } from "react-router";
import { vi } from "vitest";
import type { Profile, Series } from "../api/client";
import type { UiApi } from "../api/ui";
import { BackendProvider } from "../app/BackendContext";
import { ProfileProvider } from "../app/ProfileContext";
import { screenRoutes } from "../app/routes";

export const lea: Profile = {
  id: "p-lea",
  name: "Léa",
  grade: "CP",
  characterId: "lion",
  points: 120,
  lastUsedAt: "2026-10-11T08:00:00Z",
};

/** Series of 2 exercises: 3 + 4 = ? (answer 7), then 5 + ? = 9 (answer 4). */
export const twoExercises: Series = {
  seriesId: "s1",
  seed: 1,
  skillId: "cp-add-sum-10",
  instruction: "Calcule.",
  answerMode: "Keyboard",
  exercises: [
    { index: 0, left: 3, operator: "+", right: 4, result: null, hidden: "Result", prompt: "3 + 4 = ?", spokenPrompt: "Combien font 3 plus 4 ?", choices: null },
    { index: 1, left: 5, operator: "+", right: null, result: 9, hidden: "Right", prompt: "5 + ? = 9", spokenPrompt: "Combien faut-il ajouter à 5 pour obtenir 9 ?", choices: null },
  ],
};

/** A fake UI API: every call is a vi.fn, overridable per test. Speech fails (no voice in tests). */
export function fakeApi(overrides: Partial<UiApi> = {}): UiApi {
  const expected = [7, 4];
  return {
    getStatus: vi.fn().mockResolvedValue({ version: "0.1.0", database: { isReady: true }, speech: { isAvailable: false, voice: null, problem: "tests" } }),
    getSkills: vi.fn().mockResolvedValue({
      grade: "CP",
      skillsGrade: "CP",
      currentPeriod: 1,
      skills: [
        { id: "cp-add-sum-10", title: "Additions jusqu'à 10", instruction: "Calcule.", period: 1, isCurrentPeriod: true, revisionFrom: null, answerModes: ["Keyboard", "Choice"] },
        { id: "cp-add-tables", title: "Tables d'addition", instruction: "Calcule.", period: 2, isCurrentPeriod: false, revisionFrom: null, answerModes: ["Keyboard"] },
      ],
    }),
    getCharacters: vi.fn().mockResolvedValue([
      { id: "lion", name: "Lion", kind: "Animal" },
      { id: "ours", name: "Ours", kind: "Animal" },
    ]),
    drawCharacter: vi.fn(async (except?: string) => (except === "lion" ? { id: "ours", name: "Ours", kind: "Animal" } : { id: "lion", name: "Lion", kind: "Animal" })),
    getCharacterSvg: vi.fn().mockResolvedValue("<svg xmlns='http://www.w3.org/2000/svg'/>"),
    listProfiles: vi.fn().mockResolvedValue([lea]),
    createProfile: vi.fn(async (name: string, grade, characterId?: string) => ({ ...lea, id: "p-new", name, grade, points: 0, characterId: characterId ?? "lion" })),
    updateProfile: vi.fn(async (id: string, changes) => ({ ...lea, id, ...changes })),
    selectProfile: vi.fn().mockResolvedValue(lea),
    startSeries: vi.fn().mockResolvedValue(twoExercises),
    answerSeries: vi.fn(async (_s: string, index: number, answer: number) => ({
      correct: answer === expected[index],
      expected: expected[index],
      explanation: index === 0 ? "3 + 4 = 7" : "5 + 4 = 9",
      summary: index === 1 ? { correctCount: answer === 4 ? 2 : 1, exerciseCount: 2, pointsEarned: 40, profilePoints: 160 } : null,
    })),
    abandonSeries: vi.fn().mockResolvedValue({ correctCount: 0, exerciseCount: 2, pointsEarned: 0, profilePoints: 120 }),
    speak: vi.fn().mockRejectedValue(new Error("no voice in tests")),
    ...overrides,
  } as UiApi;
}

/** Renders the application screens at `path`, with the fake API and (optionally) a chosen profile. */
export function renderApp(api: UiApi, path: string, profileId: string | null = lea.id) {
  const router = createMemoryRouter(screenRoutes, { initialEntries: [path] });
  render(
    <QueryClientProvider client={new QueryClient({ defaultOptions: { queries: { retry: false } } })}>
      <BackendProvider api={api}>
        <ProfileProvider initialProfileId={profileId}>
          <RouterProvider router={router} />
        </ProfileProvider>
      </BackendProvider>
    </QueryClientProvider>,
  );
  return router;
}
