import { expect, test } from "@playwright/test";
import { mockLot3, openAsLea, type Calls } from "./lot3.mock";

test.use({ viewport: { width: 1024, height: 700 } });

test("the menu shows the level and the goal of the day; the avatar is dressed and saved at once", async ({ page }) => {
  const calls: Calls = [];
  await mockLot3(page, calls);
  await openAsLea(page);

  await expect(page.getByText("Niveau 5")).toBeVisible();
  await expect(page.getByText("2 séries sur 3")).toBeVisible();
  await page.getByRole("link", { name: "Habiller mon avatar" }).click();
  await page.getByRole("button", { name: "Bonnet à pompon" }).click();
  await expect.poll(() => calls.filter((c) => c.command === "update_avatar").length).toBe(1);
  const update = calls.find((c) => c.command === "update_avatar")!.args.request as { accessories: { accessoryId: string }[] };
  expect(update.accessories.map((a) => a.accessoryId).sort()).toEqual(["bonnet", "echarpe"]);
  // Locked (aria-disabled), but clickable to say why.
  await page.getByRole("button", { name: /Couronne/ }).click({ force: true });
  await expect(page.getByText(/Couronne : Atteins le niveau 20 pour le débloquer/)).toBeVisible();
});

test("the progress map leads to the exercises of a step", async ({ page }) => {
  await mockLot3(page);
  await openAsLea(page);
  await page.getByRole("link", { name: "Ma carte" }).click();
  await page.getByRole("button", { name: /Tables d'addition, en ce moment/ }).click();
  await page.getByRole("button", { name: "Les exercices" }).click();
  await expect(page).toHaveURL(/#\/exercices\/maths\/cp-add-tables/);
});

test("a theme chosen in Options applies at once to every screen", async ({ page }) => {
  await mockLot3(page);
  await openAsLea(page);
  await page.getByRole("button", { name: /Options/ }).click();
  await page.getByRole("button", { name: /Cosmos/ }).click();
  await expect(page.locator("html")).toHaveAttribute("data-theme", "cosmos");
  await page.getByRole("button", { name: "Grande", exact: true }).click();
  await expect(page.locator("html")).toHaveAttribute("data-text-size", "large");
  await page.getByRole("button", { name: /Menu/ }).click();
  await expect(page.locator("html")).toHaveAttribute("data-theme", "cosmos");
});
