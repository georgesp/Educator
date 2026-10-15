import type { Page } from "@playwright/test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

/**
 * The application core simulated for the screens of lot 3 (motivation and parents' space). The avatars
 * and the badges are those of the content folder: the avatar is composed here like
 * `educator-core::rewards::accessories::compose_avatar` (anchors, colors through --acc).
 */
const CONTENT = join(process.cwd(), "..", "content");

type AccessoryEntry = {
  id: string;
  name: string;
  slot: string;
  layer?: string;
  file: string;
  anchor: { x: number; y: number };
  colors: string[];
  unlock: { level?: number; badge?: string };
};
type Color = { id: string; name: string; main: string; light: string };
type CharacterEntry = { id: string; name: string; kind: string; file: string; anchors: Record<string, { x: number; y: number }> };

const read = (path: string) => readFileSync(join(CONTENT, path), "utf8");
const accessoriesFile = JSON.parse(read("accessories/accessories.json")) as { colors: Color[]; accessories: AccessoryEntry[] };
const charactersFile = JSON.parse(read("characters/characters.json")) as { characters: CharacterEntry[] };
const badgesFile = JSON.parse(read("rewards/badges.json")) as { badges: { id: string; title: string; description: string; icon: string }[] };

const inner = (svg: string) => svg.slice(svg.indexOf(">", svg.indexOf("<svg")) + 1, svg.lastIndexOf("</svg>"));
const SLOT_ORDER: Record<string, number> = { neck: 0, eyes: 1, head: 2, companion: 3 };

export function avatarSvg(characterId: string, worn: { accessoryId: string; colorId: string }[]): string {
  const character = charactersFile.characters.find((c) => c.id === characterId) ?? charactersFile.characters[0];
  const layers = worn
    .map((w) => ({ a: accessoriesFile.accessories.find((x) => x.id === w.accessoryId), c: accessoriesFile.colors.find((x) => x.id === w.colorId) }))
    .filter((l): l is { a: AccessoryEntry; c: Color } => !!l.a && !!l.c)
    .sort((x, y) => SLOT_ORDER[x.a.slot] - SLOT_ORDER[y.a.slot]);
  const layer = ({ a, c }: { a: AccessoryEntry; c: Color }) => {
    const anchors = character.anchors;
    const target = a.slot === "head" ? anchors.headTop : a.slot === "eyes" ? anchors.eyes : a.slot === "neck" ? anchors.neck : { x: 222, y: 252 };
    return `<g transform="translate(${target.x - a.anchor.x} ${target.y - a.anchor.y})" style="--acc:${c.main};--acc-light:${c.light}">${inner(read(`accessories/${a.file}`))}</g>`;
  };
  return (
    `<svg xmlns="http://www.w3.org/2000/svg" viewBox="-12 -24 280 280" width="256" height="256">` +
    layers.filter((l) => l.a.layer === "back").map(layer).join("") +
    inner(read(`characters/${character.file}`)) +
    layers.filter((l) => l.a.layer !== "back").map(layer).join("") +
    "</svg>"
  );
}

export const lea = {
  id: "p1",
  name: "Léa",
  grade: "CP",
  characterId: "renard",
  accessories: [
    { accessoryId: "casquette", colorId: "bleu" },
    { accessoryId: "echarpe", colorId: "rouge" },
  ],
  points: 830,
  placementSeconds: 20,
  examSeconds: 0,
  lastUsedAt: "2026-10-15T08:00:00.000Z",
};

/** Level 5 (700 to 1000 points): what levels 1 to 5 unlock, and the badge « premier-tour ». */
const LEVEL = 5;
const EARNED = ["premier-tour", "complements-10", "sept-jours"];

const skill = (id: string, title: string, period: number, category: string, mastery: number | null, stars: number) => ({
  id,
  title,
  instruction: "Calcule.",
  period,
  isCurrentPeriod: period === 2,
  revisionFrom: null,
  answerModes: ["Keyboard", "Choice"],
  family: "Calculation",
  category,
  spellingFormats: [],
  listeningFormats: [],
  mastery,
  stars,
});

export const SKILLS = [
  skill("cp-add-sum-10", "Additions jusqu'à 10", 1, "AddSubtract", 95, 3),
  skill("cp-add-sub-1-2", "Ajouter ou enlever 1 ou 2", 1, "AddSubtract", 88, 2),
  skill("cp-sub-within-10", "Soustractions jusqu'à 10", 1, "AddSubtract", 72, 2),
  skill("cp-num-100", "Les nombres jusqu'à 100", 1, "Numbers", 64, 1),
  skill("cp-add-tables", "Tables d'addition", 2, "AddSubtract", 55, 1),
  skill("cp-compare-100", "Comparer les nombres jusqu'à 100", 2, "Numbers", null, 0),
  skill("cp-add-sub-tens", "Ajouter ou enlever des dizaines", 3, "AddSubtract", null, 0),
  skill("cp-sub-within-20", "Soustractions jusqu'à 20", 3, "AddSubtract", null, 0),
  skill("cp-add-9", "Ajouter 9", 4, "AddSubtract", null, 0),
  skill("cp-complement-next-ten", "Compléments à la dizaine", 4, "AddSubtract", null, 0),
  skill("cp-problems", "Petits problèmes", 4, "Problems", null, 0),
  skill("cp-add-2-digits", "Additions de deux nombres plus petits que 100", 5, "AddSubtract", null, 0),
  skill("cp-sub-from-tens", "Enlever un petit nombre à une dizaine", 5, "AddSubtract", null, 0),
];

export type Calls = { command: string; args: Record<string, unknown> }[];

export async function mockLot3(page: Page, calls: Calls = []) {
  let settings = {
    sound: true,
    mascot: true,
    voiceSpeed: "Normal",
    textSize: "Normal",
    dyslexiaFont: false,
    theme: "sport",
    dailyGoal: 3,
    timedEnabled: true,
    timedSeconds: 0,
    sessionMinutes: 0,
  };
  let profile = { ...lea };
  const commands = async (command: string, args: Record<string, unknown> = {}): Promise<unknown> => {
    calls.push({ command, args });
    switch (command) {
      case "get_startup":
        return { state: "ready" };
      case "get_status":
        return { version: "0.1.0", database: { isReady: true }, speech: { isAvailable: false, voice: null, problem: "e2e" } };
      case "list_profiles":
        return [profile];
      case "select_profile":
        return profile;
      case "get_parent_profile":
        return { name: "Parents", characterId: "ours" };
      case "get_characters":
        return charactersFile.characters.map((c) => ({ id: c.id, name: c.name, kind: c.kind === "animal" ? "Animal" : "Child" }));
      case "get_character_svg":
        return read(`characters/${charactersFile.characters.find((c) => c.id === args.id)?.file ?? "lion.svg"}`);
      case "get_avatar_svg":
        return avatarSvg(args.characterId as string, args.accessories as { accessoryId: string; colorId: string }[]);
      case "get_placement_status":
        return { subjects: [{ subject: "Math", available: false, done: true }, { subject: "Spelling", available: false, done: true }], questionSeconds: 20 };
      case "get_profile_overview":
        return {
          level: { level: LEVEL, points: profile.points, levelStart: 700, nextLevelAt: 1000, percent: 43 },
          goal: { target: settings.dailyGoal, done: 2, reached: false, streak: 4, bonusPoints: 30 },
          badgesEarned: EARNED.length,
          badgesTotal: badgesFile.badges.length,
          reviews: [
            { subject: "Math", due: 4 },
            { subject: "Spelling", due: 0 },
          ],
        };
      case "get_badges":
        return badgesFile.badges.map((b) => ({
          id: b.id,
          title: b.title,
          description: b.description,
          icon: read(`rewards/${b.icon}`),
          earned: EARNED.includes(b.id),
          earnedAt: EARNED.includes(b.id) ? "2026-10-12T16:30:00.000Z" : null,
        }));
      case "get_accessories":
        return accessoriesFile.accessories.map((a) => ({
          id: a.id,
          name: a.name,
          slot: a.slot[0].toUpperCase() + a.slot.slice(1),
          colors: a.colors.map((id) => accessoriesFile.colors.find((c) => c.id === id)).map((c) => ({ id: c!.id, name: c!.name, main: c!.main })),
          unlocked: a.unlock.level !== undefined ? a.unlock.level <= LEVEL : EARNED.includes(a.unlock.badge ?? ""),
          unlock: {
            level: a.unlock.level ?? null,
            badgeId: a.unlock.badge ?? null,
            badgeTitle: badgesFile.badges.find((b) => b.id === a.unlock.badge)?.title ?? null,
          },
        }));
      case "update_avatar": {
        const request = args.request as { characterId?: string; accessories: { accessoryId: string; colorId: string }[] };
        profile = { ...profile, characterId: request.characterId ?? profile.characterId, accessories: request.accessories };
        return profile;
      }
      case "get_skills":
        return { subject: args.subject ?? "Math", grade: "CP", skillsGrade: "CP", currentPeriod: 2, skills: SKILLS };
      case "list_lessons":
        return {
          subject: "Math",
          grade: "CP",
          lessonsGrade: "CP",
          currentPeriod: 2,
          lessons: [{ id: "cp-add-sum-10", title: "Additions jusqu'à 10", period: 1, isCurrentPeriod: false, durationMinutes: 4, category: "AddSubtract", stars: 3, skillId: "cp-add-sum-10" }],
        };
      case "get_advice":
        return { skillId: "cp-num-100", title: "Les nombres jusqu'à 100", reason: "Consolidate", prepares: "Comparer les nombres jusqu'à 100" };
      case "get_profile_settings":
        return settings;
      case "update_profile_settings":
        settings = { ...settings, ...(args.changes as object) };
        return settings;
      case "get_settings_options":
        return {
          themes: (JSON.parse(read("themes/themes.json")) as { themes: unknown[] }).themes,
          goalChoices: [0, 1, 2, 3, 4, 5, 6],
          timedChoices: [0, 60, 120, 180],
          sessionChoices: [0, 15, 20, 30, 45],
        };
      case "get_mascot":
        return null;
      case "get_mascot_phrase":
        return null;
      default:
        throw { status: 404, title: "Unknown command", detail: command, field: null };
    }
  };
  await page.exposeFunction("__educatorE2eInvoke", async (command: string, args: Record<string, unknown>) => {
    try {
      return { ok: true, value: await commands(command, args) };
    } catch (error) {
      return { ok: false, error };
    }
  });
  await page.addInitScript(() => {
    type Answer = { ok: boolean; value?: unknown; error?: unknown };
    const call = (window as unknown as { __educatorE2eInvoke: (c: string, a: unknown) => Promise<Answer> }).__educatorE2eInvoke;
    (window as unknown as { __TAURI_INTERNALS__: unknown }).__TAURI_INTERNALS__ = {
      invoke: async (command: string, args: unknown) => {
        const answer = await call(command, args ?? {});
        if (!answer.ok) {
          throw answer.error;
        }
        return answer.value;
      },
      transformCallback: () => 0,
      metadata: { currentWindow: { label: "main" }, currentWebview: { windowLabel: "main", label: "main" } },
    };
  });
}

/** Opens the application and chooses Léa. */
export async function openAsLea(page: Page) {
  await page.goto("/");
  await page.getByRole("button", { name: /Léa/ }).first().click();
  await page.waitForURL(/#\/menu/);
}
