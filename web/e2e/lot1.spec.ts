import { expect, test, type Page } from "@playwright/test";

type Exercise = { index: number; left: number | null; operator: string; right: number | null; result: number | null; hidden: string; prompt: string; spokenPrompt: string; choices: number[] | null };

/** A fixed series of 10 additions a + 1 = ? (answer a + 1), like a series replayed with a fixed seed. */
function series(answerMode: "Keyboard" | "Choice") {
  const exercises: Exercise[] = Array.from({ length: 10 }, (_, i) => ({
    index: i,
    left: i + 1,
    operator: "+",
    right: 1,
    result: null,
    hidden: "Result",
    prompt: `${i + 1} + 1 = ?`,
    spokenPrompt: `Combien font ${i + 1} plus 1 ?`,
    choices: answerMode === "Choice" ? [i + 2, i + 3, i + 12, i + 1].sort((a, b) => a - b) : null,
  }));
  return { seriesId: `series-${answerMode}`, seed: 42, skillId: "cp-add-sum-10", instruction: "Calcule.", answerMode, exercises };
}

/**
 * The application core simulated in the page: the Tauri IPC (`window.__TAURI_INTERNALS__.invoke`) is
 * replaced by a function running here, in the test, that answers the commands like src-tauri/src/commands.rs.
 */
async function mockApi(page: Page, profiles: Record<string, unknown>[] = []) {
  const answered = new Map<string, number>();
  const commands = async (command: string, args: Record<string, unknown> = {}): Promise<unknown> => {
    switch (command) {
      case "get_startup":
        return { state: "ready" };
      case "get_status":
        return { version: "0.1.0", database: { isReady: true }, speech: { isAvailable: true, voice: "fr_FR-siwis-medium", problem: null } };
      case "get_characters":
        return [{ id: "lion", name: "Lion", kind: "Animal" }, { id: "ours", name: "Ours", kind: "Animal" }];
      case "draw_character":
        return { id: "lion", name: "Lion", kind: "Animal" };
      case "get_character_svg":
        return "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 256 256'><circle cx='128' cy='128' r='100' fill='orange'/></svg>";
      case "list_profiles":
        return profiles;
      case "create_profile": {
        const { name, grade } = args.request as { name: string; grade: string };
        const profile = { id: "p1", name, grade, characterId: "lion", points: 0, lastUsedAt: "2026-10-11T08:00:00.000Z" };
        profiles.push(profile);
        return profile;
      }
      case "update_profile":
        Object.assign(profiles[0], args.changes);
        return profiles[0];
      case "select_profile":
      case "reroll_character":
        return profiles[0];
      case "get_skills":
        expect(args.grade).toBe("CP");
        return {
          grade: "CP",
          skillsGrade: "CP",
          currentPeriod: 1,
          skills: [{ id: "cp-add-sum-10", title: "Additions jusqu'à 10", instruction: "Calcule.", period: 1, isCurrentPeriod: true, answerModes: ["Keyboard", "Choice"] }],
        };
      case "start_series": {
        const { answerMode } = args.request as { answerMode: "Keyboard" | "Choice" };
        return series(answerMode);
      }
      case "answer_series": {
        const { seriesId, index, answer } = args as { seriesId: string; index: number; answer: number };
        const expected = index + 2;
        const correct = answer === expected;
        answered.set(seriesId, (answered.get(seriesId) ?? 0) + (correct ? 1 : 0));
        const done = index === 9;
        const count = answered.get(seriesId) ?? 0;
        const earned = count * 10 + 20 + (count === 10 ? 20 : 0);
        if (done) {
          answered.delete(seriesId);
          (profiles[0] as { points: number }).points += earned;
        }
        return {
          correct,
          expected,
          explanation: `${index + 1} + 1 = ${expected}`,
          summary: done ? { correctCount: count, exerciseCount: 10, pointsEarned: earned, profilePoints: (profiles[0] as { points: number }).points } : null,
        };
      }
      case "speak":
        return Array.from({ length: 44 }, () => 0);
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

test("a CP child creates a profile, then does a whole series with the keyboard, then with the QCM", async ({ page }) => {
  await mockApi(page);
  await page.goto("/");

  // No profile yet: the creation shows at once.
  await page.getByLabel("Ton prénom").fill("Léa");
  await page.getByRole("button", { name: "CP" }).click();
  await page.getByRole("button", { name: "C'est parti !" }).click();

  await expect(page.getByRole("heading", { name: "En piste, Léa !" })).toBeVisible();
  await page.getByRole("button", { name: /Exercices/ }).click();
  await expect(page.getByText("C'est ce que tu travailles en ce moment")).toBeVisible();
  await page.getByRole("button", { name: /Additions jusqu'à 10/ }).click();
  await page.getByRole("button", { name: /J'écris la réponse/ }).click();

  // Keyboard mode, keyboard only: type and Enter; one mistake on the 3rd exercise.
  for (let i = 0; i < 10; i++) {
    await expect(page.getByLabel(`${i + 1} + 1 = ?`)).toBeVisible();
    const input = page.getByLabel("Ta réponse");
    await input.fill(String(i === 2 ? 99 : i + 2));
    await input.press("Enter");
    if (i === 2) {
      await expect(page.getByText("3 + 1 = 4")).toBeVisible();
      await page.getByRole("button", { name: "Suivant" }).click();
    }
  }

  await expect(page.getByRole("heading", { name: "Série terminée !" })).toBeVisible();
  await expect(page.getByText("9 bonnes réponses sur 10")).toBeVisible();
  await expect(page.getByText("+110 points")).toBeVisible();

  // Back to the menu: points are up to date. Then a QCM series, mouse only.
  await page.locator(".summary").getByRole("button", { name: "Menu" }).click();
  await expect(page.getByLabel("110 points")).toBeVisible();
  await page.getByRole("button", { name: /Exercices/ }).click();
  await page.getByRole("button", { name: /Additions jusqu'à 10/ }).click();
  await page.getByRole("button", { name: /Je choisis la réponse/ }).click();
  for (let i = 0; i < 10; i++) {
    await expect(page.getByLabel(`${i + 1} + 1 = ?`)).toBeVisible();
    await page.getByRole("group", { name: "Choisis la bonne réponse" }).getByRole("button", { name: String(i + 2), exact: true }).click();
  }

  await expect(page.getByText("10 bonnes réponses sur 10")).toBeVisible();
  await expect(page.getByText("+140 points")).toBeVisible();
});

test("an existing profile is chosen on the first screen", async ({ page }) => {
  await mockApi(page, [{ id: "p1", name: "Tom", grade: "CP", characterId: "ours", points: 30, lastUsedAt: "2026-10-10T08:00:00Z" }]);
  await page.goto("/");

  await expect(page.getByRole("heading", { name: "Qui joue ?" })).toBeVisible();
  await page.getByRole("button", { name: "Jouer avec le profil de Tom" }).click();

  await expect(page.getByRole("heading", { name: "En piste, Tom !" })).toBeVisible();
  await expect(page.getByLabel("30 points")).toBeVisible();
});
