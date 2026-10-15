import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { ApiError, type Profile } from "../api/client";
import { fakeApi, lea, renderApp } from "../test/renderWithApp";

/** No profile at first; the one that is created is then listed, like with the real core. */
function noProfileYet() {
  const profiles: Profile[] = [];
  return {
    listProfiles: vi.fn(async () => [...profiles]),
    createProfile: vi.fn(async (name: string, grade: Profile["grade"], characterId?: string) => {
      const created: Profile = { ...lea, id: "p-new", name, grade, points: 0, characterId: characterId ?? "lion" };
      profiles.push(created);
      return created;
    }),
  };
}

describe("ProfilesScreen", () => {
  it("lists the profiles and opens the menu with the chosen one", async () => {
    const api = fakeApi();
    const router = renderApp(api, "/", null);

    await userEvent.click(await screen.findByRole("button", { name: "Jouer avec le profil de Léa" }));

    await waitFor(() => expect(router.state.location.pathname).toBe("/menu"));
    expect(api.selectProfile).toHaveBeenCalledWith(lea.id);
    expect(await screen.findByRole("heading", { name: "En piste, Léa !" })).toBeInTheDocument();
  });

  it("shows the creation at once when there is no profile, and creates one", async () => {
    const api = fakeApi(noProfileYet());
    const router = renderApp(api, "/", null);
    const user = userEvent.setup();

    await user.type(await screen.findByLabelText("Ton prénom"), "Tom");
    await user.click(screen.getByRole("button", { name: "CE1" }));
    expect(await screen.findByRole("img", { name: "Lion" })).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Changer de personnage" }));
    expect(await screen.findByRole("img", { name: "Ours" })).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "C'est parti !" }));

    await waitFor(() => expect(router.state.location.pathname).toBe("/menu"));
    expect(api.drawCharacter).toHaveBeenCalledWith("lion");
    expect(api.createProfile).toHaveBeenCalledWith("Tom", "CE1", "ours");
  });

  it("offers the level assessment after the creation of a profile (SUB001418)", async () => {
    const api = fakeApi({
      ...noProfileYet(),
      getPlacementStatus: vi.fn().mockResolvedValue({
        subjects: [
          { subject: "Math", available: true, done: false },
          { subject: "Spelling", available: true, done: false },
        ],
        questionSeconds: 20,
      }),
    });
    const router = renderApp(api, "/", null);
    const user = userEvent.setup();

    await user.type(await screen.findByLabelText("Ton prénom"), "Tom");
    await user.click(await screen.findByRole("button", { name: "C'est parti !" }));

    await waitFor(() => expect(router.state.location.pathname).toBe("/evaluation"));
    expect(await screen.findByRole("heading", { name: "Évaluation de niveau" })).toBeInTheDocument();
  });

  it("explains a refused name in a simple sentence", async () => {
    const api = fakeApi({
      listProfiles: vi.fn().mockResolvedValue([]),
      createProfile: vi.fn().mockRejectedValue(new ApiError(400, "Invalid profile", "Ce prénom est déjà pris. Choisis-en un autre.")),
    });
    renderApp(api, "/", null);
    const user = userEvent.setup();

    await user.type(await screen.findByLabelText("Ton prénom"), "Léa");
    await user.click(screen.getByRole("button", { name: "C'est parti !" }));

    expect(await screen.findByRole("alert")).toHaveTextContent("Ce prénom est déjà pris. Choisis-en un autre.");
  });
  it("shows the parents' card last, which leads to the parents' space through the gate", async () => {
    const router = renderApp(fakeApi(), "/", null);
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: "Parents : espace parent" }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/parent"));
    await user.type(await screen.findByRole("textbox", { name: "Résultat du calcul" }), "162{Enter}");
    await waitFor(() => expect(router.state.location.pathname).toBe("/parent/espace"));

    // Back to the profile choice, where it was opened from.
    await user.click(await screen.findByRole("button", { name: "← Retour au choix du profil" }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/"));
  });

  it("the gate opened from the parents' card goes back to the profile choice", async () => {
    const router = renderApp(fakeApi(), "/", null);
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: "Parents : espace parent" }));
    await user.click(await screen.findByRole("button", { name: "← Retour" }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/"));
  });
});
