import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { NumberLine } from "./NumberLine";

const jumps = [
  { delta: 20, value: 77 },
  { delta: -1, value: 76 },
];

const hoverValue = (container: HTMLElement) => container.querySelector(".line-hover-text")?.textContent ?? null;

describe("NumberLine", () => {
  it("shows the starting point in its own color", () => {
    const { container } = render(<NumberLine from={55} to={80} step={1} start={57} jumps={jumps} interactive hoverValues />);

    expect(container.querySelector(".line-start")).not.toBeNull();
    expect(container.querySelector(".line-label-start")?.textContent).toBe("57");
  });

  it("shows the value of the graduation under the pointer", async () => {
    const { container } = render(<NumberLine from={55} to={80} step={1} start={57} jumps={jumps} interactive hoverValues />);
    const user = userEvent.setup();

    expect(hoverValue(container)).toBeNull();
    await user.hover(screen.getByRole("button", { name: "Graduation 73" }));
    expect(hoverValue(container)).toBe("73");
    await user.unhover(screen.getByRole("button", { name: "Graduation 73" }));
    expect(hoverValue(container)).toBeNull();
  });

  it("hides the values when finding the number is the lesson, and once the line is done", async () => {
    const hidden = render(<NumberLine from={55} to={80} step={1} start={57} jumps={jumps} interactive hoverValues={false} />);
    const user = userEvent.setup();
    await user.hover(screen.getByRole("button", { name: "Graduation 73" }));
    expect(hoverValue(hidden.container)).toBeNull();
    hidden.unmount();

    const { container } = render(<NumberLine from={55} to={80} step={1} start={57} jumps={jumps} interactive hoverValues />);
    await user.click(screen.getByRole("button", { name: "Graduation 77" }));
    await user.click(screen.getByRole("button", { name: "Graduation 76" }));
    await user.hover(screen.getByRole("button", { name: "Graduation 70" }));
    expect(hoverValue(container)).toBeNull();
  });
});
