import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { fakeApi, renderApp } from "../test/renderWithApp";
import { AUTO_NEXT_DELAY_MS } from "../screens/SeriesScreen";
import { isVoiceEnabled, setVoiceEnabled } from "./useSpeech";

describe("voice switch", () => {
  it("is off by default", () => {
    expect(isVoiceEnabled()).toBe(false);
  });

  it("while it is off, nothing is read and no speaker button is shown", async () => {
    const api = fakeApi();
    const router = renderApp(api, "/", null);
    const user = userEvent.setup();

    // Profile choice, menu (with the hover reading), skills, mode and series: no speaker button anywhere.
    await user.click(await screen.findByRole("button", { name: "Jouer avec le profil de Léa" }));
    await screen.findByRole("heading", { name: "En piste, Léa !" });
    await user.hover(screen.getByRole("button", { name: /Leçons/ }));
    await user.click(screen.getByRole("button", { name: /Leçons/ }));
    expect(screen.queryByRole("button", { name: /Écouter/ })).not.toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: /Exercices/ }));
    await screen.findByRole("heading", { name: "Exercices de calcul" });
    expect(screen.queryByRole("button", { name: /Écouter/ })).not.toBeInTheDocument();
    await user.click(await screen.findByRole("button", { name: /Additions jusqu'à 10/ }));
    await user.click(await screen.findByRole("button", { name: /J'écris la réponse/ }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/serie/cp-add-sum-10/Keyboard"));

    await user.type(await screen.findByLabelText("Ta réponse"), "6{Enter}");
    await screen.findByText("3 + 4 = 7");
    await user.click(screen.getByRole("button", { name: "Suivant" }));
    await waitFor(() => expect(screen.getByLabelText("5 + ? = 9")).toBeInTheDocument(), { timeout: AUTO_NEXT_DELAY_MS + 1000 });
    expect(screen.queryByRole("button", { name: /Écouter/ })).not.toBeInTheDocument();

    expect(api.speak).not.toHaveBeenCalled();
  });

  it("turned on again, the speaker buttons come back and read aloud", async () => {
    setVoiceEnabled(true);
    const api = fakeApi();
    renderApp(api, "/", null);

    const listen = await screen.findByRole("button", { name: /Écouter/ });
    await userEvent.click(listen);

    expect(api.speak).toHaveBeenCalledWith(expect.stringContaining("Qui joue ?"));
  });
});
