import { act, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { Fireworks, FIREWORKS_DURATION_MS } from "./Fireworks";

function preferReducedMotion(reduce: boolean) {
  vi.stubGlobal(
    "matchMedia",
    vi.fn((query: string) => ({
      matches: reduce && query.includes("prefers-reduced-motion"),
      media: query,
      onchange: null,
      addListener: vi.fn(),
      removeListener: vi.fn(),
      addEventListener: vi.fn(),
      removeEventListener: vi.fn(),
      dispatchEvent: vi.fn(),
    })),
  );
}

afterEach(() => {
  vi.unstubAllGlobals();
  vi.useRealTimers();
});

describe("Fireworks", () => {
  it("bursts over the screen without catching clicks, then goes away", () => {
    vi.useFakeTimers();
    const onDone = vi.fn();
    render(<Fireworks label="Bravo, tout est juste !" onDone={onDone} />);

    const fireworks = screen.getByRole("img", { name: "Bravo, tout est juste !" });
    expect(fireworks).toHaveClass("fireworks");
    expect(fireworks.querySelectorAll(".fireworks-spark").length).toBeGreaterThan(20);

    act(() => vi.advanceTimersByTime(FIREWORKS_DURATION_MS));
    expect(screen.queryByRole("img")).not.toBeInTheDocument();
    expect(onDone).toHaveBeenCalledOnce();
  });

  it("ends on time even when the screen gives a new callback at each render", () => {
    vi.useFakeTimers();
    const calls: string[] = [];
    const { rerender } = render(<Fireworks label="Bravo" onDone={() => calls.push("first")} />);

    act(() => vi.advanceTimersByTime(FIREWORKS_DURATION_MS - 100));
    rerender(<Fireworks label="Bravo" onDone={() => calls.push("second")} />);
    act(() => vi.advanceTimersByTime(100));

    expect(screen.queryByRole("img")).not.toBeInTheDocument();
    expect(calls).toEqual(["second"]);
  });

  it("shows still stars when the system asks for less motion", () => {
    preferReducedMotion(true);
    render(<Fireworks label="Bravo, tout est juste !" />);

    const still = screen.getByRole("img", { name: "Bravo, tout est juste !" });
    expect(still).toHaveClass("fireworks-still");
    expect(still.querySelectorAll("svg")).toHaveLength(3);
    expect(still.querySelectorAll(".fireworks-spark")).toHaveLength(0);
  });
});
