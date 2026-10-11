import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { fakeApi, renderApp } from "../test/renderWithApp";

describe("MenuScreen", () => {
  it("shows the profile panel: name, grade and points", async () => {
    renderApp(fakeApi(), "/menu");

    expect(await screen.findByRole("heading", { name: "En piste, Léa !" })).toBeInTheDocument();
    expect(screen.getByText("CP")).toBeInTheDocument();
    expect(screen.getByLabelText("120 points")).toBeInTheDocument();
  });

  it("opens Exercises, and says aloud that the locked entries come soon", async () => {
    const api = fakeApi();
    const router = renderApp(api, "/menu");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: /Leçons/ }));
    expect(api.speak).toHaveBeenCalledWith("Cette partie arrive bientôt.");
    expect(router.state.location.pathname).toBe("/menu");

    await user.click(screen.getByRole("button", { name: /Exercices/ }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/exercices"));
  });

  it("moves between the entries with the arrow keys", async () => {
    renderApp(fakeApi(), "/menu");
    const user = userEvent.setup();
    const exercises = await screen.findByRole("button", { name: /Exercices/ });
    exercises.focus();

    await user.keyboard("{ArrowDown}");
    expect(screen.getByRole("button", { name: /Leçons/ })).toHaveFocus();
    await user.keyboard("{ArrowUp}{ArrowUp}");
    expect(screen.getByRole("button", { name: /Options/ })).toHaveFocus();
  });

  it("without a chosen profile, goes back to the profile choice", async () => {
    const router = renderApp(fakeApi(), "/menu", null);

    await waitFor(() => expect(router.state.location.pathname).toBe("/"));
  });
});
