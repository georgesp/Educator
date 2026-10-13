import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { setVoiceEnabled } from "../audio/useSpeech";
import { fakeApi, renderApp } from "../test/renderWithApp";

describe("MenuScreen", () => {
  it("shows the profile panel: name, grade and points", async () => {
    renderApp(fakeApi(), "/menu");

    expect(await screen.findByRole("heading", { name: "En piste, Léa !" })).toBeInTheDocument();
    expect(screen.getByText("CP")).toBeInTheDocument();
    expect(screen.getByLabelText("120 points")).toBeInTheDocument();
  });

  it("every entry is open (SUB001416): Lessons, Exercises, Exam and Options", async () => {
    setVoiceEnabled(true);
    const api = fakeApi();
    const router = renderApp(api, "/menu");
    const user = userEvent.setup();

    const entries = await screen.findAllByRole("button", { name: /Leçons|Exercices|Examen|Options/ });
    expect(entries.map((e) => e.textContent)).toEqual([
      expect.stringContaining("Leçons"),
      expect.stringContaining("Exercices"),
      expect.stringContaining("Examen"),
      expect.stringContaining("Options"),
    ]);
    expect(entries.every((e) => e.getAttribute("aria-disabled") === null)).toBe(true);

    await user.click(screen.getByRole("button", { name: /Examen/ }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/examen"));
    expect(api.speak).not.toHaveBeenCalledWith("Cette partie arrive bientôt.");
  });

  it("opens Exercises", async () => {
    const router = renderApp(fakeApi(), "/menu");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: /Exercices/ }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/exercices"));
  });

  it("opens Lessons", async () => {
    const router = renderApp(fakeApi(), "/menu");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: /Leçons/ }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/lecons"));
  });

  it("moves between the entries with the arrow keys", async () => {
    renderApp(fakeApi(), "/menu");
    const user = userEvent.setup();
    const exercises = await screen.findByRole("button", { name: /Exercices/ });
    exercises.focus();

    await user.keyboard("{ArrowDown}");
    expect(screen.getByRole("button", { name: /Examen/ })).toHaveFocus();
    await user.keyboard("{ArrowUp}{ArrowUp}{ArrowUp}");
    expect(screen.getByRole("button", { name: /Options/ })).toHaveFocus();
  });

  it("without a chosen profile, goes back to the profile choice", async () => {
    const router = renderApp(fakeApi(), "/menu", null);

    await waitFor(() => expect(router.state.location.pathname).toBe("/"));
  });
});
