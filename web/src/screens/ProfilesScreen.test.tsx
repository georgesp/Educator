import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { ApiError } from "../api/client";
import { fakeApi, lea, renderApp } from "../test/renderWithApp";

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
    const api = fakeApi({ listProfiles: vi.fn().mockResolvedValue([]) });
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
});
