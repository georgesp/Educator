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
          { id: "ce1-num-compare", title: "Comparer et ranger", instruction: "Compare.", period: 2, isCurrentPeriod: true, revisionFrom: null, answerModes: ["Keyboard"], family: "Numeration", spellingFormats: [], mastery: null },
          { id: "ce1-mul-meaning", title: "Le signe ×", instruction: "Calcule.", period: 2, isCurrentPeriod: true, revisionFrom: null, answerModes: ["Keyboard", "Choice"], family: "Multiplication", spellingFormats: [], mastery: null },
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
