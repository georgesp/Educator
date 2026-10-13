import { act, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it } from "vitest";
import { setVoiceEnabled } from "../audio/useSpeech";
import { fakeApi, renderApp, twoExercises } from "../test/renderWithApp";
import { AUTO_NEXT_DELAY_MS } from "./SeriesScreen";

describe("SeriesScreen", () => {
  // These tests check what is read aloud: the voice is off by default (see voice.test.tsx).
  beforeEach(() => setVoiceEnabled(true));

  it("keyboard mode: a right answer moves on, a mistake shows the correction, then the summary", async () => {
    const api = fakeApi();
    renderApp(api, "/serie/cp-add-sum-10/Keyboard");
    const user = userEvent.setup();

    const input = await screen.findByLabelText("Ta réponse");
    expect(screen.getByLabelText("3 + 4 = ?")).toBeInTheDocument();
    expect(api.startSeries).toHaveBeenCalledWith("p-lea", "cp-add-sum-10", "Keyboard");
    expect(api.speak).toHaveBeenCalledWith("Calcule. Combien font 3 plus 4 ?");

    await user.type(input, "a7{Enter}");
    expect(api.answerSeries).toHaveBeenCalledWith("s1", 0, { kind: "number", value: 7 }, expect.any(Number));
    expect(await screen.findByRole("status")).toHaveTextContent("Bravo !");

    // The next exercise comes by itself.
    await waitFor(() => expect(screen.getByLabelText("5 + ? = 9")).toBeInTheDocument(), { timeout: AUTO_NEXT_DELAY_MS + 1000 });
    await user.click(screen.getByRole("button", { name: "6" }));
    await user.click(screen.getAllByRole("button", { name: "Valider" })[0]);

    expect(await screen.findByText("5 + 4 = 9")).toBeInTheDocument();
    expect(api.speak).toHaveBeenCalledWith("5 + 4 = 9");
    await user.click(screen.getByRole("button", { name: "Suivant" }));

    expect(await screen.findByRole("heading", { name: "Série terminée !" })).toBeInTheDocument();
    expect(screen.getByText("1 bonne réponse sur 2")).toBeInTheDocument();
    expect(screen.getByText("+40 points")).toBeInTheDocument();
    // 1 out of 2: no fireworks.
    expect(screen.queryByTestId("fireworks")).not.toBeInTheDocument();
  });

  it("fireworks when every answer is right, and only then", async () => {
    const api = fakeApi();
    renderApp(api, "/serie/cp-add-sum-10/Keyboard");
    const user = userEvent.setup();

    await user.type(await screen.findByLabelText("Ta réponse"), "7{Enter}");
    await waitFor(() => expect(screen.getByLabelText("5 + ? = 9")).toBeInTheDocument(), { timeout: AUTO_NEXT_DELAY_MS + 1000 });
    await user.type(screen.getByLabelText("Ta réponse"), "4{Enter}");

    expect(await screen.findByText("2 bonnes réponses sur 2", {}, { timeout: AUTO_NEXT_DELAY_MS + 1000 })).toBeInTheDocument();
    expect(screen.getByTestId("fireworks")).toHaveAccessibleName("Bravo, tout est juste !");
    expect(api.speak).toHaveBeenLastCalledWith(expect.stringMatching(/^Bravo, tout est juste ! Série terminée !/));
    // The buttons of the summary stay usable during the animation.
    expect(screen.getByRole("button", { name: "Recommencer" })).toBeEnabled();
  });

  it("choice mode: four tiles, the picked one is checked by the host", async () => {
    const withChoices = {
      ...twoExercises,
      answerMode: "Choice" as const,
      exercises: twoExercises.exercises.map((e) => ({
        ...e,
        input: { kind: "pick" as const },
        choices: (e.index === 0 ? [6, 7, 8, 17] : [3, 4, 5, 14]).map((value) => ({ label: String(value), answer: { kind: "number" as const, value } })),
      })),
    };
    const api = fakeApi({ startSeries: async () => withChoices });
    renderApp(api, "/serie/cp-add-sum-10/Choice");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: "8" }));

    expect(api.answerSeries).toHaveBeenCalledWith("s1", 0, { kind: "number", value: 8 }, expect.any(Number));
    expect(await screen.findByText("3 + 4 = 7")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "7" })).toHaveClass("choice-right");
    expect(screen.queryByLabelText("Ta réponse")).not.toBeInTheDocument();
  });

  it("quitting asks first, then abandons the series", async () => {
    const api = fakeApi();
    const router = renderApp(api, "/serie/cp-add-sum-10/Keyboard");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: "Quitter" }));
    expect(screen.getByRole("alertdialog")).toHaveTextContent("Tes bonnes réponses sont gardées.");
    await act(() => user.click(screen.getByRole("button", { name: "Oui, j'arrête" })));

    await waitFor(() => expect(router.state.location.pathname).toBe("/menu"));
    expect(api.abandonSeries).toHaveBeenCalledWith("s1");
  });

  it("only accepts digits", async () => {
    renderApp(fakeApi(), "/serie/cp-add-sum-10/Keyboard");
    const input = await screen.findByLabelText("Ta réponse");

    await userEvent.type(input, "a1b2");

    expect(input).toHaveValue("12");
  });
});
