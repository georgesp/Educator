import { render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { isInside, LetterTiles } from "./LetterTiles";
import { SoundSort } from "./SoundSort";

describe("letterTiles", () => {
  const bateau = { word: "bateau", tiles: ["b", "a", "t", "eau"], distractors: ["o", "au"], order: [3, 5, 0, 4, 2, 1] };

  it("the child clicks the tiles in order; an intruder shakes and stays", async () => {
    const onComplete = vi.fn();
    render(<LetterTiles {...bateau} interactive onComplete={onComplete} />);
    const user = userEvent.setup();
    const pool = screen.getByRole("group", { name: "Les morceaux du mot" });

    // Shown in the order computed by the core.
    expect(within(pool).getAllByRole("button").map((b) => b.textContent)).toEqual(["eau", "au", "b", "o", "t", "a"]);
    await user.click(within(pool).getByRole("button", { name: "Morceau b" }));
    await user.click(within(pool).getByRole("button", { name: "Morceau a" }));
    await user.click(within(pool).getByRole("button", { name: "Morceau t" }));
    await user.click(within(pool).getByRole("button", { name: "Morceau o" }));
    expect(screen.getByRole("status")).toHaveTextContent("Presque !");
    expect(onComplete).not.toHaveBeenCalled();

    await user.click(within(pool).getByRole("button", { name: "Morceau eau" }));
    expect(screen.getByRole("status")).toHaveTextContent("Bravo ! Tu as écrit « bateau ».");
    expect(onComplete).toHaveBeenCalledOnce();
    expect(screen.getByLabelText("Le mot : bateau")).toHaveTextContent("bateau");
  });

  it("a tile dropped on the word counts, elsewhere it goes back to its place", () => {
    const word = { left: 100, right: 300, top: 50, bottom: 110 } as DOMRect;
    const scroll = { x: 0, y: 200 };
    // The drag gives page coordinates: the scroll of the page is taken off.
    expect(isInside({ x: 150, y: 280 }, word, scroll)).toBe(true);
    expect(isInside({ x: 150, y: 80 }, word, scroll)).toBe(false);
    expect(isInside({ x: 350, y: 280 }, word, scroll)).toBe(false);
  });

  it("shown only, the tiles go to their place by themselves", async () => {
    render(<LetterTiles {...bateau} interactive={false} />);
    expect(screen.queryByRole("group", { name: "Les morceaux du mot" })).not.toBeInTheDocument();
    expect(await screen.findByLabelText("Le mot : bateau", {}, { timeout: 5000 })).toHaveTextContent("bateau");
  });
});

describe("soundSort", () => {
  const words = [
    { word: "vélo", group: 0, start: 3, length: 1 },
    { word: "jaune", group: 1, start: 1, length: 2 },
    { word: "gâteau", group: 2, start: 3, length: 3 },
    { word: "auto", group: 1, start: 0, length: 2 },
  ];

  it("the child clicks a word, then its box; the grapheme is colored in the box", async () => {
    const onComplete = vi.fn();
    render(<SoundSort groups={["o", "au", "eau"]} words={words} interactive onComplete={onComplete} />);
    const user = userEvent.setup();
    const pool = screen.getByRole("group", { name: "Les mots à ranger" });

    await user.click(within(pool).getByRole("button", { name: "jaune" }));
    expect(screen.getByRole("status")).toHaveTextContent("Dans quelle boîte va « jaune » ?");
    await user.click(screen.getByRole("button", { name: "La boîte o" }));
    expect(screen.getByRole("status")).toHaveTextContent("Presque ! Regarde bien comment s'écrit « jaune ».");
    await user.click(screen.getByRole("button", { name: "La boîte au" }));
    const box = screen.getByRole("button", { name: "La boîte au" });
    expect(within(box).getByText("au", { selector: ".sound-grapheme" })).toBeInTheDocument();

    for (const [word, group] of [
      ["vélo", "o"],
      ["gâteau", "eau"],
      ["auto", "au"],
    ]) {
      await user.click(within(pool).getByRole("button", { name: word }));
      await user.click(screen.getByRole("button", { name: `La boîte ${group}` }));
    }
    expect(screen.getByRole("status")).toHaveTextContent("Bravo, tous les mots sont rangés !");
    expect(onComplete).toHaveBeenCalledOnce();
  });

  it("shown only, every word goes to its box", async () => {
    render(<SoundSort groups={["o", "au", "eau"]} words={words} interactive={false} />);
    const box = screen.getByRole("group", { name: "La boîte eau" });
    expect(await within(box).findByText("eau", { selector: ".sound-grapheme" }, { timeout: 6000 })).toBeInTheDocument();
  });
});
