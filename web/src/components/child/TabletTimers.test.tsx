import { act, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { QuestionTimer } from "./QuestionTimer";
import { TimerBar } from "./TimerBar";

function goTo(state: "hidden" | "visible") {
  Object.defineProperty(document, "visibilityState", { value: state, configurable: true });
  document.dispatchEvent(new Event("visibilitychange"));
}

describe("timers and the background (SUB023811)", () => {
  afterEach(() => {
    goTo("visible");
    vi.useRealTimers();
  });

  it("the time of a question stops while the application is in the background", async () => {
    vi.useFakeTimers();
    const onOver = vi.fn();
    render(<QuestionTimer seconds={5} running onOver={onOver} />);

    await act(async () => {
      vi.advanceTimersByTime(2_000);
    });
    act(() => goTo("hidden"));
    await act(async () => {
      vi.advanceTimersByTime(30_000);
    });
    act(() => goTo("visible"));
    await act(async () => {
      vi.advanceTimersByTime(100);
    });

    expect(onOver).not.toHaveBeenCalled();
    expect(screen.getByText("3 s")).toBeInTheDocument();
  });

  it("a timed challenge whose time ran out in the background ends on return", () => {
    vi.useFakeTimers();
    const onOver = vi.fn();
    render(<TimerBar seconds={60} rightCount={0} onOver={onOver} />);

    act(() => goTo("hidden"));
    // The application was suspended: the clock went on, the timer did not fire.
    vi.setSystemTime(Date.now() + 61_000);
    act(() => goTo("visible"));

    expect(onOver).toHaveBeenCalled();
  });
});
