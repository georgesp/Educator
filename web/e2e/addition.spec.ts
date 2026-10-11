import { expect, test } from "@playwright/test";
import { E2E_BACKEND_URL, E2E_TOKEN } from "../playwright.config";

test("a child answers an addition from the home screen", async ({ page }) => {
  const calls: string[] = [];
  await page.route(`${E2E_BACKEND_URL}/api/ui/**`, async (route) => {
    const request = route.request();
    const path = new URL(request.url()).pathname;
    calls.push(`${request.method()} ${path}`);
    expect(request.headers()["x-local-token"]).toBe(E2E_TOKEN);

    if (path === "/api/ui/status") {
      return route.fulfill({
        json: { version: "0.1.0", database: { isReady: true }, speech: { isAvailable: true, voice: "fr_FR-siwis-medium", problem: null } },
      });
    }
    if (path === "/api/ui/exercises/additions") {
      return route.fulfill({ json: { id: "e2e-1", left: 5, right: 3, statement: "5 + 3" } });
    }
    if (path.endsWith("/answer")) {
      const { answer } = request.postDataJSON() as { answer: number };
      return route.fulfill({ json: { isCorrect: answer === 8, expectedAnswer: 8 } });
    }
    if (path.endsWith("/speech")) {
      return route.fulfill({ status: 200, contentType: "audio/wav", body: Buffer.alloc(44) });
    }
    return route.fulfill({ status: 404 });
  });

  await page.goto("/");
  await expect(page.getByText("Voix : fr_FR-siwis-medium")).toBeVisible();
  await page.getByRole("link", { name: "Faire une addition" }).click();

  await expect(page.getByText("5 + 3")).toBeVisible();
  await page.getByRole("button", { name: /Écouter/ }).click();
  await page.getByLabel("Ta réponse").fill("8");
  await page.getByRole("button", { name: "Valider" }).click();

  await expect(page.getByRole("status")).toHaveText("Bravo ! C'est la bonne réponse.");
  expect(calls).toContain("GET /api/ui/exercises/additions/e2e-1/speech");
});
