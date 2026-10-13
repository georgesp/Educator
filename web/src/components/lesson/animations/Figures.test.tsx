import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import type { Blocks, Chain } from "../../../api/client";
import { ArrayGrid } from "./ArrayGrid";
import { CalcChain } from "./CalcChain";
import { CompareNumbers } from "./CompareNumbers";
import { PlaceValueTable } from "./PlaceValueTable";
import { SequencePattern } from "./SequencePattern";
import { TensUnits } from "./TensUnits";

const blocks = (hundreds: number, tens: number, units: number): Blocks => ({ hundreds, tens, units, value: hundreds * 100 + tens * 10 + units });

describe("TensUnits", () => {
  it("lets the child add blocks, and asks for the exchange of 10 cubes", async () => {
    const onComplete = vi.fn();
    render(
      <TensUnits
        hundreds={0}
        tens={3}
        units={6}
        missing={0}
        steps={[]}
        change={27}
        result={63}
        parts={1}
        task="Add"
        interactiveStart={blocks(0, 3, 6)}
        target={blocks(0, 6, 3)}
        onComplete={onComplete}
      />,
    );
    const user = userEvent.setup();

    await user.click(screen.getByRole("button", { name: "+ barre" }));
    await user.click(screen.getByRole("button", { name: "+ barre" }));
    for (let i = 0; i < 7; i += 1) {
      await user.click(screen.getByRole("button", { name: "+ cube" }));
    }
    // 5 bars and 13 cubes: 63, but 10 cubes must become a bar.
    expect(screen.getByText("Tu as 10 cubes ou plus : échange 10 cubes contre une barre !")).toBeInTheDocument();
    expect(onComplete).not.toHaveBeenCalled();

    await user.click(screen.getByRole("button", { name: "Échanger 10 cubes contre 1 barre" }));
    expect(onComplete).toHaveBeenCalledOnce();
    expect(screen.getByText("Bravo ! Tu as fabriqué 63.")).toBeInTheDocument();
  });

  it("lets the child share the blocks, breaking a bar that cannot be shared", async () => {
    const onComplete = vi.fn();
    render(
      <TensUnits
        hundreds={0}
        tens={5}
        units={6}
        missing={0}
        steps={[]}
        change={2}
        result={28}
        parts={2}
        task="Share"
        interactiveStart={blocks(0, 5, 6)}
        target={blocks(0, 2, 8)}
        onComplete={onComplete}
      />,
    );
    const user = userEvent.setup();

    for (let i = 0; i < 4; i += 1) {
      await user.click(screen.getByRole("button", { name: /barre/ }));
    }
    // 2 bars each: the fifth bar cannot be shared, it is broken into 10 cubes.
    expect(screen.getByText("Cette barre ne se partage pas : clique dessus pour la casser en 10 cubes.")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Casser la barre en 10 cubes" }));
    for (let i = 0; i < 16; i += 1) {
      await user.click(screen.getByRole("button", { name: /cube/ }));
    }
    expect(onComplete).toHaveBeenCalledOnce();
    expect(screen.getByText("Bravo ! Chaque part a 28.")).toBeInTheDocument();
  });
});

describe("TensUnits (remove)", () => {
  it("lets the child break a bar to take cubes away", async () => {
    const onComplete = vi.fn();
    render(
      <TensUnits
        hundreds={0}
        tens={3}
        units={0}
        missing={0}
        steps={[]}
        change={7}
        result={23}
        parts={1}
        task="Remove"
        interactiveStart={blocks(0, 3, 0)}
        target={blocks(0, 2, 3)}
        onComplete={onComplete}
      />,
    );
    const user = userEvent.setup();

    await user.click(screen.getByRole("button", { name: "− cube" }));
    expect(screen.getByText("Il n'y a plus assez de cubes : casse une barre.")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Casser une barre en 10 cubes" }));
    for (let i = 0; i < 7; i += 1) {
      await user.click(screen.getByRole("button", { name: "− cube" }));
    }
    expect(onComplete).toHaveBeenCalledOnce();
    expect(screen.getByText("Bravo ! Il reste 23.")).toBeInTheDocument();
  });
});

describe("TensUnits (cases that must never block the child)", () => {
  it("lets the child take back a plate made from 10 bars on a number below 100", async () => {
    const onComplete = vi.fn();
    render(<TensUnits hundreds={0} tens={3} units={6} missing={0} steps={[]} change={27} result={63} parts={1} task="Add" interactiveStart={blocks(0, 3, 6)} target={blocks(0, 6, 3)} onComplete={onComplete} />);
    const user = userEvent.setup();

    for (let i = 0; i < 7; i += 1) {
      await user.click(screen.getByRole("button", { name: "+ barre" }));
    }
    // 10 bars: no exchange for a plate below 100, the child takes bars back.
    expect(screen.queryByRole("button", { name: "Échanger 10 barres contre 1 plaque" })).not.toBeInTheDocument();
    for (let i = 0; i < 5; i += 1) {
      await user.click(screen.getByRole("button", { name: "− barre" }));
    }
    for (let i = 0; i < 7; i += 1) {
      await user.click(screen.getByRole("button", { name: "+ cube" }));
    }
    await user.click(screen.getByRole("button", { name: "Échanger 10 cubes contre 1 barre" }));
    expect(onComplete).toHaveBeenCalledOnce();
  });

  it("refuses to take away more than the subtraction asks", async () => {
    const onComplete = vi.fn();
    render(<TensUnits hundreds={0} tens={3} units={0} missing={0} steps={[]} change={7} result={23} parts={1} task="Remove" interactiveStart={blocks(0, 3, 0)} target={blocks(0, 2, 3)} onComplete={onComplete} />);
    const user = userEvent.setup();

    await user.click(screen.getByRole("button", { name: "− barre" }));
    expect(screen.getByText(/il restera moins que 23/)).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Casser une barre en 10 cubes" }));
    for (let i = 0; i < 7; i += 1) {
      await user.click(screen.getByRole("button", { name: "− cube" }));
    }
    expect(onComplete).toHaveBeenCalledOnce();
  });

  it("breaks the plate before the bars when sharing 120 in 4", async () => {
    const onComplete = vi.fn();
    render(<TensUnits hundreds={1} tens={2} units={0} missing={0} steps={[]} change={4} result={30} parts={4} task="Share" interactiveStart={blocks(1, 2, 0)} target={blocks(0, 3, 0)} onComplete={onComplete} />);
    const user = userEvent.setup();

    // The bars wait while the plate must be broken.
    expect(screen.getByRole("button", { name: "Donner une barre" })).toBeDisabled();
    await user.click(screen.getByRole("button", { name: "Casser la plaque en 10 barres" }));
    for (let i = 0; i < 12; i += 1) {
      await user.click(screen.getByRole("button", { name: "Donner une barre" }));
    }
    expect(onComplete).toHaveBeenCalledOnce();
    expect(screen.getByText("Bravo ! Chaque part a 30.")).toBeInTheDocument();
  });
});

describe("CompareNumbers", () => {
  it("lets the child pick the sign, with a hint on the first rank", async () => {
    const onComplete = vi.fn();
    render(
      <CompareNumbers
        a={607}
        b={670}
        sign="<"
        ranks={[
          { rank: "Hundreds", a: 6, b: 6 },
          { rank: "Tens", a: 0, b: 7 },
          { rank: "Units", a: 7, b: 0 },
        ]}
        decisiveRank="Tens"
        interactive
        onComplete={onComplete}
      />,
    );
    const user = userEvent.setup();

    await user.click(screen.getByRole("button", { name: "est plus grand que" }));
    expect(screen.getByText("Presque ! Regarde d'abord les centaines.")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "est plus petit que" }));
    expect(onComplete).toHaveBeenCalledOnce();
    expect(screen.getByText("Bravo ! 607 < 670")).toBeInTheDocument();
  });
});

describe("SequencePattern", () => {
  it("lets the child put the cars in order", async () => {
    const onComplete = vi.fn();
    render(<SequencePattern mode="Order" items={[483, 348, 438]} ordered={[348, 438, 483]} hidden={[]} step={0} descending={false} interactive onComplete={onComplete} />);
    const user = userEvent.setup();

    await user.click(screen.getByRole("button", { name: /483/ }));
    expect(screen.getByText("Presque ! Cherche le plus petit nombre qui reste.")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: /348/ }));
    await user.click(screen.getByRole("button", { name: /438/ }));
    await user.click(screen.getByRole("button", { name: /483/ }));
    expect(onComplete).toHaveBeenCalledOnce();
    expect(screen.getByText("Bravo, la grille de départ est rangée !")).toBeInTheDocument();
  });
});

describe("ArrayGrid", () => {
  const grid = { total: 12, split: null, parts: [{ rows: 4, cols: 3, product: 12 }], repeated: [], rotate: false, removed: 0, remaining: 12 };

  it("lets the child place the rows", async () => {
    const onComplete = vi.fn();
    render(<ArrayGrid rows={4} cols={3} {...grid} task="Build" onComplete={onComplete} />);
    const user = userEvent.setup();

    for (let i = 0; i < 4; i += 1) {
      await user.click(screen.getByRole("button", { name: "Ajouter une rangée" }));
    }
    expect(onComplete).toHaveBeenCalledOnce();
    expect(screen.getByText("Bravo ! 4 × 3 = 12")).toBeInTheDocument();
  });

  it("lets the child cut the grid after 10 rows", async () => {
    const onComplete = vi.fn();
    const parts = [
      { rows: 10, cols: 4, product: 40 },
      { rows: 6, cols: 4, product: 24 },
    ];
    render(<ArrayGrid rows={16} cols={4} total={64} split={10} parts={parts} repeated={[]} rotate={false} removed={0} remaining={64} task="Split" onComplete={onComplete} />);
    const user = userEvent.setup();

    await user.click(screen.getByRole("button", { name: "Couper après 9 rangées" }));
    expect(screen.getByText("Presque ! Compte 10 rangées depuis le haut.")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Couper après 10 rangées" }));
    expect(onComplete).toHaveBeenCalledOnce();
    expect(screen.getByText("Bravo ! 40 + 24 = 64")).toBeInTheDocument();
  });
});

describe("PlaceValueTable", () => {
  it("lets the child write the number digit by digit", async () => {
    const onComplete = vi.fn();
    render(
      <PlaceValueTable
        value={512}
        ranks={["Hundreds", "Tens", "Units"]}
        digits={[5, 1, 2]}
        focus={null}
        countOf={null}
        decomposition={[]}
        words={null}
        interactive
        onComplete={onComplete}
      />,
    );
    const user = userEvent.setup();

    expect(screen.getByText("Quel chiffre dans la colonne des centaines ?")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Chiffre 5" }));
    await user.click(screen.getByRole("button", { name: "Chiffre 2" }));
    expect(screen.getByText("Presque ! Réécoute le nombre, puis regarde la colonne.")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Chiffre 1" }));
    await user.click(screen.getByRole("button", { name: "Chiffre 2" }));
    expect(onComplete).toHaveBeenCalledOnce();
    expect(screen.getByText("Bravo ! Tu as écrit 512.")).toBeInTheDocument();
  });

  it("shows both spellings of a number in words", async () => {
    render(
      <PlaceValueTable
        value={221}
        ranks={["Hundreds", "Tens", "Units"]}
        digits={[2, 2, 1]}
        focus={null}
        countOf={null}
        decomposition={[]}
        words={{ traditional: "deux cent vingt et un", rectified: "deux-cent-vingt-et-un" }}
        interactive={false}
      />,
    );

    expect(await screen.findByText("deux-cent-vingt-et-un", { exact: false }, { timeout: 4000 })).toBeInTheDocument();
    expect(screen.getByText("deux cent vingt et un", { exact: false })).toBeInTheDocument();
  });
});

describe("CalcChain (pick)", () => {
  const chain = (start: number, deltas: number[], crossed = false): Chain => {
    let at = start;
    const steps = deltas.map((delta) => {
      at += delta;
      return { delta, value: at };
    });
    const total = deltas.reduce((a, b) => a + b, 0);
    return { start, steps, total, result: start + total, crossed };
  };

  it("lets the child pick the right way", async () => {
    const onComplete = vi.fn();
    render(<CalcChain mode="Pick" chains={[chain(42, [10, 1], true), chain(42, [10, -1])]} onComplete={onComplete} />);
    const user = userEvent.setup();

    await user.click(screen.getByRole("button", { name: "Partir de 42, puis + 10, + 1" }));
    expect(screen.getByText("Presque ! Regarde bien où mène chaque chemin.")).toBeInTheDocument();
    expect(onComplete).not.toHaveBeenCalled();
    await user.click(screen.getByRole("button", { name: "Partir de 42, puis + 10, − 1" }));
    expect(onComplete).toHaveBeenCalledOnce();
    expect(screen.getByText("51")).toBeInTheDocument();
  });
});
