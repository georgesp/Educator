import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { fakeApi, renderApp } from "../test/renderWithApp";

const spellingSkills = {
  subject: "Spelling" as const,
  grade: "CP" as const,
  skillsGrade: "CP" as const,
  currentPeriod: 2,
  skills: [
    {
      id: "cp-son-ou",
      title: "Le son [ou]",
      instruction: "Écoute bien le mot, puis écris-le.",
      period: 2,
      isCurrentPeriod: true,
      revisionFrom: null,
      answerModes: ["Keyboard", "Choice"] as ("Keyboard" | "Choice")[],
      family: "Spelling" as const,
      spellingFormats: ["Dictation", "MissingLetters", "ChooseSpelling"] as ("Dictation" | "MissingLetters" | "ChooseSpelling")[],
      mastery: 80,
    },
  ],
};

describe("Exercises by subject (SUB001416)", () => {
  it("Exercices → Orthographe → a skill → the kind of exercise → the series", async () => {
    const api = fakeApi({ getSkills: vi.fn().mockResolvedValue(spellingSkills) });
    const router = renderApp(api, "/exercices");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: /Orthographe/ }));
    expect(await screen.findByRole("heading", { name: "Exercices d'orthographe" })).toBeInTheDocument();
    expect(api.getSkills).toHaveBeenCalledWith("CP", "Spelling", "p-lea");
    // The mastery of the skill is shown.
    await user.click(await screen.findByRole("button", { name: /Le son \[ou\].*Réussi à 80 %/ }));

    await waitFor(() => expect(router.state.location.pathname).toBe("/exercices/orthographe/cp-son-ou"));
    expect(await screen.findByRole("heading", { name: "Quel exercice veux-tu faire ?" })).toBeInTheDocument();
    const kinds = screen.getAllByRole("button", { name: /dictée|mots à trous|bonne écriture/i }).map((b) => b.textContent);
    expect(kinds).toHaveLength(4);
    await user.click(screen.getByRole("button", { name: /Je choisis les lettres qui manquent/ }));

    await waitFor(() => expect(router.state.location.pathname).toBe("/serie/cp-son-ou/Choice"));
    expect(router.state.location.search).toBe("?format=MissingLetters");
  });

  it("Exercices → Mathématiques shows the maths skills", async () => {
    const api = fakeApi();
    renderApp(api, "/exercices");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: /Mathématiques/ }));

    expect(await screen.findByRole("heading", { name: "Exercices de maths" })).toBeInTheDocument();
    expect(api.getSkills).toHaveBeenCalledWith("CP", "Math", "p-lea");
  });
});
