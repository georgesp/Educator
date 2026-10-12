import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { useState } from "react";
import { describe, expect, it, vi } from "vitest";
import { NumberPad } from "./NumberPad";

function Harness({ onSubmit, maxLength }: { onSubmit: (value: string) => void; maxLength?: number }) {
  const [value, setValue] = useState("");
  return (
    <>
      <output>{value}</output>
      <NumberPad value={value} onChange={setValue} onSubmit={() => onSubmit(value)} maxLength={maxLength} />
    </>
  );
}

describe("NumberPad", () => {
  it("types digits, erases and validates", async () => {
    const onSubmit = vi.fn();
    render(<Harness onSubmit={onSubmit} />);
    const user = userEvent.setup();

    expect(screen.getByRole("button", { name: "Valider" })).toBeDisabled();
    for (const key of ["1", "2", "3"]) {
      await user.click(screen.getByRole("button", { name: key }));
    }
    await user.click(screen.getByRole("button", { name: "Effacer" }));
    await user.click(screen.getByRole("button", { name: "0" }));
    await user.click(screen.getByRole("button", { name: "Valider" }));

    expect(screen.getByRole("status")).toHaveTextContent("120");
    expect(onSubmit).toHaveBeenCalledWith("120");
  });

  it("lays out the keys in reading order, 1 at the top left", () => {
    render(<Harness onSubmit={vi.fn()} />);
    const keys = screen.getAllByRole("button").map((button) => button.getAttribute("aria-label") ?? button.textContent);
    expect(keys).toEqual(["1", "2", "3", "4", "5", "6", "7", "8", "9", "Effacer", "0", "Valider"]);
  });

  it("tabs through the keys in the same order", async () => {
    render(<Harness onSubmit={vi.fn()} />);
    const user = userEvent.setup();

    const visited: string[] = [];
    for (let i = 0; i < 9; i++) {
      await user.tab();
      visited.push(document.activeElement?.textContent ?? "");
    }
    // Erase and Validate are disabled while the answer is empty, so focus goes from 9 straight to 0.
    await user.tab();
    visited.push(document.activeElement?.textContent ?? "");

    expect(visited).toEqual(["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"]);
  });

  it("stops at the maximum length", async () => {
    render(<Harness onSubmit={vi.fn()} maxLength={2} />);
    const user = userEvent.setup();

    for (const key of ["4", "5", "6"]) {
      await user.click(screen.getByRole("button", { name: key }));
    }

    expect(screen.getByRole("status")).toHaveTextContent("45");
  });
});
