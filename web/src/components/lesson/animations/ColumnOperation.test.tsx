import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import type { WrittenColumn, WrittenOperation } from "../../../api/client";
import { ColumnOperation } from "./ColumnOperation";

const col = (digits: (number | null)[], terms: number[], value: number, digit: number, carry = 0): WrittenColumn => ({
  digits,
  carry,
  rewrites: [],
  plusTen: false,
  terms,
  value,
  digit,
  written: true,
});

// 34 + 48 = 82, as computed by the core.
const addition: WrittenOperation[] = [{ method: null, columns: [col([4, 8], [4, 8], 12, 2), col([3, 4], [1, 3, 4], 8, 8, 1)] }];

const result = (container: HTMLElement) =>
  Array.from(container.querySelectorAll(".written-result"))
    .map((c) => c.textContent)
    .join("");

describe("ColumnOperation", () => {
  it("sets the numbers in columns without computing them when asked", () => {
    const { container } = render(
      <ColumnOperation operator="Add" operands={[34, 48]} result={82} layouts={addition} computedColumns={0} interactive={false} />,
    );

    expect(screen.getByRole("img", { name: "Opération posée : 34 + 48 = 82" })).toBeInTheDocument();
    expect(result(container)).toBe("");
    expect(container.querySelector(".written-mark")?.textContent).toBe("");
  });

  it("lets the child write the result from the units, with a gentle hint and the carry", async () => {
    const onComplete = vi.fn();
    const { container } = render(
      <ColumnOperation operator="Add" operands={[34, 48]} result={82} layouts={addition} computedColumns={2} interactive onComplete={onComplete} />,
    );
    const user = userEvent.setup();

    expect(screen.getByText("Quel chiffre écris-tu dans la colonne des unités ?")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Chiffre 1" }));
    expect(screen.getByText("Presque ! 4 + 8 = 12 : j'écris 2 et je retiens 1.")).toBeInTheDocument();
    expect(result(container)).toBe("");

    await user.click(screen.getByRole("button", { name: "Chiffre 2" }));
    expect(result(container)).toBe("2");
    // The carry is now above the tens.
    expect(Array.from(container.querySelectorAll(".written-mark")).map((m) => m.textContent)).toContain("1");
    expect(onComplete).not.toHaveBeenCalled();

    await user.click(screen.getByRole("button", { name: "Chiffre 8" }));
    expect(result(container)).toBe("82");
    expect(screen.getByText("Bravo ! 34 + 48 = 82")).toBeInTheDocument();
    expect(onComplete).toHaveBeenCalledOnce();
    expect(screen.getByRole("button", { name: "Chiffre 0" })).toBeDisabled();
  });

  it("shows the broken ten of a cassage while the child computes the units", () => {
    // 63 − 27 by cassage: the 6 becomes 5, the 3 becomes 13.
    const layouts: WrittenOperation[] = [
      {
        method: "Cassage",
        columns: [
          { ...col([3, 7], [13, 7], 6, 6), rewrites: [{ atColumn: 0, value: 13 }] },
          { ...col([6, 2], [5, 2], 3, 3), rewrites: [{ atColumn: 0, value: 5 }] },
        ],
      },
    ];
    const { container } = render(
      <ColumnOperation operator="Subtract" operands={[63, 27]} result={36} layouts={layouts} computedColumns={2} interactive />,
    );

    expect(container.querySelectorAll(".written-crossed")).toHaveLength(2);
    expect(Array.from(container.querySelectorAll(".written-mark")).map((m) => m.textContent)).toEqual(["5", "13"]);
  });

  it("lets the child compute with the two techniques side by side", async () => {
    // 63 − 27: cassage (5 − 2 at the tens) and compensation (6 − 3 at the tens).
    const layouts: WrittenOperation[] = [
      {
        method: "Cassage",
        columns: [
          { ...col([3, 7], [13, 7], 6, 6), rewrites: [{ atColumn: 0, value: 13 }] },
          { ...col([6, 2], [5, 2], 3, 3), rewrites: [{ atColumn: 0, value: 5 }] },
        ],
      },
      {
        method: "Compensation",
        columns: [
          { ...col([3, 7], [13, 7], 6, 6), plusTen: true },
          { ...col([6, 2], [6, 3], 3, 3), carry: 1 },
        ],
      },
    ];
    const onComplete = vi.fn();
    const { container } = render(
      <ColumnOperation operator="Subtract" operands={[63, 27]} result={36} layouts={layouts} computedColumns={2} interactive onComplete={onComplete} />,
    );
    const user = userEvent.setup();

    expect(screen.getByText("Le cassage")).toBeInTheDocument();
    expect(screen.getByText("La compensation")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Chiffre 6" }));
    await user.click(screen.getByRole("button", { name: "Chiffre 4" }));
    expect(screen.getByText("Presque ! Avec le cassage, 5 − 2 = 3 ; avec la compensation, 6 − 3 = 3. J'écris 3.")).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Chiffre 3" }));
    expect(onComplete).toHaveBeenCalledOnce();
    // Both grids show the result.
    const results = Array.from(container.querySelectorAll(".written-grid")).map((g) =>
      Array.from(g.querySelectorAll(".written-result"))
        .map((c) => c.textContent)
        .join(""),
    );
    expect(results).toEqual(["36", "36"]);
  });
});
