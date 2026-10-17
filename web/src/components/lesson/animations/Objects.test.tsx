import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import type { FrameSlot } from "../../../api/client";
import { BarModel } from "./BarModel";
import { FractionBar } from "./FractionBar";
import { GroupObjects } from "./GroupObjects";
import { TenFrame } from "./TenFrame";

const objects = {
  object: "Car" as const,
  counts: [],
  total: 0,
  removed: 0,
  size: 0,
  groupings: [],
  numbers: [],
  parities: [],
  commute: false,
  choices: [],
};

describe("GroupObjects", () => {
  it("lets the child bring the cars to the line, then count them", async () => {
    const onComplete = vi.fn();
    render(<GroupObjects {...objects} mode="Join" counts={[4, 3]} result={7} interactive choices={[6, 7, 8]} onComplete={onComplete} />);
    const user = userEvent.setup();

    for (let i = 0; i < 7; i += 1) {
      await user.click(screen.getAllByRole("button", { name: "voiture" })[0]);
    }
    expect(screen.getByText("Combien y en a-t-il en tout ?")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "8" }));
    expect(screen.getByText("Presque ! Compte encore une fois, doucement.")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "7" }));
    expect(onComplete).toHaveBeenCalledOnce();
    expect(screen.getByText("Bravo ! 4 + 3 = 7")).toBeInTheDocument();
  });

  it("lets the child make the packets of 10, then read the number (SUB001410)", async () => {
    const onComplete = vi.fn();
    render(<GroupObjects {...objects} mode="GroupBy" counts={[23]} size={10} result={23} interactive choices={[13, 23, 32]} onComplete={onComplete} />);
    const user = userEvent.setup();

    expect(screen.getByRole("status")).toHaveTextContent("Clique sur les objets pour faire un paquet de 10.");
    for (let i = 0; i < 4; i += 1) {
      await user.click(screen.getAllByRole("button", { name: "voiture" })[0]);
    }
    expect(screen.getByRole("status")).toHaveTextContent("Encore 6 pour finir le paquet de 10 !");
    for (let i = 0; i < 16; i += 1) {
      await user.click(screen.getAllByRole("button", { name: "voiture" })[0]);
    }
    // Two packets: the 3 left stay alone, they cannot make a packet.
    expect(screen.getByRole("group", { name: "2 paquets de 10" })).toBeInTheDocument();
    expect(screen.getAllByRole("button", { name: "voiture" })).toHaveLength(3);
    expect(screen.getByText("2 paquets de 10 et 3 tout seuls.")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "32" }));
    expect(screen.getByText("Presque ! Compte encore une fois, doucement.")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "23" }));
    expect(onComplete).toHaveBeenCalledOnce();
    expect(screen.getByText("Bravo ! 2 dizaines et 3 unités : 23.")).toBeInTheDocument();
  });

  it("lets the child share the objects one by one, then say how many each part has (SUB012215)", async () => {
    const onComplete = vi.fn();
    render(<GroupObjects {...objects} mode="Share" object="Helmet" total={12} size={4} result={3} interactive choices={[2, 3, 4]} onComplete={onComplete} />);
    const user = userEvent.setup();

    expect(screen.getByText("Distribue les casques : à chaque clic, le suivant va dans la part suivante, chacun son tour.")).toBeInTheDocument();
    for (let i = 0; i < 5; i += 1) {
      await user.click(screen.getAllByRole("button", { name: "casque" })[0]);
    }
    // Given in turn: the first part has 2, the others 1.
    expect(screen.getByRole("group", { name: "Part 1 : 2 casques" })).toBeInTheDocument();
    expect(screen.getByRole("group", { name: "Part 4 : 1 casque" })).toBeInTheDocument();
    for (let i = 0; i < 7; i += 1) {
      await user.click(screen.getAllByRole("button", { name: "casque" })[0]);
    }
    expect(screen.queryAllByRole("button", { name: "casque" })).toHaveLength(0);
    expect(screen.getByText("Combien chaque part en a-t-elle ?")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "4" }));
    expect(onComplete).not.toHaveBeenCalled();
    await user.click(screen.getByRole("button", { name: "3" }));
    expect(onComplete).toHaveBeenCalledOnce();
    expect(screen.getByText("Bravo ! 12 partagé en 4 parts égales : 3 dans chaque part.")).toBeInTheDocument();
  });

  it("lets the child park the numbers in the even and odd garages", async () => {
    const onComplete = vi.fn();
    render(<GroupObjects {...objects} mode="SortParity" numbers={[768, 773]} parities={[true, false]} result={1} interactive onComplete={onComplete} />);
    const user = userEvent.setup();

    await user.click(screen.getByRole("button", { name: "773" }));
    await user.click(screen.getByRole("button", { name: /^Pair/ }));
    expect(screen.getByText("Presque ! Regarde le chiffre des unités de 773.")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: /^Impair/ }));
    await user.click(screen.getByRole("button", { name: "768" }));
    await user.click(screen.getByRole("button", { name: /^Pair/ }));
    expect(onComplete).toHaveBeenCalledOnce();
    expect(screen.getByText("Bravo, tous les nombres sont garés !")).toBeInTheDocument();
  });
});

describe("TenFrame", () => {
  const slots = (filled: number, other: FrameSlot, count: number): FrameSlot[] =>
    Array.from({ length: 10 }, (_, i) => (i < filled ? (i >= filled - (other === "Removed" ? count : 0) ? other : "Filled") : i < filled + (other === "Added" ? count : 0) ? other : "Empty"));

  it("lets the child park the missing cars", async () => {
    const onComplete = vi.fn();
    render(<TenFrame frames={1} slots={slots(7, "Added", 3)} filled={7} added={3} removed={0} total={10} friends={[]} interactive onComplete={onComplete} />);
    const user = userEvent.setup();

    for (let i = 0; i < 3; i += 1) {
      await user.click(screen.getByRole("button", { name: "Place 10" }));
    }
    expect(onComplete).toHaveBeenCalledOnce();
    expect(screen.getByText("Bravo ! 7 + 3 = 10")).toBeInTheDocument();
  });

  it("lets the child make cars leave", async () => {
    const onComplete = vi.fn();
    render(<TenFrame frames={1} slots={slots(8, "Removed", 5)} filled={8} added={0} removed={5} total={3} friends={[]} interactive onComplete={onComplete} />);
    const user = userEvent.setup();

    for (let i = 1; i <= 5; i += 1) {
      await user.click(screen.getByRole("button", { name: `Place ${i}` }));
    }
    expect(onComplete).toHaveBeenCalledOnce();
    expect(screen.getByText("Bravo ! 8 − 5 = 3")).toBeInTheDocument();
  });
});

describe("FractionBar", () => {
  it("lets the child color a quarter", async () => {
    const onComplete = vi.fn();
    render(
      <FractionBar
        bars={[{ denominator: 4, numerators: [1], colored: 1, label: "1/4", words: "un quart" }]}
        comparison={null}
        unequal={false}
        notation={false}
        interactive
        onComplete={onComplete}
      />,
    );
    const user = userEvent.setup();

    expect(screen.getAllByRole("button")).toHaveLength(4);
    await user.click(screen.getByRole("button", { name: "Part 2" }));
    expect(onComplete).toHaveBeenCalledOnce();
    expect(screen.getByText("Bravo ! Tu as colorié un quart.")).toBeInTheDocument();
  });
});

describe("BarModel", () => {
  it("lets the child place the numbers of the story", async () => {
    const onComplete = vi.fn();
    render(
      <BarModel
        kind="Parts"
        values={[
          { part: null, value: 42, unknown: false },
          { part: 0, value: 24, unknown: false },
          { part: 1, value: 18, unknown: true },
        ]}
        groups={0}
        labels={[]}
        answer={18}
        solution={{ left: 42, operator: "Subtract", right: 24, result: 18 }}
        solutionMissing={null}
        solve
        interactive
        onComplete={onComplete}
      />,
    );
    const user = userEvent.setup();
    const whole = () => screen.getByRole("button", { name: "Place du nombre en tout" });
    const part = () => screen.getByRole("button", { name: "Place de la partie 1" });

    // 24 does not go in the whole.
    await user.click(screen.getByRole("button", { name: "24" }));
    await user.click(whole());
    expect(screen.getByText("Presque ! Relis l'histoire : où va ce nombre ?")).toBeInTheDocument();
    await user.click(part());
    await user.click(screen.getByRole("button", { name: "42" }));
    await user.click(whole());
    expect(onComplete).toHaveBeenCalledOnce();
    expect(screen.getByText("42 − 24 = 18")).toBeInTheDocument();
  });
});
