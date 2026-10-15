/// <reference types="node" />
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import themesFile from "../../content/themes/themes.json";

/**
 * Themes (SUB001513): every theme of the content has its block in the styles, defines every variable of
 * the sport theme, and its texts reach the WCAG AA contrast (4.5:1) on their backgrounds.
 */
// The tests run from web/.
const styles = readFileSync(`${process.cwd()}/src/styles.css`, "utf8");
const themes: { id: string }[] = themesFile.themes;

/** The variables of a `[data-theme="id"]` block. */
function variablesOf(id: string): Map<string, string> {
  const start = styles.search(new RegExp(`\\[data-theme="${id}"\\]\\s*\\{`));
  expect(start, `no [data-theme="${id}"] block`).toBeGreaterThanOrEqual(0);
  const body = styles.slice(styles.indexOf("{", start) + 1, styles.indexOf("\n}", start));
  const variables = new Map<string, string>();
  for (const match of body.matchAll(/(--[\w-]+):\s*([^;]+);/g)) {
    variables.set(match[1], match[2].trim());
  }
  return variables;
}

function luminance(hex: string): number {
  const channels = [1, 3, 5].map((i) => parseInt(hex.slice(i, i + 2), 16) / 255);
  const [r, g, b] = channels.map((c) => (c <= 0.03928 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4));
  return 0.2126 * r + 0.7152 * g + 0.0722 * b;
}

export function contrast(a: string, b: string): number {
  const [light, dark] = [luminance(a), luminance(b)].sort((x, y) => y - x);
  return (light + 0.05) / (dark + 0.05);
}

/** Texts and the backgrounds they are written on. */
const PAIRS: [text: string, background: string][] = [
  ["--text", "--bg"],
  ["--text", "--surface"],
  ["--text", "--surface-strong"],
  ["--muted", "--bg"],
  ["--muted", "--surface"],
  ["--primary-text", "--primary"],
  ["--primary-text", "--primary-strong"],
  ["--primary-strong", "--bg"],
  ["--correct", "--bg"],
  ["--incorrect", "--bg"],
  ["--family-1-text", "--family-1"],
  ["--family-2-text", "--family-2"],
  ["--family-3-text", "--family-3"],
  ["--family-4-text", "--family-4"],
  ["--family-5-text", "--family-5"],
];

describe("themes (SUB001513)", () => {
  const sport = variablesOf("sport");

  it("the sport theme is the default and every theme of the content has its block", () => {
    expect(themes.map((t) => t.id)).toContain("sport");
    for (const theme of themes) {
      expect(variablesOf(theme.id).size).toBeGreaterThan(0);
    }
  });

  it.each(themes.map((t) => t.id))("theme %s defines every variable of the sport theme", (id) => {
    const variables = variablesOf(id);
    const missing = [...sport.keys()].filter((name) => !variables.has(name));
    expect(missing).toEqual([]);
  });

  it.each(themes.map((t) => t.id))("theme %s: texts and backgrounds reach the AA contrast", (id) => {
    const variables = variablesOf(id);
    const failures = PAIRS.map(([text, background]) => ({ text, background, ratio: contrast(variables.get(text)!, variables.get(background)!) }))
      .filter((p) => !(p.ratio >= 4.5))
      .map((p) => `${p.text} on ${p.background}: ${p.ratio.toFixed(2)}`);
    expect(failures).toEqual([]);
  });

  it("measures the contrast like WCAG", () => {
    expect(contrast("#000000", "#ffffff")).toBeCloseTo(21, 0);
    expect(contrast("#777777", "#ffffff")).toBeCloseTo(4.48, 1);
  });
});
