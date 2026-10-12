import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { render } from "@testing-library/react";
import { createMemoryRouter, RouterProvider } from "react-router";
import { vi } from "vitest";
import type { AnswerPayload, Exercise, Lesson, Profile, Series, Token } from "../api/client";
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

const n = (value: number): Token => ({ kind: "number", value });
const sign = (value: string): Token => ({ kind: "sign", value });
const slot: Token = { kind: "slot", slot: 0 };

/** A calculation typed with the keypad (no question, no picture). */
export function numberExercise(index: number, statement: Token[], prompt: string, spokenPrompt: string): Exercise {
  return { index, question: null, statement, picture: null, input: { kind: "number", slots: 1 }, choices: null, prompt, spokenPrompt };
}

/** Series of 2 exercises: 3 + 4 = ? (answer 7), then 5 + ? = 9 (answer 4). */
export const twoExercises: Series = {
  seriesId: "s1",
  seed: 1,
  skillId: "cp-add-sum-10",
  instruction: "Calcule.",
  answerMode: "Keyboard",
  exercises: [
    numberExercise(0, [n(3), sign("+"), n(4), sign("="), slot], "3 + 4 = ?", "Combien font 3 plus 4 ?"),
    numberExercise(1, [n(5), sign("+"), slot, sign("="), n(9)], "5 + ? = 9", "Combien faut-il ajouter à 5 pour obtenir 9 ?"),
  ],
};

/**
 * A short CE1 lesson: title, a number line where the child clicks 77 then 76 (57 + 19), the matching
 * of 24 + 9 / 24 + 19 to their chains, « Je retiens »; quiz: 28 + 9 (typed, 37), 46 + 19 (QCM, 65).
 */
export const shortLesson: Lesson = {
  id: "ce1-add-9-19-29",
  title: "Ajouter 9, 19 ou 29",
  grade: "CE1",
  skillIds: ["ce1-add-9-19-29"],
  slides: [
    {
      title: "Leçon",
      text: "Une astuce de pilote.",
      speech: "Une astuce de pilote.",
      interactive: false,
      animation: { type: "titleCard", title: "Ajouter 9, 19 ou 29", subtitle: "L'astuce de la dizaine ronde" },
    },
    {
      title: "À toi !",
      text: "57 + 19 en deux bonds.",
      speech: "À toi !",
      interactive: true,
      animation: {
        type: "numberLine",
        from: 55,
        to: 80,
        step: 1,
        start: 57,
        jumps: [
          { delta: 20, value: 77 },
          { delta: -1, value: 76 },
        ],
        interactive: true,
        hoverValues: true,
      },
    },
    {
      title: "À toi !",
      text: "Relie chaque calcul à son chemin.",
      speech: "Relie.",
      interactive: true,
      animation: {
        type: "calcChain",
        mode: "Match",
        chains: [
          { start: 24, steps: [{ delta: 10, value: 34 }, { delta: -1, value: 33 }], total: 9, result: 33, crossed: false },
          { start: 24, steps: [{ delta: 20, value: 44 }, { delta: -1, value: 43 }], total: 19, result: 43, crossed: false },
        ],
      },
    },
    {
      title: "Je retiens",
      text: "J'ajoute la dizaine ronde, puis j'enlève 1.",
      speech: "Je retiens.",
      interactive: false,
      animation: { type: "recapCard", text: "19 = 20 − 1", example: "46 + 19 = 65" },
    },
  ],
  quiz: [
    { index: 0, question: "28 + 9 = ?", spoken: "Combien font 28 + 9 ?", choices: null },
    {
      index: 1,
      question: "46 + 19 = ?",
      spoken: "Combien font 46 + 19 ?",
      choices: [64, 65, 66].map((value) => ({ label: String(value), answer: { kind: "number" as const, value } })),
    },
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
        { id: "cp-add-sum-10", title: "Additions jusqu'à 10", instruction: "Calcule.", period: 1, isCurrentPeriod: true, revisionFrom: null, answerModes: ["Keyboard", "Choice"], family: "Calculation" },
        { id: "cp-add-tables", title: "Tables d'addition", instruction: "Calcule.", period: 2, isCurrentPeriod: false, revisionFrom: null, answerModes: ["Keyboard"], family: "Calculation" },
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
    answerSeries: vi.fn(async (_s: string, index: number, answer: AnswerPayload) => {
      const value = answer.kind === "number" ? answer.value : null;
      return {
        correct: value === expected[index],
        expected: { kind: "number", value: expected[index] },
        expectedLabel: String(expected[index]),
        explanation: index === 0 ? "3 + 4 = 7" : "5 + 4 = 9",
        help: null,
        summary: index === 1 ? { correctCount: value === 4 ? 2 : 1, exerciseCount: 2, pointsEarned: 40, profilePoints: 160 } : null,
      };
    }),
    abandonSeries: vi.fn().mockResolvedValue({ correctCount: 0, exerciseCount: 2, pointsEarned: 0, profilePoints: 120 }),
    getMultiplicationTable: vi.fn(async (number: number) => ({
      number,
      lines: Array.from({ length: 11 }, (_, right) => ({ left: number, right, result: number * right, spoken: `${number} fois ${right}, ${number * right}.` })),
    })),
    listLessons: vi.fn().mockResolvedValue({
      grade: "CP",
      lessonsGrade: "CP",
      currentPeriod: 1,
      lessons: [{ id: shortLesson.id, title: shortLesson.title, period: 2, isCurrentPeriod: false, durationMinutes: 4 }],
    }),
    getLesson: vi.fn().mockResolvedValue(shortLesson),
    answerLessonQuiz: vi.fn(async (_l: string, index: number, answer: AnswerPayload) => {
      const right = [37, 65][index];
      return {
        correct: answer.kind === "number" && answer.value === right,
        expected: { kind: "number", value: right },
        expectedLabel: String(right),
        explanation: index === 0 ? "28 + 10 = 38, puis j'enlève 1 : 28 + 9 = 37." : "46 + 20 = 66, puis j'enlève 1 : 46 + 19 = 65.",
      };
    }),
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
