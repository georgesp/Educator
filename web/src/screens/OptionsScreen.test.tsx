import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { isMuted } from "../audio/sounds";
import { isMascotHidden } from "../app/settings";
import { fakeApi, lea, renderApp } from "../test/renderWithApp";

const tom = { ...lea, id: "p-tom", name: "Tom", grade: "CE1" as const };

describe("OptionsScreen (SUB001416)", () => {
  it("deleting a profile takes two steps and the typed name: never an accidental click", async () => {
    const api = fakeApi({ listProfiles: vi.fn().mockResolvedValue([lea, tom]) });
    renderApp(api, "/options");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: "Supprimer le profil de Tom" }));
    expect(screen.getByRole("alertdialog")).toHaveTextContent("Supprimer le profil de Tom ?");
    // Step 1: keeping is the default.
    expect(screen.getByRole("button", { name: "Non, je garde le profil" })).toHaveFocus();
    await user.click(screen.getByRole("button", { name: "Continuer" }));

    // Step 2: the button stays disabled until the name is typed.
    const remove = screen.getByRole("button", { name: "Supprimer pour toujours" });
    expect(remove).toBeDisabled();
    await user.type(screen.getByRole("textbox"), "Léa");
    expect(remove).toBeDisabled();
    await user.clear(screen.getByRole("textbox"));
    await user.type(screen.getByRole("textbox"), "tom");
    expect(remove).toBeEnabled();
    await user.click(remove);

    await waitFor(() => expect(api.deleteProfile).toHaveBeenCalledWith("p-tom"));
    expect(api.deleteProfile).toHaveBeenCalledTimes(1);
  });

  it("deleting the current profile goes back to the profile choice", async () => {
    const api = fakeApi();
    const router = renderApp(api, "/options");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: "Supprimer le profil de Léa" }));
    await user.click(screen.getByRole("button", { name: "Continuer" }));
    await user.type(screen.getByRole("textbox"), "Léa{Enter}");

    await waitFor(() => expect(router.state.location.pathname).toBe("/"));
    expect(api.deleteProfile).toHaveBeenCalledWith("p-lea");
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
