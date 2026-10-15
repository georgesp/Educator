import { expect, test, type Page } from "@playwright/test";

/** The words of a dictation replayed with the fixed seed 42 (CP, « le son [ou] »). */
const WORDS = ["loup", "poule", "roue", "cou", "jour", "soupe", "route", "mouton", "bijou", "écoute"];

const profile = { id: "p1", name: "Léa", grade: "CP", characterId: "lion", points: 0, placementSeconds: 20, examSeconds: 0, lastUsedAt: "2026-10-11T08:00:00.000Z" };

/** The child's word letter by letter against the expected one (same alignment as the core, for the cases of the test). */
function correction(expected: string, given: string) {
  const letters = [];
  const length = Math.max(expected.length, given.length);
  for (let i = 0; i < length; i++) {
    const e = expected[i] ?? null;
    const g = given[i] ?? null;
    letters.push({ kind: e === null ? "Extra" : g === null ? "Missing" : e === g ? "Right" : "Wrong", given: g, expected: e });
  }
  return { correct: expected === given, letters };
}

async function mockApi(page: Page, calls: string[]) {
  const results = new Map<string, boolean[]>();
  const commands = async (command: string, args: Record<string, unknown> = {}): Promise<unknown> => {
    calls.push(command);
    switch (command) {
      case "get_startup":
        return { state: "ready" };
      case "get_status":
        return { version: "0.1.0", database: { isReady: true }, speech: { isAvailable: true, voice: "fr_FR-siwis-medium", problem: null } };
      case "get_characters":
        return [{ id: "lion", name: "Lion", kind: "Animal" }];
      case "get_character_svg":
        return "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 256 256'><circle cx='128' cy='128' r='100' fill='orange'/></svg>";
      case "list_profiles":
        return [profile];
      case "select_profile":
        return profile;
      case "get_skills":
        expect(args.subject).toBe("Spelling");
        return {
          subject: "Spelling",
          grade: "CP",
          skillsGrade: "CP",
          currentPeriod: 2,
          skills: [
            {
              id: "cp-son-ou",
              title: "Le son [ou]",
              instruction: "Écoute bien le mot, puis écris-le.",
              period: 2,
              isCurrentPeriod: true,
              revisionFrom: null,
              answerModes: ["Keyboard", "Choice"],
              family: "Spelling",
              category: "LettersAndSounds",
              spellingFormats: ["Dictation", "MissingLetters", "ChooseSpelling"],
              listeningFormats: ["Dictation", "ChooseSpelling"],
              mastery: null,
            },
          ],
        };
      case "start_series":
      case "start_exam": {
        const exam = command === "start_exam";
        if (!exam) {
          expect(args.request).toMatchObject({ skillId: "cp-son-ou", answerMode: "Keyboard", format: "Dictation" });
        }
        const words = exam ? WORDS.slice(0, 3) : WORDS;
        return {
          seriesId: exam ? "exam" : "dictation",
          sessionType: exam ? "Exam" : "Exercise",
          subject: "Spelling",
          seed: 42,
          skillId: exam ? null : "cp-son-ou",
          instruction: exam ? "" : "Écoute bien le mot, puis écris-le.",
          answerMode: "Keyboard",
          timeLimitSeconds: null,
          exercises: words.map((_, index) => ({
            index,
            question: "Écoute le mot et écris-le.",
            statement: [],
            picture: null,
            input: { kind: "spelling", maxLength: 16 },
            choices: null,
            prompt: "Dictée",
            spokenPrompt: "",
            word: { before: null, after: null, missing: null, hasExample: true, article: null },
          })),
        };
      }
      case "answer_series": {
        const { seriesId, index, answer } = args as { seriesId: string; index: number; answer: { kind: string; value: string } };
        const words = seriesId === "exam" ? WORDS.slice(0, 3) : WORDS;
        const expected = words[index];
        const given = answer.value.trim().toLowerCase();
        const right = given === expected;
        const done = [...(results.get(seriesId) ?? []), right];
        results.set(seriesId, done);
        const last = index === words.length - 1;
        const correct = done.filter(Boolean).length;
        return {
          correct: right,
          expected: { kind: "text", value: expected },
          expectedLabel: expected,
          explanation: `On écrit « ${expected} » : ici, le son ou s'écrit « ou ».`,
          help: null,
          correction: correction(expected, given),
          summary: last
            ? {
                correctCount: correct,
                exerciseCount: words.length,
                pointsEarned: correct * 10 + 20,
                profilePoints: correct * 10 + 20,
                skills: [{ skillId: "cp-son-ou", title: "Le son [ou]", correctCount: correct, questionCount: words.length, lessonId: "cp-son-ou", strong: correct * 3 >= words.length * 2 }],
                mascotSituation: correct === words.length ? "SeriesPerfect" : "SeriesWell",
              }
            : null,
        };
      }
      case "speak":
      case "speak_series_word":
        return Array.from({ length: 44 }, () => 0);
      case "get_mascot":
      case "get_mascot_phrase":
        return null;
      case "get_placement_status":
        return { subjects: [{ subject: "Math", available: false, done: true }, { subject: "Spelling", available: false, done: true }], questionSeconds: 20 };
      default:
        throw { status: 404, title: "Unknown command", detail: command, field: null };
    }
  };

  await page.exposeFunction("__educatorE2eInvoke", async (command: string, args: Record<string, unknown>) => {
    try {
      return { ok: true, value: await commands(command, args) };
    } catch (error) {
      return { ok: false, error };
    }
  });
  await page.addInitScript(() => {
    type Answer = { ok: boolean; value?: unknown; error?: unknown };
    const call = (window as unknown as { __educatorE2eInvoke: (c: string, a: unknown) => Promise<Answer> }).__educatorE2eInvoke;
    (window as unknown as { __TAURI_INTERNALS__: unknown }).__TAURI_INTERNALS__ = {
      invoke: async (command: string, args: unknown) => {
        const answer = await call(command, args ?? {});
        if (!answer.ok) {
          throw answer.error;
        }
        return answer.value;
      },
      transformCallback: () => 0,
      metadata: { currentWindow: { label: "main" }, currentWebview: { windowLabel: "main", label: "main" } },
    };
  });
}

test("a CP child does a dictation of 10 words with the mouse only (fixed seed), one mistake corrected letter by letter", async ({ page }) => {
  test.setTimeout(90_000);
  const calls: string[] = [];
  await mockApi(page, calls);
  await page.goto("/");

  await page.getByRole("button", { name: "Jouer avec le profil de Léa" }).click();
  await page.getByRole("button", { name: /Exercices/ }).click();
  await page.getByRole("button", { name: /Orthographe/ }).click();
  await page.getByRole("button", { name: /Le son \[ou\]/ }).click();
  await page.getByRole("button", { name: /J'écoute le mot et je l'écris/ }).click();

  const keyboard = page.getByRole("group", { name: "Clavier des lettres" });
  for (const [i, word] of WORDS.entries()) {
    await expect(page.getByLabel("Écris le mot")).toBeVisible();
    // Nothing is read by itself: the child clicks to hear the word.
    await page.getByRole("button", { name: "Écouter le mot" }).click();
    // The 10th word is written without its accent: « ecoute ».
    const written = i === 9 ? "ecoute" : word;
    for (const letter of written) {
      await keyboard.getByRole("button", { name: `Lettre ${letter}`, exact: true }).click();
    }
    await keyboard.getByRole("button", { name: "Valider" }).click();
    if (i === 9) {
      await expect(page.getByText("Tu as écrit :")).toBeVisible();
      await expect(page.getByTitle("Il fallait « é » au lieu de « e »")).toBeVisible();
      await expect(page.locator(".word-right-text")).toHaveText("écoute");
      await page.getByRole("button", { name: "Suivant" }).click();
    } else {
      await expect(page.getByRole("status").first()).toBeVisible();
    }
  }

  await expect(page.getByRole("heading", { name: "Série terminée !" })).toBeVisible();
  await expect(page.getByText("9 bonnes réponses sur 10")).toBeVisible();
  // Each word was read by the voice on click, once (French exercises).
  expect(calls.filter((c) => c === "speak_series_word").length).toBe(10);
});

test("the exam of spelling ends with the strong points", async ({ page }) => {
  test.setTimeout(60_000);
  await mockApi(page, []);
  await page.goto("/");

  await page.getByRole("button", { name: "Jouer avec le profil de Léa" }).click();
  await page.getByRole("button", { name: /Examen/ }).click();
  await page.getByRole("button", { name: /Bilan d'orthographe/ }).click();
  for (const word of WORDS.slice(0, 3)) {
    await page.getByLabel("Écris le mot").fill(word);
    await page.getByLabel("Écris le mot").press("Enter");
    await expect(page.getByRole("status").first()).toBeVisible();
  }

  await expect(page.getByRole("heading", { name: "Bilan terminé !" })).toBeVisible();
  await expect(page.getByRole("region", { name: "Tes points forts" })).toContainText("Le son [ou]");
  await expect(page.getByRole("img", { name: "Bravo, tout est juste !" })).toBeVisible();
});
