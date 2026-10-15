import { act, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { QuestionTimer } from "./QuestionTimer";

describe("QuestionTimer (SUB001418)", () => {
  afterEach(() => vi.useRealTimers());

  it("shows the time given and calls onOver once, when it is over", async () => {
    vi.useFakeTimers();
    const onOver = vi.fn();
    render(<QuestionTimer seconds={5} running onOver={onOver} />);

    expect(screen.getByRole("timer", { name: "Temps pour répondre : 5 secondes" })).toBeInTheDocument();
    expect(screen.getByText("5 s")).toBeInTheDocument();

    await act(async () => {
      vi.advanceTimersByTime(4_900);
    });
    expect(onOver).not.toHaveBeenCalled();
    expect(screen.getByText("1 s")).toBeInTheDocument();

    await act(async () => {
      vi.advanceTimersByTime(300);
    });
    expect(onOver).toHaveBeenCalledTimes(1);

    await act(async () => {
      vi.advanceTimersByTime(5_000);
    });
    expect(onOver).toHaveBeenCalledTimes(1);
  });

  it("waits while it is paused (a question being read) and goes on from where it was", async () => {
    vi.useFakeTimers();
    const onOver = vi.fn();
    const { rerender } = render(<QuestionTimer seconds={2} running={false} onOver={onOver} />);

    await act(async () => {
      vi.advanceTimersByTime(10_000);
    });
    expect(onOver).not.toHaveBeenCalled();
    expect(screen.getByText("2 s")).toBeInTheDocument();

    rerender(<QuestionTimer seconds={2} running onOver={onOver} />);
    await act(async () => {
      vi.advanceTimersByTime(1_900);
    });
    expect(onOver).not.toHaveBeenCalled();

    // Paused again: the remaining time is kept.
    rerender(<QuestionTimer seconds={2} running={false} onOver={onOver} />);
    await act(async () => {
      vi.advanceTimersByTime(10_000);
    });
    expect(onOver).not.toHaveBeenCalled();

    rerender(<QuestionTimer seconds={2} running onOver={onOver} />);
    await act(async () => {
      vi.advanceTimersByTime(200);
    });
    expect(onOver).toHaveBeenCalledTimes(1);
  });
});
