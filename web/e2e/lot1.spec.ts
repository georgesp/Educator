import { expect, test, type Page } from "@playwright/test";
import { E2E_BACKEND_URL, E2E_TOKEN } from "../playwright.config";

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

/** The UI API simulated in the page: profiles, skills, characters, series and speech. */
async function mockApi(page: Page, profiles: Record<string, unknown>[] = []) {
  const answered = new Map<string, number>();
  await page.route(`${E2E_BACKEND_URL}/api/ui/**`, async (route) => {
    const request = route.request();
    const { pathname, searchParams } = new URL(request.url());
    const path = pathname.replace("/api/ui", "");
    expect(request.headers()["x-local-token"]).toBe(E2E_TOKEN);

    if (path === "/status") {
      return route.fulfill({ json: { version: "0.1.0", database: { isReady: true }, speech: { isAvailable: true, voice: "fr_FR-siwis-medium", problem: null } } });
    }
    if (path === "/characters") {
      return route.fulfill({ json: [{ id: "lion", name: "Lion", kind: "Animal" }, { id: "ours", name: "Ours", kind: "Animal" }] });
    }
    if (path === "/characters/draw") {
      return route.fulfill({ json: { id: "lion", name: "Lion", kind: "Animal" } });
    }
    if (path.startsWith("/characters/")) {
      return route.fulfill({ contentType: "image/svg+xml", body: "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 256 256'><circle cx='128' cy='128' r='100' fill='orange'/></svg>" });
    }
    if (path === "/profiles" && request.method() === "GET") {
      return route.fulfill({ json: profiles });
    }
    if (path === "/profiles" && request.method() === "POST") {
      const { name, grade } = request.postDataJSON() as { name: string; grade: string };
      const profile = { id: "p1", name, grade, characterId: "lion", points: 0, lastUsedAt: "2026-10-11T08:00:00Z" };
      profiles.push(profile);
      return route.fulfill({ status: 201, json: profile });
    }
    if (path.startsWith("/profiles/")) {
      const profile = profiles[0];
      if (request.method() === "PATCH") {
        Object.assign(profile, request.postDataJSON());
      }
      return route.fulfill({ json: profile });
    }
    if (path === "/skills") {
      expect(searchParams.get("grade")).toBe("CP");
      return route.fulfill({
        json: {
          grade: "CP",
          skillsGrade: "CP",
          currentPeriod: 1,
          skills: [{ id: "cp-add-sum-10", title: "Additions jusqu'à 10", instruction: "Calcule.", period: 1, isCurrentPeriod: true, answerModes: ["Keyboard", "Choice"] }],
        },
      });
    }
    if (path === "/series") {
      const { answerMode } = request.postDataJSON() as { answerMode: "Keyboard" | "Choice" };
      return route.fulfill({ json: series(answerMode) });
    }
    if (path.endsWith("/answers")) {
      const { index, answer } = request.postDataJSON() as { index: number; answer: number };
      const expected = index + 2;
      const correct = answer === expected;
      const key = path;
      answered.set(key, (answered.get(key) ?? 0) + (correct ? 1 : 0));
      const done = index === 9;
      if (done) {
        const earned = (answered.get(key) ?? 0) * 10 + 20 + (answered.get(key) === 10 ? 20 : 0);
        (profiles[0] as { points: number }).points += earned;
      }
      return route.fulfill({
        json: {
          correct,
          expected,
          explanation: `${index + 1} + 1 = ${expected}`,
          summary: done ? { correctCount: answered.get(key), exerciseCount: 10, pointsEarned: (answered.get(key) ?? 0) * 10 + 20 + (answered.get(key) === 10 ? 20 : 0), profilePoints: (profiles[0] as { points: number }).points } : null,
        },
      });
    }
    if (path === "/speech") {
      return route.fulfill({ status: 200, contentType: "audio/wav", body: Buffer.alloc(44) });
    }
    return route.fulfill({ status: 404 });
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
