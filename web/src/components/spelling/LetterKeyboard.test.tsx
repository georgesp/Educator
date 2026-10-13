import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { useState } from "react";
import { describe, expect, it, vi } from "vitest";
import { ACCENTED_LETTERS, LetterKeyboard } from "./LetterKeyboard";

function Harness({ onSubmit, withSpace = false }: { onSubmit(value: string): void; withSpace?: boolean }) {
  const [value, setValue] = useState("");
  return (
    <>
      <output aria-label="valeur">{value}</output>
      <LetterKeyboard value={value} onChange={setValue} onSubmit={() => onSubmit(value)} maxLength={5} withSpace={withSpace} />
    </>
  );
}

describe("LetterKeyboard", () => {
  it("types letters and accented letters with the mouse, erases, then validates", async () => {
    const onSubmit = vi.fn();
    render(<Harness onSubmit={onSubmit} />);
    const user = userEvent.setup();

    expect(screen.getByRole("button", { name: "Valider" })).toBeDisabled();
    await user.click(screen.getByRole("button", { name: "Lettre é" }));
    await user.click(screen.getByRole("button", { name: "Lettre t" }));
    await user.click(screen.getByRole("button", { name: "Lettre t" }));
    await user.click(screen.getByRole("button", { name: "Effacer" }));
    await user.click(screen.getByRole("button", { name: "Lettre é" }));
    expect(screen.getByLabelText("valeur")).toHaveTextContent("été");

    await user.click(screen.getByRole("button", { name: "Valider" }));
    expect(onSubmit).toHaveBeenCalledWith("été");
  });

  it("stops at the longest word and has every accented letter of French", async () => {
    render(<Harness onSubmit={vi.fn()} withSpace />);
    const user = userEvent.setup();

    for (let i = 0; i < 7; i++) {
      await user.click(screen.getByRole("button", { name: "Lettre a" }));
    }
    expect(screen.getByLabelText("valeur")).toHaveTextContent("aaaaa");
    for (const letter of ["é", "è", "ê", "à", "ç", "ù", "ï", "î", "ô"]) {
      expect(ACCENTED_LETTERS).toContain(letter);
    }
    expect(screen.getByRole("button", { name: "Espace" })).toBeInTheDocument();
  });
});
