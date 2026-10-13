import { screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { setMascotHidden } from "../../app/settings";
import { setVoiceEnabled } from "../../audio/useSpeech";
import { fakeApi, renderApp } from "../../test/renderWithApp";

describe("Mascot (SUB001417)", () => {
  afterEach(() => setMascotHidden(false));

  it("presents the lesson at its start and reads her phrase", async () => {
    setVoiceEnabled(true);
    const api = fakeApi();
    renderApp(api, "/lecons/maths/ce1-add-9-19-29");

    expect(await screen.findByText("Aujourd'hui, on apprend : Ajouter 9, 19 ou 29.")).toBeInTheDocument();
    expect(screen.getByRole("img", { name: "Plume" })).toBeInTheDocument();
    expect(api.getMascotPhrase).toHaveBeenCalledWith("LessonStart", "Ajouter 9, 19 ou 29");
    await screen.findByTestId("mascot");
    await new Promise((r) => setTimeout(r, 50));
    expect(api.speak).toHaveBeenCalledWith("Aujourd'hui, on apprend : Ajouter 9, 19 ou 29.");
  });

  it("can be hidden from the Options", async () => {
    setMascotHidden(true);
    const api = fakeApi();
    renderApp(api, "/lecons/maths/ce1-add-9-19-29");

    expect(await screen.findByText("Une astuce de pilote.")).toBeInTheDocument();
    expect(screen.queryByTestId("mascot")).not.toBeInTheDocument();
    expect(api.getMascotPhrase).not.toHaveBeenCalled();
  });
});
