import { expect, test } from "@playwright/test";
import { mockLot3, type Calls } from "./lot3.mock";

test.use({ viewport: { width: 1024, height: 700 } });

/**
 * Lot tablette 2 (SUB027603): the screen « Synchronisation » of the parents' space on the computer.
 * The core is simulated: the server listens while the screen is open, a tablet pairs with the
 * code, then is removed.
 */
test("the computer shows its pairing code, lists the tablet and removes it", async ({ page }) => {
  const calls: Calls = [];
  let devices: { id: string; name: string; pairedAt: string; lastSyncAt: string | null }[] = [];
  let receiving = false;
  const status = () => ({
    role: "Computer",
    receiving,
    receiveAlways: false,
    address: receiving ? "192.168.1.10:47615" : null,
    problem: null,
    devices,
    questions: [],
  });
  await mockLot3(page, calls, (command) => {
    switch (command) {
      case "get_parent_gate":
        return { id: "gate", left: 27, right: 6 };
      case "check_parent_gate":
        return true;
      case "get_sync_status":
        return status();
      case "open_sync_screen":
        receiving = true;
        return status();
      case "close_sync_screen":
        receiving = false;
        return status();
      case "start_sync_pairing":
        // The tablet scans the code a moment later.
        setTimeout(() => {
          devices = [{ id: "tab-1", name: "Tablette 1", pairedAt: "2026-10-15T19:00:00.000Z", lastSyncAt: "2026-10-15T19:01:00.000Z" }];
        }, 500);
        return {
          qrSvg: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><rect width="10" height="10"/></svg>',
          code: "ABCD-EFGH",
          address: "192.168.1.10:47615",
          expiresAt: "2026-10-15T19:05:00.000Z",
        };
      case "cancel_sync_pairing":
        return status();
      case "remove_sync_device":
        devices = [];
        return status();
      default:
        return undefined;
    }
  });

  await page.goto("/");
  await page.getByRole("button", { name: "Parents : espace parent" }).click();
  await page.getByRole("textbox", { name: "Résultat du calcul" }).fill("162");
  await page.getByRole("button", { name: "Entrer" }).click();

  const section = page.getByRole("region", { name: "Synchronisation avec la tablette" });
  await expect(section.getByText("L'ordinateur attend la tablette sur le Wi-Fi (192.168.1.10:47615).")).toBeVisible();
  await section.getByRole("button", { name: "Associer une tablette" }).click();
  await expect(section.getByRole("img", { name: "Code à scanner avec la tablette" })).toBeVisible();
  await expect(section.getByText("code ABCD-EFGH")).toBeVisible();

  // Paired: the code closes by itself, the tablet is listed (the screen reads the state again).
  await expect(section.getByText("Tablette 1")).toBeVisible({ timeout: 10_000 });
  await expect(section.getByRole("img", { name: "Code à scanner avec la tablette" })).toBeHidden();

  await section.getByRole("button", { name: "Retirer Tablette 1" }).click();
  await section.getByRole("alertdialog").getByRole("button", { name: "Retirer" }).click();
  await expect(section.getByText("Aucune tablette associée pour l'instant.")).toBeVisible();

  await page.getByRole("button", { name: /Retour au choix du profil/ }).click();
  await expect.poll(() => calls.some((c) => c.command === "close_sync_screen")).toBe(true);
});
