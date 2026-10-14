import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { ApiError, type ProfileStatistics, type SkillStatistics } from "../api/client";
import { lockParentSpace } from "../app/parentAccess";
import { emptyStatistics, fakeApi, lea, renderApp } from "../test/renderWithApp";

const tom = { ...lea, id: "p-tom", name: "Tom", grade: "CE1" as const };

function skill(overrides: Partial<SkillStatistics>): SkillStatistics {
  return {
    skillId: "cp-add-sum-10",
    title: "Additions jusqu'à 10",
    subject: "Math",
    grade: "CP",
    completedCount: 0,
    abandonedCount: 0,
    answerCount: 0,
    correctCount: 0,
    mastery: 0,
    state: "TooEarly",
    pointsEarned: 0,
    averagePoints: null,
    bestPoints: null,
    trend: "Unknown",
    lastPracticedAt: "2026-10-13T17:00:00.000Z",
    history: [],
    ...overrides,
  };
}

const point = (date: string, pointsEarned: number, correctCount: number) => ({
  startedAt: `${date}T17:00:00.000Z`,
  date,
  sessionType: "Exercise" as const,
  questionCount: 10,
  correctCount,
  pointsEarned,
});

/** Léa: additions done 4 times and better and better, subtractions to review. */
function leaStatistics(): ProfileStatistics {
  const empty = emptyStatistics(lea.id);
  return {
    ...empty,
    summary: { sessionCount: 6, totalSeconds: 1500, pointsEarned: 390, answerCount: 55, correctCount: 38, successRate: 69, activeDays: 4, lastActivityAt: "2026-10-14T08:05:00.000Z" },
    days: empty.days.map((d) => (d.date === "2026-10-14" ? { ...d, seconds: 300, sessionCount: 1 } : d)),
    weeks: empty.weeks.map((w, i) => (i === 7 ? { ...w, seconds: 900, answerCount: 20, correctCount: 15, successRate: 75, pointsEarned: 140 } : w)),
    skills: [
      skill({
        skillId: "cp-sub-10",
        title: "Soustractions jusqu'à 10",
        completedCount: 1,
        abandonedCount: 1,
        answerCount: 15,
        correctCount: 3,
        mastery: 20,
        state: "ToReview",
        pointsEarned: 40,
        averagePoints: 40,
        bestPoints: 40,
        history: [point("2026-10-13", 40, 2)],
      }),
      skill({
        completedCount: 4,
        answerCount: 40,
        correctCount: 35,
        mastery: 90,
        state: "Mastered",
        pointsEarned: 320,
        averagePoints: 80,
        bestPoints: 130,
        trend: "Up",
        history: [point("2026-10-01", 50, 3), point("2026-10-02", 60, 4), point("2026-10-05", 80, 8), point("2026-10-06", 130, 10)],
      }),
    ],
    toReview: ["cp-sub-10"],
    tests: [
      {
        sessionType: "Timed",
        subject: "Math",
        startedAt: "2026-10-12T09:00:00.000Z",
        date: "2026-10-12",
        questionCount: 8,
        correctCount: 7,
        pointsEarned: 90,
        timeLimitSeconds: 120,
        abandoned: false,
      },
    ],
    frequentMistakes: [
      {
        skillId: "cp-sub-10",
        skillTitle: "Soustractions jusqu'à 10",
        prompt: "7 − ? = 3",
        expected: "4",
        mistakeCount: 3,
        attemptCount: 4,
        usualAnswer: "3",
        lastAt: "2026-10-13T17:00:00.000Z",
      },
    ],
  };
}

async function enter(user: ReturnType<typeof userEvent.setup>) {
  await user.type(await screen.findByRole("textbox", { name: "Résultat du calcul" }), "162{Enter}");
}

describe("Parents' space (SUB001510, SUB001511)", () => {
  beforeEach(() => lockParentSpace());

  it("opens with the multiplication only, and draws another one after a wrong answer", async () => {
    const api = fakeApi();
    const router = renderApp(api, "/parent");
    const user = userEvent.setup();

    expect(await screen.findByText("27 × 6 = ?")).toBeInTheDocument();
    await user.type(screen.getByRole("textbox", { name: "Résultat du calcul" }), "160{Enter}");
    expect(await screen.findByRole("status")).toHaveTextContent("Ce n'est pas le bon résultat");
    await waitFor(() => expect(api.getParentGate).toHaveBeenCalledTimes(2));
    expect(router.state.location.pathname).toBe("/parent");

    await enter(user);
    await waitFor(() => expect(router.state.location.pathname).toBe("/parent/espace"));
    expect(api.checkParentGate).toHaveBeenLastCalledWith("gate", 162);
  });

  it("is closed without the gate, and closed again when going back to the child's menu", async () => {
    const router = renderApp(fakeApi(), "/parent/espace");
    await waitFor(() => expect(router.state.location.pathname).toBe("/parent"));
    const user = userEvent.setup();

    await enter(user);
    await user.click(await screen.findByRole("button", { name: "← Retour au menu enfant" }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/menu"));
    await router.navigate("/parent/espace");
    await waitFor(() => expect(router.state.location.pathname).toBe("/parent"));
  });

  it("says so for a child without any activity", async () => {
    renderApp(fakeApi(), "/parent");
    await enter(userEvent.setup());

    expect(await screen.findByText(/Léa n'a encore fait aucune activité/)).toBeInTheDocument();
  });

  it("shows each exercise with how many times it was done, its points and their evolution", async () => {
    const api = fakeApi({
      listProfiles: vi.fn().mockResolvedValue([lea, tom]),
      getProfileStatistics: vi.fn(async (id: string) => (id === lea.id ? leaStatistics() : emptyStatistics(id))),
    });
    renderApp(api, "/parent");
    const user = userEvent.setup();
    await enter(user);

    // Overview.
    expect(await screen.findByText("25 min")).toBeInTheDocument();
    expect(screen.getByText("69 %")).toBeInTheDocument();
    expect(screen.getByText("4 sur 14 jours")).toBeInTheDocument();
    expect(screen.getByText("mer. 14 oct. : 5 min")).toHaveClass("visually-hidden");

    // To review first.
    const review = screen.getByRole("heading", { name: "À revoir en priorité" }).parentElement!;
    expect(review).toHaveTextContent("Soustractions jusqu'à 10 · 20 % · À revoir");

    // The table: one line per exercise.
    const table = screen.getAllByRole("table")[0];
    const additions = within(table).getByRole("row", { name: /Additions jusqu'à 10/ });
    expect(additions).toHaveTextContent("4 fois");
    expect(additions).toHaveTextContent("320");
    expect(additions).toHaveTextContent("80 en moyenne");
    expect(additions).toHaveTextContent("Maîtrisé");
    // The chart's button says what it does, its points and its trend.
    expect(within(additions).getByRole("button")).toHaveAccessibleName(
      "Voir le détail de « Additions jusqu'à 10 » · Points des 4 dernières séries : 50, 60, 80, 130 · En progrès",
    );
    const subtractions = within(table).getByRole("row", { name: /Soustractions jusqu'à 10/ });
    expect(subtractions).toHaveTextContent("1 fois");
    expect(subtractions).toHaveTextContent("+ 1 non terminée");

    // Sorted by the points.
    await user.selectOptions(screen.getByRole("combobox", { name: "Trier par" }), "Le plus de points");
    const rows = within(table).getAllByRole("row");
    expect(rows[1]).toHaveTextContent("Additions jusqu'à 10");

    // The detail of the points, series by series.
    await user.click(screen.getByRole("button", { name: /^Voir le détail de « Additions jusqu'à 10 »/ }));
    expect(screen.getByRole("img", { name: /Points par série · Additions jusqu'à 10 : .*6 oct\. : 130 points, 10 sur 10/ })).toBeInTheDocument();
    expect(screen.getByText("Meilleure série : 130 points")).toBeInTheDocument();

    // Exams and challenges, frequent mistakes.
    expect(screen.getByText("Défi chrono de maths (2 min)")).toBeInTheDocument();
    expect(screen.getByText("7 sur 8")).toBeInTheDocument();
    const mistake = screen.getByRole("row", { name: /7 − \? = 3/ });
    expect(mistake).toHaveTextContent("4");
    expect(mistake).toHaveTextContent("3 sur 4");

    // Another child, then back: the detail and the order start again.
    await user.click(screen.getByRole("tab", { name: /Tom · CE1/ }));
    expect(await screen.findByText(/Tom n'a encore fait aucune activité/)).toBeInTheDocument();
    expect(api.getProfileStatistics).toHaveBeenCalledWith("p-tom");
    await user.click(screen.getByRole("tab", { name: /Léa · CP/ }));
    expect(await screen.findByRole("button", { name: /^Voir le détail de « Additions jusqu'à 10 »/ })).toHaveAttribute("aria-expanded", "false");
    expect(screen.getByRole("combobox", { name: "Trier par" })).toHaveValue("recent");
  });

  it("deletes a profile in two steps and with the typed name: never an accidental click", async () => {
    const api = fakeApi({ listProfiles: vi.fn().mockResolvedValue([lea, tom]) });
    renderApp(api, "/parent");
    const user = userEvent.setup();
    await enter(user);

    await user.click(await screen.findByRole("button", { name: "Supprimer le profil de Tom" }));
    expect(screen.getByRole("alertdialog")).toHaveTextContent("Supprimer le profil de Tom ?");
    // Step 1: keeping is the default.
    expect(screen.getByRole("button", { name: "Non, je garde le profil" })).toHaveFocus();
    await user.click(screen.getByRole("button", { name: "Continuer" }));

    // Step 2: the button stays disabled until the name is typed.
    const remove = screen.getByRole("button", { name: "Supprimer pour toujours" });
    expect(remove).toBeDisabled();
    const name = within(screen.getByRole("alertdialog")).getByRole("textbox");
    await user.type(name, "Léa");
    expect(remove).toBeDisabled();
    await user.clear(name);
    await user.type(name, "tom");
    expect(remove).toBeEnabled();
    await user.click(remove);

    await waitFor(() => expect(api.deleteProfile).toHaveBeenCalledWith("p-tom"));
    expect(api.deleteProfile).toHaveBeenCalledTimes(1);
    await waitFor(() => expect(screen.queryByRole("alertdialog")).not.toBeInTheDocument());
  });

  it("deleting the current profile leaves the child's menu for the profile choice", async () => {
    const api = fakeApi();
    const router = renderApp(api, "/parent");
    const user = userEvent.setup();
    await enter(user);

    await user.click(await screen.findByRole("button", { name: "Supprimer le profil de Léa" }));
    await user.click(screen.getByRole("button", { name: "Continuer" }));
    await user.type(within(screen.getByRole("alertdialog")).getByRole("textbox"), "Léa{Enter}");
    await waitFor(() => expect(api.deleteProfile).toHaveBeenCalledWith("p-lea"));

    (api.listProfiles as ReturnType<typeof vi.fn>).mockResolvedValue([]);
    await user.click(screen.getByRole("button", { name: "← Retour au menu enfant" }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/"));
  });
  it("renames the parents' card and draws it another character", async () => {
    const api = fakeApi();
    renderApp(api, "/parent");
    const user = userEvent.setup();
    await enter(user);

    const name = await screen.findByRole("textbox", { name: "Nom de la carte" });
    expect(name).toHaveValue("Parents");
    const save = screen.getByRole("button", { name: "Enregistrer le nom" });
    expect(save).toBeDisabled();
    await user.clear(name);
    await user.type(name, "Papa{Enter}");
    await waitFor(() => expect(api.updateParentProfile).toHaveBeenCalledWith({ name: "Papa", characterId: undefined }));
    expect(await screen.findByText("Enregistré.")).toBeInTheDocument();
    expect(name).toHaveValue("Papa");

    await user.click(screen.getByRole("button", { name: "Changer de personnage" }));
    await waitFor(() => expect(api.updateParentProfile).toHaveBeenLastCalledWith({ name: undefined, characterId: "lion" }));
    expect(api.drawCharacter).toHaveBeenCalledWith("ours");
  });

  it("explains a refused name of the parents' card", async () => {
    const api = fakeApi({
      updateParentProfile: vi.fn().mockRejectedValue(new ApiError(400, "Invalid profile", "Écris un nom pour la carte des parents.", "name")),
    });
    renderApp(api, "/parent");
    const user = userEvent.setup();
    await enter(user);

    const name = await screen.findByRole("textbox", { name: "Nom de la carte" });
    await user.clear(name);
    await user.type(name, " {Enter}");

    expect(await screen.findByRole("alert")).toHaveTextContent("Écris un nom pour la carte des parents.");
    expect(name).toHaveAttribute("aria-invalid", "true");
  });
});
