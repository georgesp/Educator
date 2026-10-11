import { screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { fakeApi, renderApp } from "../test/renderWithApp";

describe("SkillsScreen", () => {
  it("shows the revisions of the grade before after the current period, labelled with their grade", async () => {
    const api = fakeApi({
      getSkills: vi.fn().mockResolvedValue({
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
    renderApp(api, "/exercices");

    const revisions = await screen.findByRole("region", { name: "Révisions" });
    expect(within(revisions).getByRole("button", { name: /Additions à deux chiffres/ })).toHaveTextContent("Révision du CP");

    const titles = screen.getAllByRole("heading", { level: 2 }).map((h) => h.textContent);
    expect(titles).toEqual(["C'est ce que tu travailles en ce moment", "Révisions", "Les autres exercices de ta classe"]);
    const others = screen.getByRole("region", { name: "Les autres exercices de ta classe" });
    expect(within(others).queryByText(/Additions à deux chiffres/)).not.toBeInTheDocument();
  });

  it("has no revision section when nothing is revised", async () => {
    renderApp(fakeApi(), "/exercices");

    expect(await screen.findByRole("region", { name: "C'est ce que tu travailles en ce moment" })).toBeInTheDocument();
    expect(screen.queryByRole("region", { name: "Révisions" })).not.toBeInTheDocument();
  });
});
