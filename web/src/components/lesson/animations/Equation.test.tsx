import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import type { Equality } from "../../../api/client";
import { Equation } from "./Equation";

const family: Equality[] = [
  { left: 7, operator: "Add", right: 8, result: 15 },
  { left: 8, operator: "Add", right: 7, result: 15 },
  { left: 15, operator: "Subtract", right: 7, result: 8 },
  { left: 15, operator: "Subtract", right: 8, result: 7 },
];

describe("Equation", () => {
  it("lets the child find the box, then shows the family", async () => {
    const onComplete = vi.fn();
    render(
      <Equation
        left={7}
        operator="Add"
        right={8}
        result={15}
        missing="Right"
        missingValue={8}
        equalities={family}
        interactive
        choices={[7, 8, 9]}
        onComplete={onComplete}
      />,
    );
    const user = userEvent.setup();

    expect(screen.getByLabelText("Case à trouver")).toHaveTextContent("?");
    await user.click(screen.getByRole("button", { name: "9" }));
    expect(screen.getByText("Presque ! Combien faut-il ajouter à 7 pour arriver à 15 ?")).toBeInTheDocument();
    expect(onComplete).not.toHaveBeenCalled();

    await user.click(screen.getByRole("button", { name: "8" }));
    expect(onComplete).toHaveBeenCalledOnce();
    expect(screen.getByText("Bravo ! 7 + 8 = 15")).toBeInTheDocument();
    expect(await screen.findByText("15 − 8 = 7", {}, { timeout: 4000 })).toBeInTheDocument();
  });

  it("shows a plain equality without box nor choices", () => {
    render(
      <Equation left={46} operator="Add" right={28} result={74} missing={null} missingValue={null} equalities={family.slice(0, 1)} interactive={false} choices={[]} />,
    );

    expect(screen.queryByLabelText("Case à trouver")).not.toBeInTheDocument();
    expect(screen.queryAllByRole("button")).toHaveLength(0);
    expect(screen.getByText("74")).toBeInTheDocument();
  });
});
