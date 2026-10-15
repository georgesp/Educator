import { act, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { PlacementStatus, SeriesSummary } from "../api/client";
import { fakeApi, renderApp } from "../test/renderWithApp";
import { AUTO_NEXT_DELAY_MS, TIMED_WRONG_DELAY_MS } from "./SeriesScreen";

const both: PlacementStatus = {
  subjects: [
    { subject: "Math", available: true, done: false },
    { subject: "Spelling", available: true, done: false },
  ],
  questionSeconds: 30,
};

const spellingLeft: PlacementStatus = {
  subjects: [
    { subject: "Math", available: false, done: true },
    { subject: "Spelling", available: true, done: false },
  ],
  questionSeconds: 30,
};

const summary: SeriesSummary = {
  correctCount: 1,
  exerciseCount: 2,
  pointsEarned: 0,
  profilePoints: 120,
  skills: [
    { skillId: "cp-add-sum-10", title: "Additions jusqu'à 10", correctCount: 1, questionCount: 1, lessonId: "cp-add-sum-10", strong: true },
    { skillId: "cp-add-tables", title: "Tables d'addition", correctCount: 0, questionCount: 1, lessonId: "cp-add-tables", strong: false },
  ],
  mascotSituation: "ExamEnd",
  previousBest: null,
  newRecord: false,
  reviewLesson: false,
};

describe("PlacementScreen (SUB001418)", () => {
  afterEach(() => vi.useRealTimers());

  it("offers the assessment of each subject still to do, with the time given", async () => {
    const api = fakeApi({ getPlacementStatus: vi.fn().mockResolvedValue(both) });
    const router = renderApp(api, "/evaluation");
    const user = userEvent.setup();

    expect(await screen.findByRole("heading", { name: "Évaluation de niveau" })).toBeInTheDocument();
    expect(screen.getByText(/30 secondes pour répondre/)).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Évaluation d'orthographe/ })).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: /Évaluation de maths/ }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/evaluation/maths"));
    expect(api.startPlacement).toHaveBeenCalledWith("p-lea", "Math");
  });

  it("only offers the subject that is not done yet, and « Plus tard » leads to the menu", async () => {
    const api = fakeApi({ getPlacementStatus: vi.fn().mockResolvedValue(spellingLeft) });
    const router = renderApp(api, "/evaluation");
    const user = userEvent.setup();

    expect(await screen.findByRole("button", { name: /Évaluation d'orthographe/ })).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: /Évaluation de maths/ })).not.toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Plus tard" }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/menu"));
  });

  it("goes straight to the menu when there is nothing left to assess", async () => {
    const router = renderApp(fakeApi(), "/evaluation");

    await waitFor(() => expect(router.state.location.pathname).toBe("/menu"));
  });

  it("a question left without answer is wrong, then the next one comes by itself", async () => {
    vi.useFakeTimers({ shouldAdvanceTime: true });
    const api = fakeApi();
    renderApp(api, "/evaluation/maths");

    expect(await screen.findByRole("timer", { name: "Temps pour répondre : 20 secondes" })).toBeInTheDocument();
    expect(screen.getByLabelText("3 + 4 = ?")).toBeInTheDocument();

    await act(async () => {
      vi.advanceTimersByTime(21_000);
    });
    await waitFor(() => expect(api.answerSeries).toHaveBeenCalledWith("s1", 0, { kind: "timedOut" }, expect.any(Number)));
    expect(await screen.findByText("Temps écoulé !")).toBeInTheDocument();
    // No « Suivant » to click: the assessment goes on by itself.
    expect(screen.queryByRole("button", { name: "Suivant" })).not.toBeInTheDocument();

    await act(async () => {
      vi.advanceTimersByTime(TIMED_WRONG_DELAY_MS + 200);
    });
    expect(await screen.findByLabelText("5 + ? = 9")).toBeInTheDocument();
    // A new time for the new question.
    expect(screen.getByRole("timer", { name: "Temps pour répondre : 20 secondes" })).toBeInTheDocument();
  });

  it("stopping asks for a confirmation that says the assessment will be to do again", async () => {
    const api = fakeApi();
    const router = renderApp(api, "/evaluation/maths");
    const user = userEvent.setup();

    await screen.findByLabelText("3 + 4 = ?");
    await user.click(screen.getByRole("button", { name: "Quitter" }));
    expect(screen.getByText(/L'évaluation sera à refaire/)).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Oui, j'arrête" }));
    await waitFor(() => expect(api.abandonSeries).toHaveBeenCalledWith("s1"));
    await waitFor(() => expect(router.state.location.pathname).toBe("/menu"));
  });

  it("the end shows the review, then goes on with the other subject", async () => {
    const api = fakeApi({
      getPlacementStatus: vi.fn().mockResolvedValue(spellingLeft),
      answerSeries: vi.fn(async () => ({
        correct: true,
        expected: { kind: "number" as const, value: 7 },
        expectedLabel: "7",
        explanation: "3 + 4 = 7",
        help: null,
        correction: null,
        summary,
      })),
    });
    const router = renderApp(api, "/evaluation/maths");
    const user = userEvent.setup();

    await user.type(await screen.findByLabelText("Ta réponse"), "7{Enter}");

    expect(await screen.findByRole("heading", { name: "Évaluation terminée !" }, { timeout: AUTO_NEXT_DELAY_MS + 1000 })).toBeInTheDocument();
    expect(screen.getByRole("region", { name: "Tes points forts" })).toBeInTheDocument();
    expect(screen.getByRole("region", { name: "À retravailler" })).toBeInTheDocument();
    // The assessment brings no point.
    expect(screen.queryByText(/\+\d+ point/)).not.toBeInTheDocument();

    await user.click(await screen.findByRole("button", { name: "Continuer : Évaluation d'orthographe" }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/evaluation/orthographe"));
    expect(api.startPlacement).toHaveBeenLastCalledWith("p-lea", "Spelling");
  });
});
