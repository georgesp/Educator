import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import type { AnswerPayload, Exercise, Series, SeriesAnswer } from "../api/client";
import { fakeApi, renderApp, summaryOf } from "../test/renderWithApp";

/** A series of one exercise, answered by `expected`. */
function oneExercise(exercise: Omit<Exercise, "index">, expected: AnswerPayload, help: SeriesAnswer["help"] = null) {
  const series: Series = {
    seriesId: "s",
    sessionType: "Exercise",
    subject: "Math",
    seed: 1,
    skillId: "ce1",
    instruction: "Consigne.",
    answerMode: "Keyboard",
    timeLimitSeconds: null,
    exercises: [{ index: 0, ...exercise }],
  };
  const answerSeries = vi.fn(async (_s: string, _i: number, given: AnswerPayload) => ({
    correct: JSON.stringify(given) === JSON.stringify(expected),
    expected,
    expectedLabel: "label",
    explanation: "La correction.",
    help,
    correction: null,
    summary: summaryOf(0, 1, 0, 120, "ce1"),
  }));
  return fakeApi({ startSeries: async () => series, answerSeries });
}

const base = { question: null, picture: null, choices: null, prompt: "?", spokenPrompt: "?", word: null } as const;

describe("SeriesScreen — CE1 families", () => {
  it("comparison: three sign buttons, the picked sign goes in the slot", async () => {
    const api = oneExercise(
      { ...base, statement: [{ kind: "number", value: 438 }, { kind: "slot", slot: 0 }, { kind: "number", value: 483 }], input: { kind: "comparison" }, prompt: "438 ? 483" },
      { kind: "comparison", sign: "<" },
    );
    renderApp(api, "/serie/ce1-num-compare/Keyboard");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: "est plus grand que" }));

    expect(api.answerSeries).toHaveBeenCalledWith("s", 0, { kind: "comparison", sign: ">" }, expect.any(Number));
    expect(await screen.findByText("La correction.")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "est plus petit que" })).toHaveClass("choice-right");
    expect(within(document.querySelector(".equation") as HTMLElement).getByText(">")).toHaveClass("slot-wrong");
  });

  it("framing: two fields, Enter goes from the first to the second, then sends both", async () => {
    const api = oneExercise(
      {
        ...base,
        question: "Encadre 347 entre deux centaines.",
        statement: [{ kind: "slot", slot: 0 }, { kind: "sign", value: "<" }, { kind: "number", value: 347 }, { kind: "sign", value: "<" }, { kind: "slot", slot: 1 }],
        input: { kind: "number", slots: 2 },
      },
      { kind: "sequence", values: [300, 400] },
    );
    renderApp(api, "/serie/ce1-num-frame-line/Keyboard");
    const user = userEvent.setup();

    await user.type(await screen.findByLabelText("Premier nombre"), "300{Enter}");
    expect(screen.getByLabelText("Deuxième nombre")).toHaveFocus();
    // The keypad types in the field that has the focus.
    await user.click(screen.getByRole("button", { name: "4" }));
    await user.click(screen.getByRole("button", { name: "0" }));
    await user.click(screen.getByRole("button", { name: "0" }));
    await user.click(screen.getByRole("button", { name: "Valider" }));

    expect(api.answerSeries).toHaveBeenCalledWith("s", 0, { kind: "sequence", values: [300, 400] }, expect.any(Number));
    expect(await screen.findByRole("status")).toHaveTextContent(/!/);
  });

  it("fraction: a numerator and a denominator", async () => {
    const api = oneExercise(
      {
        ...base,
        question: "Quelle fraction de la bande est coloriée ?",
        statement: [{ kind: "slot", slot: 0 }],
        picture: { kind: "fractionShape", shape: "Bar", parts: 4, colored: 3 },
        input: { kind: "fraction" },
      },
      { kind: "fraction", numerator: 3, denominator: 4 },
    );
    renderApp(api, "/serie/ce1-frac-read-write/Keyboard");
    const user = userEvent.setup();

    expect(await screen.findByRole("img", { name: "Une bande partagée en 4 parts, 3 coloriées" })).toBeInTheDocument();
    await user.type(screen.getByLabelText("Numérateur (en haut)"), "6{Enter}");
    await user.type(screen.getByLabelText("Dénominateur (en bas)"), "8{Enter}");

    expect(api.answerSeries).toHaveBeenCalledWith("s", 0, { kind: "fraction", numerator: 6, denominator: 8 }, expect.any(Number));
  });

  it("order: the arrows move the numbers, then « Valider » sends the order", async () => {
    const api = oneExercise(
      { ...base, question: "Range ces nombres du plus petit au plus grand.", statement: [], input: { kind: "order", items: [483, 348, 438], ascending: true } },
      { kind: "sequence", values: [348, 438, 483] },
    );
    renderApp(api, "/serie/ce1-num-compare/Keyboard");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: "Déplacer 483 vers la droite" }));
    await user.click(screen.getByRole("button", { name: "Déplacer 483 vers la droite" }));
    await user.click(screen.getByRole("button", { name: "Valider" }));

    expect(api.answerSeries).toHaveBeenCalledWith("s", 0, { kind: "sequence", values: [348, 438, 483] }, expect.any(Number));
  });

  it("several right numbers: pick them, then validate; the missed ones are shown", async () => {
    const api = oneExercise(
      { ...base, question: "Clique sur tous les nombres pairs.", statement: [], input: { kind: "multiChoice", items: [711, 496, 461, 710, 152, 41] } },
      { kind: "set", values: [496, 710, 152] },
    );
    renderApp(api, "/serie/ce1-parity/Keyboard");
    const user = userEvent.setup();

    expect(await screen.findByRole("button", { name: "Valider" })).toBeDisabled();
    await user.click(screen.getByRole("button", { name: "496" }));
    await user.click(screen.getByRole("button", { name: "461" }));
    await user.click(screen.getByRole("button", { name: "Valider" }));

    expect(api.answerSeries).toHaveBeenCalledWith("s", 0, { kind: "set", values: [496, 461] }, expect.any(Number));
    await waitFor(() => expect(screen.getByRole("button", { name: "461" })).toHaveClass("choice-wrong"));
    expect(screen.getByRole("button", { name: "710" })).toHaveClass("choice-missed");
    expect(screen.getByRole("button", { name: "496" })).toHaveClass("choice-right");
  });

  it("yes / no", async () => {
    const api = oneExercise({ ...base, question: "573 est-il un nombre pair ?", statement: [], input: { kind: "yesNo" } }, { kind: "yesNo", value: false });
    renderApp(api, "/serie/ce1-parity/Keyboard");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: "Non" }));

    expect(api.answerSeries).toHaveBeenCalledWith("s", 0, { kind: "yesNo", value: false }, expect.any(Number));
  });

  it("number line: a click on a graduation answers with its number", async () => {
    const api = oneExercise(
      {
        ...base,
        question: "Place 470 sur la droite graduée.",
        statement: [],
        picture: { kind: "numberLine", start: 400, step: 10, ticks: 11, labeled: [0, 10], arrow: null },
        input: { kind: "numberLine" },
      },
      { kind: "number", value: 470 },
    );
    renderApp(api, "/serie/ce1-num-frame-line/Keyboard");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: "Graduation 6" }));

    expect(api.answerSeries).toHaveBeenCalledWith("s", 0, { kind: "number", value: 460 }, expect.any(Number));
  });

  it("pick among written words, then the correction with its picture", async () => {
    const choices = ["deux cent quatre-vingts", "deux cents quatre-vingt", "deux cent quatre-vingt"].map((value) => ({ label: value, answer: { kind: "text" as const, value } }));
    const api = oneExercise(
      { ...base, question: "Choisis la bonne écriture de ce nombre.", statement: [{ kind: "number", value: 280 }], input: { kind: "pick" }, choices },
      { kind: "text", value: "deux cent quatre-vingts" },
      { kind: "barModel", bars: [[200, 80]], total: 280 },
    );
    renderApp(api, "/serie/ce1-num-read-write/Choice");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: "deux cent quatre-vingt" }));

    expect(api.answerSeries).toHaveBeenCalledWith("s", 0, { kind: "text", value: "deux cent quatre-vingt" }, expect.any(Number));
    expect(await screen.findByRole("button", { name: "deux cent quatre-vingts" })).toHaveClass("choice-right");
    expect(screen.getByRole("img", { name: "Schéma en barres" })).toBeInTheDocument();
    expect(screen.getByText("Regarde le schéma")).toBeInTheDocument();
  });

  it("draws the base-ten blocks and the rows of dots", async () => {
    const api = oneExercise(
      { ...base, question: "Quel nombre est représenté ?", statement: [{ kind: "slot", slot: 0 }], picture: { kind: "baseTen", hundreds: 3, tens: 4, units: 7 }, input: { kind: "number", slots: 1 } },
      { kind: "number", value: 347 },
      { kind: "arrayGrid", rows: 7, columns: 13, split: 10 },
    );
    renderApp(api, "/serie/ce1-num-hundreds/Keyboard");
    const user = userEvent.setup();

    expect(await screen.findByRole("img", { name: "3 plaques de cent, 4 barres de dix, 7 cubes" })).toBeInTheDocument();
    await user.type(screen.getByLabelText("Ta réponse"), "374{Enter}");
    expect(await screen.findByRole("img", { name: "7 rangées de 13 points" })).toBeInTheDocument();
  });
});
