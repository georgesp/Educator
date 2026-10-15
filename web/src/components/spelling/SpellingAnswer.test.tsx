import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { SpellingAnswer } from "./SpellingAnswer";

describe("SpellingAnswer on a tablet (SUB023810)", () => {
  it.each([
    ["a whole word", null, null],
    ["missing letters", "bat", "au"],
  ])("the field of %s is never corrected nor capitalized by the system", (_, before, after) => {
    render(<SpellingAnswer before={before} after={after} missing={before ? 3 : null} maxLength={20} disabled={false} onSubmit={vi.fn()} />);

    const field = screen.getByRole("textbox");
    expect(field).toHaveAttribute("autocorrect", "off");
    expect(field).toHaveAttribute("autocapitalize", "off");
    expect(field).toHaveAttribute("spellcheck", "false");
    expect(field).toHaveAttribute("autocomplete", "off");
    expect(field).toHaveAttribute("enterkeyhint", "done");
  });
});
