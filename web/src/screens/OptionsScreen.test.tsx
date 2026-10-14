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

  it("cuts the sounds and hides the mascot, and leads to the profile sheet", async () => {
    const router = renderApp(fakeApi(), "/options");
    const user = userEvent.setup();

    const sounds = await screen.findByRole("checkbox", { name: "Couper les sons et la voix" });
    const before = isMuted();
    await user.click(sounds);
    expect(isMuted()).toBe(!before);
    await user.click(sounds);

    const mascot = screen.getByRole("checkbox", { name: "Cacher la mascotte" });
    await user.click(mascot);
    expect(isMascotHidden()).toBe(true);
    await user.click(mascot);
    expect(isMascotHidden()).toBe(false);

    await user.click(screen.getByRole("link", { name: "Modifier le profil de Léa" }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/profil/p-lea"));
    expect(router.state.location.search).toBe("?retour=options");
  });
});
