import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { Accessory, Badge, SessionRewards } from "../api/client";
import { lockParentSpace, unlockParentSpace } from "../app/parentAccess";
import { defaultSettings, fakeApi, lea, renderApp, summaryOf } from "../test/renderWithApp";

const badge = (id: string, title: string, earned: boolean): Badge => ({
  id,
  title,
  description: `Condition de ${title}.`,
  icon: "<svg xmlns='http://www.w3.org/2000/svg'/>",
  earned,
  earnedAt: earned ? "2026-10-12T16:30:00.000Z" : null,
});

const accessory = (id: string, name: string, unlocked: boolean, level: number | null = 1): Accessory => ({
  id,
  name,
  slot: "Head",
  colors: [
    { id: "rouge", name: "Rouge", main: "#D7262B" },
    { id: "bleu", name: "Bleu", main: "#2F6FD0" },
  ],
  unlocked,
  unlock: { level, badgeId: level === null ? "sans-faute" : null, badgeTitle: level === null ? "10 séries sans faute" : null },
});

describe("menu (SUB001501, SUB001507)", () => {
  it("shows the level, the goal of the day and the way to the badges, the map and the avatar", async () => {
    const router = renderApp(fakeApi(), "/menu");

    expect(await screen.findByText("Niveau 2")).toBeInTheDocument();
    expect(screen.getByText("encore 130 points")).toBeInTheDocument();
    expect(screen.getByRole("progressbar", { name: "Vers le niveau 3" })).toHaveAttribute("aria-valuenow", "13");
    expect(screen.getByText("1 série sur 3")).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "Ma carte" })).toHaveAttribute("href", "/carte");
    expect(screen.getByRole("link", { name: "Badges 1/23" })).toHaveAttribute("href", "/badges");
    await userEvent.setup().click(screen.getByRole("link", { name: "Habiller mon avatar" }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/avatar"));
  });
});

describe("badges (SUB001503)", () => {
  it("shows the badges earned in color and those to win with their condition", async () => {
    renderApp(fakeApi({ getBadges: vi.fn().mockResolvedValue([badge("a", "Premier tour", true), badge("b", "As des additions", false)]) }), "/badges");

    const earned = await screen.findByRole("region", { name: "Mes badges (1)" });
    expect(within(earned).getByText("Premier tour")).toBeInTheDocument();
    expect(within(earned).getByText("Gagné le 12 oct. 2026")).toBeInTheDocument();
    const toWin = screen.getByRole("region", { name: "Les badges à gagner" });
    expect(within(toWin).getByText("Condition de As des additions.")).toBeInTheDocument();
  });
});

describe("avatar (SUB001505)", () => {
  it("saves each choice at once, offers its colors, and explains a locked accessory", async () => {
    const api = fakeApi({ getAccessories: vi.fn().mockResolvedValue([accessory("cap", "Casquette", true), accessory("crown", "Couronne", false, null)]) });
    renderApp(api, "/avatar");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: "Casquette" }));
    expect(api.updateAvatar).toHaveBeenLastCalledWith("p-lea", { characterId: "lion", accessories: [{ accessoryId: "cap", colorId: "rouge" }] });
    await user.click(screen.getByRole("button", { name: "Bleu" }));
    expect(api.updateAvatar).toHaveBeenLastCalledWith("p-lea", { characterId: "lion", accessories: [{ accessoryId: "cap", colorId: "bleu" }] });

    await user.click(screen.getByRole("button", { name: /Couronne/ }));
    expect(screen.getByText(/Couronne : Gagne le badge « 10 séries sans faute » pour le débloquer/)).toBeInTheDocument();
    expect(api.updateAvatar).toHaveBeenCalledTimes(2);

    await user.click(screen.getByRole("button", { name: "Rien" }));
    expect(api.updateAvatar).toHaveBeenLastCalledWith("p-lea", { characterId: "lion", accessories: [] });

    await user.click(screen.getByRole("tab", { name: "Personnage" }));
    await user.click(await screen.findByRole("button", { name: "Ours" }));
    expect(api.updateAvatar).toHaveBeenLastCalledWith("p-lea", { characterId: "ours", accessories: [] });
  });
});

describe("progress map (SUB001506)", () => {
  it("draws a step per skill, the current one highlighted, and opens its exercises", async () => {
    const router = renderApp(fakeApi(), "/carte/maths");
    const user = userEvent.setup();

    const steps = await screen.findByRole("list", { name: "Le parcours de Mathématiques, CP" });
    expect(within(steps).getAllByRole("button")).toHaveLength(2);
    await user.click(within(steps).getByRole("button", { name: /Tables d'addition, à venir/ }));
    await user.click(screen.getByRole("button", { name: "Les exercices" }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/exercices/maths/cp-add-tables"));
  });
});

describe("advice and reviews (SUB001508, SUB001509)", () => {
  it("advises a skill and offers « Mes révisions » when mistakes are due", async () => {
    const api = fakeApi({
      getAdvice: vi.fn().mockResolvedValue({ skillId: "cp-add-sum-10", title: "Additions jusqu'à 10", reason: "Start", prepares: null }),
      getProfileOverview: vi.fn().mockResolvedValue({
        level: { level: 2, points: 120, levelStart: 100, nextLevelAt: 250, percent: 13 },
        goal: { target: 3, done: 1, reached: false, streak: 0, bonusPoints: 30 },
        badgesEarned: 1,
        badgesTotal: 23,
        reviews: [
          { subject: "Math", due: 4 },
          { subject: "Spelling", due: 0 },
        ],
      }),
    });
    const router = renderApp(api, "/exercices/maths");
    const user = userEvent.setup();

    const advice = await screen.findByRole("region", { name: "Je te conseille…" });
    expect(within(advice).getByText("C'est le moment de découvrir :")).toBeInTheDocument();
    expect(within(advice).getByRole("button", { name: /Additions jusqu'à 10/ })).toBeInTheDocument();
    expect(screen.getByText("4 calculs ou mots à revoir aujourd'hui.")).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Je révise" }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/revisions/maths"));
    await waitFor(() => expect(api.startReview).toHaveBeenCalledWith("p-lea", "Math"));
    expect(await screen.findByText("Tu t'étais trompé ici : essaie encore !")).toBeInTheDocument();
  });

  it("shows the stars of each skill", async () => {
    renderApp(fakeApi(), "/exercices/maths");
    expect(await screen.findByRole("img", { name: "2 étoiles sur 3" })).toBeInTheDocument();
  });
});

describe("end of a series (lot 3)", () => {
  afterEach(() => vi.restoreAllMocks());

  it("shows the stars, the bonus of the goal, the new level, the accessories and the badges", async () => {
    const rewards: SessionRewards = {
      stars: 3,
      previousBestStars: 1,
      levelBefore: { level: 1, points: 80, levelStart: 0, nextLevelAt: 100, percent: 80 },
      level: { level: 2, points: 150, levelStart: 100, nextLevelAt: 250, percent: 33 },
      goal: { target: 3, done: 3, reached: true, streak: 2, bonusPoints: 30 },
      goalBonus: 30,
      newBadges: [badge("premier-tour", "Premier tour de piste", true)],
      newAccessories: [accessory("noeud", "Nœud", true, 2)],
    };
    const api = fakeApi();
    const base = api.answerSeries;
    api.answerSeries = vi.fn(async (...args: Parameters<typeof base>) => {
      const outcome = await base(...args);
      return outcome.summary ? { ...outcome, summary: { ...summaryOf(2, 2, 40, 190), rewards } } : outcome;
    });
    renderApp(api, "/serie/cp-add-sum-10/Keyboard");
    const user = userEvent.setup();

    const input = await screen.findByRole("textbox");
    await user.type(input, "7{Enter}");
    await user.type(await screen.findByRole("textbox"), "4{Enter}");

    expect(await screen.findByRole("img", { name: "3 étoiles sur 3" })).toBeInTheDocument();
    expect(screen.getByText("Plus d'étoiles que la dernière fois !")).toBeInTheDocument();
    expect(screen.getByText(/Objectif du jour atteint : \+ 30 points/)).toBeInTheDocument();
    expect(screen.getByText("Tu passes au niveau 2 !")).toBeInTheDocument();
    expect(screen.getByText("Nouveau pour ton avatar : Nœud !")).toBeInTheDocument();
    expect(screen.getByText("Premier tour de piste")).toBeInTheDocument();
  });
});

describe("parents' settings and backup (SUB001512)", () => {
  afterEach(() => lockParentSpace());

  it("change the goal of the day, the timed challenge and the reminder of a child", async () => {
    unlockParentSpace();
    const api = fakeApi();
    renderApp(api, "/parent/espace");
    const user = userEvent.setup();

    const settings = await screen.findByRole("region", { name: "Réglages de Léa" });
    await user.click(within(settings).getByRole("button", { name: "5 séries" }));
    expect(api.updateProfileSettings).toHaveBeenLastCalledWith("p-lea", { dailyGoal: 5 });
    await user.click(within(settings).getByRole("button", { name: "2 minutes" }));
    expect(api.updateProfileSettings).toHaveBeenLastCalledWith("p-lea", { timedSeconds: 120 });
    await user.click(within(settings).getByRole("button", { name: "20 min" }));
    expect(api.updateProfileSettings).toHaveBeenLastCalledWith("p-lea", { sessionMinutes: 20 });
    await user.click(within(settings).getByRole("checkbox", { name: "Proposer le défi chrono" }));
    expect(api.updateProfileSettings).toHaveBeenLastCalledWith("p-lea", { timedEnabled: false });
  });

  it("save every profile, and restore them after a confirmation", async () => {
    unlockParentSpace();
    const api = fakeApi();
    renderApp(api, "/parent/espace");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: "Enregistrer une sauvegarde" }));
    expect(await screen.findByText(/Sauvegarde enregistrée \(1 profil\)/)).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Restaurer une sauvegarde" }));
    expect(api.importBackup).not.toHaveBeenCalled();
    await user.click(screen.getByRole("button", { name: "Choisir le fichier" }));
    expect(api.importBackup).toHaveBeenCalled();
    expect(await screen.findByText("Sauvegarde restaurée : 1 profil.")).toBeInTheDocument();
  });
});

describe("timed challenge chosen by the parents (SUB001512)", () => {
  it("is hidden when turned off", async () => {
    renderApp(fakeApi({ getProfileSettings: vi.fn().mockResolvedValue({ ...defaultSettings(), timedEnabled: false }) }), "/examen");
    expect(await screen.findByRole("button", { name: "Bilan de maths" })).toBeInTheDocument();
    await waitFor(() => expect(screen.queryByRole("heading", { name: /Défi chrono/ })).not.toBeInTheDocument());
  });

  it("offers only the duration fixed by the parents", async () => {
    renderApp(fakeApi({ getProfileSettings: vi.fn().mockResolvedValue({ ...defaultSettings(), timedSeconds: 60 }) }), "/defi/maths");
    await waitFor(() => expect(screen.getAllByRole("button", { name: /minute/ })).toHaveLength(1));
    expect(screen.getByRole("button", { name: /1 minute/ })).toBeInTheDocument();
  });
});

// The profile of the tests wears nothing at first.
void lea;
