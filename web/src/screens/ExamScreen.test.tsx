import { act, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { SeriesSummary } from "../api/client";
import { fakeApi, renderApp } from "../test/renderWithApp";
import { AUTO_NEXT_DELAY_MS } from "./SeriesScreen";

const examSummary: SeriesSummary = {
  correctCount: 1,
  exerciseCount: 2,
  pointsEarned: 30,
  pointsPercent: 100,
  profilePoints: 150,
  skills: [
    { skillId: "cp-add-sum-10", title: "Additions jusqu'à 10", correctCount: 1, questionCount: 1, lessonId: "cp-add-sum-10", strong: true },
    { skillId: "cp-add-tables", title: "Tables d'addition", correctCount: 0, questionCount: 1, lessonId: "cp-add-tables", strong: false },
  ],
  mascotSituation: "SeriesEncourage",
  previousBest: null,
  newRecord: false,
  reviewLesson: false,
  rewards: null,
};

describe("ExamScreen (SUB001414, SUB001415)", () => {
  afterEach(() => vi.useRealTimers());

  it("le bilan: the questions, then the strong points and what to work on, with links", async () => {
    const api = fakeApi({
      answerSeries: vi.fn(async (_s: string, index: number) => ({
        correct: index === 0,
        expected: { kind: "number" as const, value: 7 },
        expectedLabel: "7",
        explanation: "3 + 4 = 7",
        help: null,
        correction: null,
        summary: index === 1 ? examSummary : null,
      })),
    });
    const router = renderApp(api, "/examen");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: /Bilan d'orthographe/ }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/examen/orthographe"));
    expect(api.startExam).toHaveBeenCalledWith("p-lea", "Spelling");

    await user.type(await screen.findByLabelText("Ta réponse"), "7{Enter}");
    await waitFor(() => expect(screen.getByLabelText("5 + ? = 9")).toBeInTheDocument(), { timeout: AUTO_NEXT_DELAY_MS + 1000 });
    await user.type(screen.getByLabelText("Ta réponse"), "1{Enter}");
    await user.click(await screen.findByRole("button", { name: "Suivant" }));

    expect(await screen.findByRole("heading", { name: "Bilan terminé !" })).toBeInTheDocument();
    const strong = screen.getByRole("region", { name: "Tes points forts" });
    expect(within(strong).getByText("Additions jusqu'à 10")).toBeInTheDocument();
    const rework = screen.getByRole("region", { name: "À retravailler" });
    expect(within(rework).getByText("Tables d'addition")).toBeInTheDocument();
    await user.click(within(rework).getByRole("button", { name: /Revoir la leçon/ }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/lecons/orthographe/cp-add-tables"));
  });

  it("le défi chrono: the duration with its record, then the questions until the time is over", async () => {
    const api = fakeApi();
    const router = renderApp(api, "/examen");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: /Défi chrono de maths/ }));
    expect(await screen.findByRole("button", { name: /2 minutes.*Ton record : 7/ })).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: /1 minute/ }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/defi/maths/60"));
    expect(api.startTimed).toHaveBeenCalledWith("p-lea", "Math", 60);
    expect(await screen.findByRole("timer")).toBeInTheDocument();
    // No marker per question: the bar of time only.
    expect(screen.queryByRole("progressbar")).not.toBeInTheDocument();
  });

  it("le défi chrono ends by itself when the time is over, without penalty", async () => {
    vi.useFakeTimers({ shouldAdvanceTime: true });
    const api = fakeApi();
    renderApp(api, "/defi/maths/60");

    await screen.findByRole("timer");
    await act(async () => {
      vi.advanceTimersByTime(61_000);
    });
    await waitFor(() => expect(api.finishSeries).toHaveBeenCalledWith("s1"));
    expect(await screen.findByRole("heading", { name: "Temps écoulé !" })).toBeInTheDocument();
    expect(screen.getByText("Nouveau record !", { selector: "p" })).toBeInTheDocument();
  });

  it("le défi chrono: no new record when the commands say the score is below the best one", async () => {
    vi.useFakeTimers({ shouldAdvanceTime: true });
    const api = fakeApi({
      finishSeries: vi.fn().mockResolvedValue({ ...examSummary, skills: [], previousBest: 7, newRecord: false }),
    });
    renderApp(api, "/defi/maths/60");

    await screen.findByRole("timer");
    await act(async () => {
      vi.advanceTimersByTime(61_000);
    });
    expect(await screen.findByRole("heading", { name: "Temps écoulé !" })).toBeInTheDocument();
    expect(screen.queryByText("Nouveau record !", { selector: "p" })).not.toBeInTheDocument();
    expect(screen.getByText(/Ton record : 7/)).toBeInTheDocument();
  });

  it("le défi chrono stays closed while no skill is mastered", async () => {
    const api = fakeApi({
      getTimedChallenge: vi.fn().mockResolvedValue({ subject: "Math", skillCount: 0, durations: [], defaultSeconds: 120 }),
    });
    renderApp(api, "/defi/maths");

    expect(await screen.findByText(/Le défi chrono s'ouvre quand tu as bien réussi/)).toBeInTheDocument();
  });
});
