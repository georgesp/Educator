import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { isMuted } from "../audio/sounds";
import { isMascotHidden } from "../app/settings";
import { fakeApi, renderApp } from "../test/renderWithApp";

describe("OptionsScreen (SUB001416)", () => {
  it("leads to the parents' space, where the profiles are deleted", async () => {
    const router = renderApp(fakeApi(), "/options");
    const user = userEvent.setup();

    await screen.findByRole("link", { name: "Modifier le profil de Léa" });
    expect(screen.queryByRole("button", { name: /Supprimer/ })).not.toBeInTheDocument();
    await user.click(screen.getByRole("link", { name: "Espace parent" }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/parent"));
  });

  it("cuts the sounds and hides the mascot in the child's settings, and leads to the profile sheet", async () => {
    const api = fakeApi();
    const router = renderApp(api, "/options");
    const user = userEvent.setup();

    const sounds = await screen.findByRole("checkbox", { name: "Couper les sons et la voix" });
    const before = isMuted();
    await user.click(sounds);
    expect(isMuted()).toBe(!before);
    expect(api.updateProfileSettings).toHaveBeenCalledWith("p-lea", { sound: false });
    await user.click(sounds);

    const mascot = screen.getByRole("checkbox", { name: "Cacher la mascotte" });
    await user.click(mascot);
    expect(isMascotHidden()).toBe(true);
    await user.click(mascot);
    expect(isMascotHidden()).toBe(false);
    expect(api.updateProfileSettings).toHaveBeenCalledWith("p-lea", { mascot: false });

    await user.click(screen.getByRole("link", { name: "Modifier le profil de Léa" }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/profil/p-lea"));
    expect(router.state.location.search).toBe("?retour=options");
  });

  it("changes the voice, the text, the font and the theme at once (SUB001512, SUB001513)", async () => {
    const api = fakeApi();
    renderApp(api, "/options");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: "Plus lente" }));
    expect(api.updateProfileSettings).toHaveBeenLastCalledWith("p-lea", { voiceSpeed: "Slower" });
    await user.click(screen.getByRole("button", { name: "Grande" }));
    await waitFor(() => expect(document.documentElement.dataset.textSize).toBe("large"));
    await user.click(screen.getByRole("checkbox", { name: "Police adaptée à la dyslexie" }));
    await waitFor(() => expect(document.documentElement.dataset.font).toBe("dyslexia"));
    await user.click(await screen.findByRole("button", { name: /Cosmos/ }));
    await waitFor(() => expect(document.documentElement.dataset.theme).toBe("cosmos"));
    expect(screen.getByRole("button", { name: /Cosmos/ })).toHaveAttribute("aria-pressed", "true");
    expect(api.updateProfileSettings).toHaveBeenLastCalledWith("p-lea", { theme: "cosmos" });
  });
});
