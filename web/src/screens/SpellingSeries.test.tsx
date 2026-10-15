import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import type { AnswerPayload, Exercise, Series, SeriesAnswer } from "../api/client";
import { fakeApi, renderApp, summaryOf } from "../test/renderWithApp";

function spellingSeries(exercise: Omit<Exercise, "index">): Series {
  return {
    seriesId: "s",
    sessionType: "Exercise",
    subject: "Spelling",
    seed: 1,
    skillId: "ce1-son-o-au-eau",
    instruction: "Écoute bien le mot, puis écris-le.",
    answerMode: "Keyboard",
    timeLimitSeconds: null,
    questionSeconds: null,
    exercises: [{ index: 0, ...exercise }],
  };
}

const dictation: Omit<Exercise, "index"> = {
  question: "Écoute le mot et écris-le.",
  statement: [],
  picture: null,
  input: { kind: "spelling", maxLength: 12 },
  choices: null,
  prompt: "Dictée",
  spokenPrompt: "",
  word: { before: null, after: null, missing: null, hasExample: true, article: "un" },
};

/** « bateau » written « batô »: b, a, t right, ô instead of e, a, u missing. */
const wrongAnswer: SeriesAnswer = {
  correct: false,
  expected: { kind: "text", value: "bateau" },
  expectedLabel: "bateau",
  explanation: "On écrit « bateau » : ici, le son o s'écrit « eau ».",
  help: null,
  correction: {
    correct: false,
    letters: [
      { kind: "Right", given: "b", expected: "b" },
      { kind: "Right", given: "a", expected: "a" },
      { kind: "Right", given: "t", expected: "t" },
      { kind: "Wrong", given: "ô", expected: "e" },
      { kind: "Missing", given: null, expected: "a" },
      { kind: "Missing", given: null, expected: "u" },
    ],
  },
  summary: summaryOf(0, 1, 20, 140, "ce1-son-o-au-eau"),
};

describe("SeriesScreen — spelling", () => {
  it("dictation: the word is played, written with the on-screen keyboard, and corrected letter by letter", async () => {
    const answerSeries = vi.fn(async (_s: string, _i: number, _a: AnswerPayload, _d?: number) => wrongAnswer);
    const api = fakeApi({ startSeries: vi.fn(async () => spellingSeries(dictation)), answerSeries });
    renderApp(api, "/serie/ce1-son-o-au-eau/Keyboard?format=Dictation");
    const user = userEvent.setup();

    expect(await screen.findByLabelText("Écris le mot")).toBeInTheDocument();
    expect(api.startSeries).toHaveBeenCalledWith("p-lea", "ce1-son-o-au-eau", "Keyboard", "Dictation");
    // Nothing is read by itself (decision of 2026-10-15): the word is heard on click, alone or in
    // its sentence; the instruction has its speaker too.
    expect(api.speakSeriesWord).not.toHaveBeenCalled();
    expect(api.speak).not.toHaveBeenCalled();
    await user.click(screen.getByRole("button", { name: "Écouter le mot" }));
    expect(api.speakSeriesWord).toHaveBeenCalledWith("s", 0, false, false);
    await user.click(screen.getByRole("button", { name: "Écouter la consigne" }));
    expect(api.speak).toHaveBeenCalledWith("Écoute bien le mot, puis écris-le.");
    await user.click(screen.getByRole("button", { name: "Le mot dans une phrase" }));
    expect(api.speakSeriesWord).toHaveBeenCalledWith("s", 0, true, false);
    // The noun's article is shown before the box (SUB026706): the child writes the noun only.
    expect(screen.getByText("un", { selector: ".spelling-article" })).toBeInTheDocument();

    // Mouse only: b, a, t, then the accented ô.
    const keyboard = screen.getByRole("group", { name: "Clavier des lettres" });
    for (const letter of ["b", "a", "t", "ô"]) {
      await user.click(within(keyboard).getByRole("button", { name: `Lettre ${letter}` }));
    }
    expect(screen.getByLabelText("Écris le mot")).toHaveValue("batô");
    await user.click(within(keyboard).getByRole("button", { name: "Valider" }));

    expect(answerSeries).toHaveBeenCalledWith("s", 0, { kind: "text", value: "batô" }, expect.any(Number));
    expect(await screen.findByText("Tu as écrit :")).toBeInTheDocument();
    expect(screen.getByTitle("Il fallait « e » au lieu de « ô »")).toHaveClass("word-letter-wrong");
    expect(screen.getAllByTitle(/Il manque/)).toHaveLength(2);
    expect(screen.getByText("bateau", { selector: ".word-right-text" })).toBeInTheDocument();
  });

  it("« Plus lentement » reads the word very slowly, with the mouse or the keyboard (SUB026702)", async () => {
    const api = fakeApi({ startSeries: vi.fn(async () => spellingSeries(dictation)) });
    renderApp(api, "/serie/ce1-son-o-au-eau/Keyboard?format=Dictation");
    const user = userEvent.setup();

    const slow = await screen.findByRole("button", { name: "Plus lentement" });
    await user.click(slow);
    expect(api.speakSeriesWord).toHaveBeenLastCalledWith("s", 0, false, true);

    vi.mocked(api.speakSeriesWord).mockClear();
    slow.focus();
    await user.keyboard("{Enter}");
    expect(api.speakSeriesWord).toHaveBeenCalledWith("s", 0, false, true);
  });

  it("the physical keyboard works too, Enter validates", async () => {
    const answerSeries = vi.fn(async () => ({ ...wrongAnswer, correct: true, correction: { correct: true, letters: [] } }));
    renderApp(fakeApi({ startSeries: vi.fn(async () => spellingSeries(dictation)), answerSeries }), "/serie/ce1-son-o-au-eau/Keyboard?format=Dictation");
    const user = userEvent.setup();

    await user.type(await screen.findByLabelText("Écris le mot"), "Bateau{Enter}");

    expect(answerSeries).toHaveBeenCalledWith("s", 0, { kind: "text", value: "Bateau" }, expect.any(Number));
    expect(await screen.findByRole("status")).toHaveTextContent(/Bravo|Super|Génial|Exactement|Bien joué/);
  });

  it("missing letters to pick: the word with its blank, « rien » for a letter left out", async () => {
    const missing: Omit<Exercise, "index"> = {
      ...dictation,
      question: "Choisis les lettres qui manquent.",
      input: { kind: "pick" },
      prompt: "des chat_",
      word: { before: "des chat", after: "", missing: 1, hasExample: false, article: null },
      choices: ["s", "", "x"].map((value) => ({ label: value, answer: { kind: "text" as const, value } })),
    };
    const answerSeries = vi.fn(async () => ({ ...wrongAnswer, expected: { kind: "text" as const, value: "s" }, expectedLabel: "s" }));
    renderApp(fakeApi({ startSeries: vi.fn(async () => ({ ...spellingSeries(missing), answerMode: "Choice" as const })), answerSeries }), "/serie/ce1-pluriel-s/Choice");
    const user = userEvent.setup();

    // As many dashes as letters missing.
    expect(await screen.findByLabelText("des chat_")).toHaveTextContent("des chat_");
    await user.click(screen.getByRole("button", { name: "rien" }));

    expect(answerSeries).toHaveBeenCalledWith("s", 0, { kind: "text", value: "" }, expect.any(Number));
    expect(await screen.findByRole("button", { name: "s" })).toHaveClass("choice-right");
    expect(screen.getByRole("button", { name: "rien" })).toHaveClass("choice-wrong");
  });

  it("missing letters to type: one dash per letter missing (« bat___ »)", async () => {
    const typed: Omit<Exercise, "index"> = {
      ...dictation,
      question: "Écris les lettres qui manquent.",
      prompt: "bat_",
      word: { before: "bat", after: "", missing: 3, hasExample: false, article: null },
    };
    renderApp(fakeApi({ startSeries: vi.fn(async () => spellingSeries(typed)) }), "/serie/ce1-son-o-au-eau/Keyboard");

    const field = await screen.findByLabelText("Les 3 lettres qui manquent");
    expect(field).toHaveAttribute("placeholder", "___");
  });
});
