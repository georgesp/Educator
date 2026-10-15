import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { ChoiceGrid } from "./ChoiceGrid";

describe("ChoiceGrid", () => {
  it("shows every choice and reports the picked one", async () => {
    const onPick = vi.fn();
    render(<ChoiceGrid labels={["11", "12", "13", "22"]} onPick={onPick} label="Choisis" />);

    expect(screen.getAllByRole("button")).toHaveLength(4);
    await userEvent.click(screen.getByRole("button", { name: "13" }));

    expect(onPick).toHaveBeenCalledWith(2);
  });

  it("after the answer, marks the right and the wrong tiles and blocks new clicks", async () => {
    const onPick = vi.fn();
    render(<ChoiceGrid labels={["11", "12", "13", "22"]} onPick={onPick} picked={2} expected={1} disabled label="Choisis" />);

    expect(screen.getByRole("button", { name: "12" })).toHaveClass("choice-right");
    expect(screen.getByRole("button", { name: "13" })).toHaveClass("choice-wrong");
    expect(screen.getByRole("button", { name: "13" })).toHaveAttribute("aria-pressed", "true");
    await userEvent.click(screen.getByRole("button", { name: "11" }));
    expect(onPick).not.toHaveBeenCalled();
  });

  it("draws fractions with the numerator above the denominator", () => {
    render(<ChoiceGrid labels={["3/4", "4/3"]} onPick={vi.fn()} label="Choisis" />);

    expect(screen.getByRole("button", { name: "3/4" })).toHaveTextContent("34");
    expect(document.querySelectorAll(".fraction-top")).toHaveLength(2);
  });

  it("uses the word layout for spelling words, and keeps the large tiles for numbers and « oui » / « non »", () => {
    const { container, rerender } = render(<ChoiceGrid labels={["souvenx", "souvent", "souvens"]} onPick={vi.fn()} label="Choisis" />);
    expect(container.querySelector(".choice-grid")).toHaveClass("choice-grid-words");

    rerender(<ChoiceGrid labels={["11", "12", "13", "22"]} onPick={vi.fn()} label="Choisis" />);
    expect(container.querySelector(".choice-grid")).not.toHaveClass("choice-grid-words");

    rerender(<ChoiceGrid labels={["oui", "non"]} onPick={vi.fn()} label="Choisis" />);
    expect(container.querySelector(".choice-grid")).not.toHaveClass("choice-grid-words");
  });
});
