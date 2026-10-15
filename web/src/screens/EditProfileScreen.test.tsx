import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { ApiError, type Profile, type UpdateProfile } from "../api/client";
import { fakeApi, lea, renderApp } from "../test/renderWithApp";

/** A fake API whose profile list follows the updates, like the real core. */
function statefulApi() {
  let stored: Profile = { ...lea };
  return fakeApi({
    listProfiles: vi.fn(async () => [stored]),
    updateProfile: vi.fn(async (_id: string, changes: UpdateProfile) => {
      stored = { ...stored, ...Object.fromEntries(Object.entries(changes).filter(([, v]) => v !== null && v !== undefined)) };
      return stored;
    }),
  });
}

describe("EditProfileScreen", () => {
  it("opens from the profile choice and saves the name, the grade and the character", async () => {
    const api = statefulApi();
    const router = renderApp(api, "/", null);
    const user = userEvent.setup();

    await user.click(await screen.findByRole("link", { name: "Modifier le profil de Léa" }));
    expect(router.state.location.pathname).toBe("/profil/p-lea");

    const name = await screen.findByLabelText("Ton prénom");
    expect(name).toHaveValue("Léa");
    expect(screen.getByRole("button", { name: "CP" })).toHaveAttribute("aria-pressed", "true");
    expect(await screen.findByRole("button", { name: "Lion" })).toHaveAttribute("aria-pressed", "true");

    await user.clear(name);
    await user.type(name, "Léonie");
    await user.click(screen.getByRole("button", { name: "CE1" }));
    await user.click(screen.getByRole("button", { name: "Ours" }));
    await user.click(screen.getByRole("button", { name: "Enregistrer" }));

    await waitFor(() => expect(router.state.location.pathname).toBe("/"));
    expect(api.updateProfile).toHaveBeenCalledWith("p-lea", { name: "Léonie", grade: "CE1", characterId: "ours", placementSeconds: 20, examSeconds: 0 });
    expect(await screen.findByRole("button", { name: "Jouer avec le profil de Léonie" })).toBeInTheDocument();
  });

  it("Annuler changes nothing and goes back to the menu it came from", async () => {
    const api = statefulApi();
    const router = renderApp(api, "/profil/p-lea?retour=menu");
    const user = userEvent.setup();

    await user.type(await screen.findByLabelText("Ton prénom"), "xyz");
    await user.click(screen.getByRole("button", { name: "CM2" }));
    await user.click(screen.getByRole("button", { name: "Au hasard" }));
    await user.click(screen.getByRole("button", { name: "Annuler" }));

    await waitFor(() => expect(router.state.location.pathname).toBe("/menu"));
    expect(api.updateProfile).not.toHaveBeenCalled();
    expect(await screen.findByRole("heading", { name: "En piste, Léa !" })).toBeInTheDocument();
  });

  it("draws a character at random without saving it before « Enregistrer »", async () => {
    const api = statefulApi();
    renderApp(api, "/profil/p-lea");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: "Au hasard" }));

    expect(api.drawCharacter).toHaveBeenCalledWith("lion");
    expect(await screen.findByRole("button", { name: "Ours" })).toHaveAttribute("aria-pressed", "true");
    expect(api.updateProfile).not.toHaveBeenCalled();
  });

  it("explains a refused name in a simple sentence", async () => {
    const api = fakeApi({ updateProfile: vi.fn().mockRejectedValue(new ApiError(400, "Invalid profile", "Ce prénom est déjà pris. Choisis-en un autre.")) });
    renderApp(api, "/profil/p-lea");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: "Enregistrer" }));

    expect(await screen.findByRole("alert")).toHaveTextContent("Ce prénom est déjà pris. Choisis-en un autre.");
  });

  it("after a change of grade, the exercises are those of the new grade", async () => {
    const api = statefulApi();
    renderApp(api, "/menu");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("link", { name: "Modifier le profil de Léa" }));
    await user.click(await screen.findByRole("button", { name: "CE1" }));
    await user.click(screen.getByRole("button", { name: "Enregistrer" }));

    expect(await screen.findByRole("heading", { name: "En piste, Léa !" })).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: /Exercices/ }));
    await user.click(await screen.findByRole("button", { name: /Mathématiques/ }));
    await waitFor(() => expect(api.getSkills).toHaveBeenLastCalledWith("CE1", "Math", "p-lea"));
  });

  it("sets the timer of the level assessment and of the exam, « Sans chrono » included (SUB001418)", async () => {
    const api = statefulApi();
    renderApp(api, "/profil/p-lea?retour=menu");
    const user = userEvent.setup();

    const placement = await screen.findByRole("group", { name: "Évaluation de niveau" });
    const exam = screen.getByRole("group", { name: "Examen (bilan)" });
    // The current durations are shown: 20 s for the assessment, no timer for the exam.
    expect(within(placement).getByRole("button", { name: "20 s" })).toHaveAttribute("aria-pressed", "true");
    expect(within(exam).getByRole("button", { name: "Sans chrono" })).toHaveAttribute("aria-pressed", "true");
    expect(within(placement).queryByRole("button", { name: "Sans chrono" })).not.toBeInTheDocument();

    await user.click(within(placement).getByRole("button", { name: "45 s" }));
    await user.click(within(exam).getByRole("button", { name: "30 s" }));
    expect(within(placement).getByRole("button", { name: "20 s" })).toHaveAttribute("aria-pressed", "false");
    await user.click(screen.getByRole("button", { name: "Enregistrer" }));

    await waitFor(() => expect(api.updateProfile).toHaveBeenCalledWith("p-lea", expect.objectContaining({ placementSeconds: 45, examSeconds: 30 })));
  });
});
