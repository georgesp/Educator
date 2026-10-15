import { act, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { fakeApi, renderApp } from "../test/renderWithApp";

describe("SkillsScreen", () => {
  it("shows the revisions of the grade before after the current period, labelled with their grade", async () => {
    const api = fakeApi({
      getSkills: vi.fn().mockResolvedValue({
        subject: "Math",
        grade: "CE1",
        skillsGrade: "CE1",
        currentPeriod: 1,
        skills: [
          { id: "ce1-add-tables", title: "Tables d'addition", instruction: "Calcule.", period: 1, isCurrentPeriod: true, revisionFrom: null, answerModes: ["Keyboard"] },
          { id: "cp-add-2-digits", title: "Additions à deux chiffres", instruction: "Calcule.", period: 1, isCurrentPeriod: false, revisionFrom: "CP", answerModes: ["Keyboard"] },
          { id: "ce1-sub-9", title: "Enlever 9", instruction: "Calcule.", period: 2, isCurrentPeriod: false, revisionFrom: null, answerModes: ["Keyboard"] },
        ],
      }),
    });
    renderApp(api, "/exercices/maths");

    const revisions = await screen.findByRole("region", { name: "Révisions" });
    expect(within(revisions).getByRole("button", { name: /Additions à deux chiffres/ })).toHaveTextContent("Révision du CP");

    const titles = screen.getAllByRole("heading", { level: 2 }).map((h) => h.textContent);
    expect(titles).toEqual(["C'est ce que tu travailles en ce moment", "Révisions", "Les autres exercices de ta classe"]);
    const others = screen.getByRole("region", { name: "Les autres exercices de ta classe" });
    expect(within(others).queryByText(/Additions à deux chiffres/)).not.toBeInTheDocument();
  });

  it("has no revision section when nothing is revised", async () => {
    renderApp(fakeApi(), "/exercices/maths");

    expect(await screen.findByRole("region", { name: "C'est ce que tu travailles en ce moment" })).toBeInTheDocument();
    expect(screen.queryByRole("region", { name: "Révisions" })).not.toBeInTheDocument();
  });
});

describe("SkillsScreen — CE1 complements", () => {
  it("goes straight to the series when the skill has a single way to answer, and offers « Mes tables »", async () => {
    const api = fakeApi({
      getSkills: vi.fn().mockResolvedValue({
        subject: "Math",
        grade: "CE1",
        skillsGrade: "CE1",
        currentPeriod: 2,
        skills: [
          { id: "ce1-num-compare", title: "Comparer et ranger", instruction: "Compare.", period: 2, isCurrentPeriod: true, revisionFrom: null, answerModes: ["Keyboard"], family: "Numeration", spellingFormats: [], listeningFormats: [], mastery: null },
          { id: "ce1-mul-meaning", title: "Le signe ×", instruction: "Calcule.", period: 2, isCurrentPeriod: true, revisionFrom: null, answerModes: ["Keyboard", "Choice"], family: "Multiplication", spellingFormats: [], listeningFormats: [], mastery: null },
        ],
      }),
    });
    const router = renderApp(api, "/exercices/maths");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: /Comparer et ranger/ }));
    expect(router.state.location.pathname).toBe("/serie/ce1-num-compare/Keyboard");

    await act(() => router.navigate("/exercices/maths"));
    await user.click(await screen.findByRole("button", { name: "Mes tables" }));
    expect(router.state.location.pathname).toBe("/tables");
  });

  it("has no « Mes tables » without multiplications", async () => {
    renderApp(fakeApi(), "/exercices/maths");

    await screen.findByRole("region", { name: "C'est ce que tu travailles en ce moment" });
    expect(screen.queryByRole("button", { name: "Mes tables" })).not.toBeInTheDocument();
  });
});

describe("SkillsScreen — families", () => {
  it("shows the band of its family on every card, and a button per family shown", async () => {
    const api = fakeApi({
      getSkills: vi.fn().mockResolvedValue({
        subject: "Math",
        grade: "CE1",
        skillsGrade: "CE1",
        currentPeriod: 1,
        skills: [
          { id: "ce1-add-tables", title: "Tables d'addition", instruction: "Calcule.", period: 1, isCurrentPeriod: true, revisionFrom: null, answerModes: ["Keyboard"], family: "Calculation", category: "AddSubtract", spellingFormats: [], listeningFormats: [], mastery: null },
          { id: "ce1-num-hundreds", title: "Centaines, dizaines et unités", instruction: "Écris.", period: 1, isCurrentPeriod: true, revisionFrom: null, answerModes: ["Keyboard"], family: "Numeration", category: "Numbers", spellingFormats: [], listeningFormats: [], mastery: null },
          { id: "ce1-sub-9", title: "Enlever 9", instruction: "Calcule.", period: 2, isCurrentPeriod: false, revisionFrom: null, answerModes: ["Keyboard"], family: "Calculation", category: "AddSubtract", spellingFormats: [], listeningFormats: [], mastery: null },
        ],
      }),
    });
    renderApp(api, "/exercices/maths");

    const filter = await screen.findByRole("group", { name: "Choisis une famille" });
    expect(within(filter).getAllByRole("button").map((b) => b.textContent)).toEqual(["Tout", "123Les nombres", "+ −Additions et soustractions"]);
    expect(within(filter).getByRole("button", { name: "Tout" })).toHaveAttribute("aria-pressed", "true");

    const current = screen.getByRole("button", { name: /Tables d'addition/ });
    expect(current).toHaveAccessibleName(/Additions et soustractions/);
    expect(current).toHaveClass("big-button-outline");
    expect(current).not.toHaveClass("big-button-primary");
    expect(current.querySelector(".big-button-family")).toHaveClass("family-2");
    expect(screen.getByRole("button", { name: /Enlever 9/ }).querySelector(".big-button-family")).toHaveTextContent("+ −");
  });

  it("keeps only the family clicked above the lists, until « Tout » or a second click", async () => {
    const api = fakeApi({
      getSkills: vi.fn().mockResolvedValue({
        subject: "Math",
        grade: "CE1",
        skillsGrade: "CE1",
        currentPeriod: 1,
        skills: [
          { id: "ce1-add-tables", title: "Tables d'addition", instruction: "Calcule.", period: 1, isCurrentPeriod: true, revisionFrom: null, answerModes: ["Keyboard"], family: "Calculation", category: "AddSubtract", spellingFormats: [], listeningFormats: [], mastery: null },
          { id: "ce1-num-hundreds", title: "Centaines, dizaines et unités", instruction: "Écris.", period: 1, isCurrentPeriod: true, revisionFrom: null, answerModes: ["Keyboard"], family: "Numeration", category: "Numbers", spellingFormats: [], listeningFormats: [], mastery: null },
          { id: "ce1-sub-9", title: "Enlever 9", instruction: "Calcule.", period: 2, isCurrentPeriod: false, revisionFrom: null, answerModes: ["Keyboard"], family: "Calculation", category: "AddSubtract", spellingFormats: [], listeningFormats: [], mastery: null },
        ],
      }),
    });
    const router = renderApp(api, "/exercices/maths");
    const user = userEvent.setup();
    const filter = await screen.findByRole("group", { name: "Choisis une famille" });
    const numbers = within(filter).getByRole("button", { name: /Les nombres/ });

    await user.click(numbers);
    expect(numbers).toHaveAttribute("aria-pressed", "true");
    expect(screen.getByRole("button", { name: /Centaines, dizaines et unités/ })).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: /Tables d'addition/ })).not.toBeInTheDocument();
    // No other exercise of that family: its section is not shown.
    expect(screen.queryByRole("region", { name: "Les autres exercices de ta classe" })).not.toBeInTheDocument();
    expect(router.state.location.search).toBe("?famille=Numbers");

    await user.click(numbers);
    expect(screen.getByRole("button", { name: /Tables d'addition/ })).toBeInTheDocument();

    await user.click(within(filter).getByRole("button", { name: /Additions et soustractions/ }));
    expect(screen.queryByRole("button", { name: /Centaines/ })).not.toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Enlever 9/ })).toBeInTheDocument();
    await user.click(within(filter).getByRole("button", { name: "Tout" }));
    expect(screen.getByRole("button", { name: /Centaines/ })).toBeInTheDocument();
    expect(router.state.location.search).toBe("");
  });

  it("starts with the family of the address, and ignores a family absent from the list", async () => {
    renderApp(fakeApi(), "/exercices/maths?famille=Numbers");

    expect(await screen.findByRole("button", { name: /Additions jusqu'à 10/ })).toBeInTheDocument();
  });
});
