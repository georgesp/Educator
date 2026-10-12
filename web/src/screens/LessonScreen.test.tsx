import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { setVoiceEnabled } from "../audio/useSpeech";
import { fakeApi, renderApp, shortLesson } from "../test/renderWithApp";

const next = () => screen.getByRole("button", { name: /Suivant/ });

describe("LessonsScreen", () => {
  it("lists the lessons of the grade and opens one", async () => {
    const api = fakeApi();
    const router = renderApp(api, "/lecons");
    const user = userEvent.setup();

    expect(await screen.findByRole("heading", { name: "Leçons de maths" })).toBeInTheDocument();
    await user.click(await screen.findByRole("button", { name: /Ajouter 9, 19 ou 29/ }));
    expect(api.listLessons).toHaveBeenCalledWith("CP");
    expect(screen.queryByText(/En attendant, voici celles du CE1/)).not.toBeInTheDocument();
    await waitFor(() => expect(router.state.location.pathname).toBe("/lecons/ce1-add-9-19-29"));
  });

  it("says when the lessons come from CE1, or when there is none yet", async () => {
    const lessons = { grade: "CM1" as const, lessonsGrade: "CE1" as const, currentPeriod: 2, lessons: [] };
    renderApp(fakeApi({ listLessons: () => Promise.resolve(lessons) }), "/lecons");

    expect(await screen.findByText(/En attendant, voici celles du CE1/)).toBeInTheDocument();
    expect(screen.getByText("Les leçons arrivent bientôt.")).toBeInTheDocument();
  });
});

describe("LessonScreen", () => {
  it("shows one slide at a time with Previous and Next", async () => {
    renderApp(fakeApi(), "/lecons/ce1-add-9-19-29");
    const user = userEvent.setup();

    expect(await screen.findByRole("heading", { name: "Leçon" })).toBeInTheDocument();
    expect(screen.getByText("L'astuce de la dizaine ronde")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Précédent/ })).toBeDisabled();

    await user.click(next());
    expect(await screen.findByText("57 + 19 en deux bonds.")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: /Précédent/ }));
    expect(await screen.findByText("Une astuce de pilote.")).toBeInTheDocument();
  });

  it("waits for the child on the number line and guides after a mistake", async () => {
    renderApp(fakeApi(), "/lecons/ce1-add-9-19-29");
    const user = userEvent.setup();
    await user.click(await screen.findByRole("button", { name: /Suivant/ }));

    await screen.findByText("57 + 19 en deux bonds.");
    expect(next()).toBeDisabled();
    expect(screen.getByText("À toi de jouer pour continuer !")).toBeInTheDocument();
    expect(screen.getByRole("status")).toHaveTextContent("Où arrive la voiture après + 20 ?");

    // A wrong graduation: a gentle hint, never a reproach.
    await user.click(screen.getByRole("button", { name: "Graduation 78" }));
    expect(screen.getByRole("status")).toHaveTextContent("Presque ! Pars de 57 et avance de 20.");

    await user.click(screen.getByRole("button", { name: "Graduation 77" }));
    expect(screen.getByRole("status")).toHaveTextContent("Où arrive la voiture après − 1 ?");
    await user.click(screen.getByRole("button", { name: "Graduation 76" }));
    expect(screen.getByRole("status")).toHaveTextContent("Bravo ! 57 + 19 = 76");
    expect(next()).toBeEnabled();
  });

  it("links each calculation to its chain", async () => {
    renderApp(fakeApi(), "/lecons/ce1-add-9-19-29");
    const user = userEvent.setup();
    await user.click(await screen.findByRole("button", { name: /Suivant/ }));
    await user.click(await screen.findByRole("button", { name: "Graduation 77" }));
    await user.click(screen.getByRole("button", { name: "Graduation 76" }));
    await user.click(next());

    const calculations = await screen.findByRole("group", { name: "Les calculs" });
    const chains = screen.getByRole("group", { name: "Les chemins" });
    expect(next()).toBeDisabled();

    await user.click(within(calculations).getByRole("button", { name: "24 + 9" }));
    await user.click(within(chains).getByRole("button", { name: "+ 20 puis − 1" }));
    expect(screen.getByRole("status")).toHaveTextContent("Presque !");

    await user.click(within(chains).getByRole("button", { name: "+ 10 puis − 1" }));
    expect(within(calculations).getByRole("button", { name: "24 + 9 = 33" })).toBeDisabled();
    await user.click(within(calculations).getByRole("button", { name: "24 + 19" }));
    await user.click(within(chains).getByRole("button", { name: "+ 20 puis − 1" }));
    expect(screen.getByRole("status")).toHaveTextContent("Bravo, tout est relié !");
    expect(next()).toBeEnabled();
  });

  it("ends with the quiz checked by the core, then « Je m'entraîne »", async () => {
    setVoiceEnabled(true);
    const api = fakeApi();
    const router = renderApp(api, "/lecons/ce1-add-9-19-29");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: /Suivant/ }));
    await user.click(await screen.findByRole("button", { name: "Graduation 77" }));
    await user.click(screen.getByRole("button", { name: "Graduation 76" }));
    await user.click(next());
    const calculations = await screen.findByRole("group", { name: "Les calculs" });
    const chains = screen.getByRole("group", { name: "Les chemins" });
    await user.click(within(calculations).getByRole("button", { name: "24 + 9" }));
    await user.click(within(chains).getByRole("button", { name: "+ 10 puis − 1" }));
    await user.click(within(calculations).getByRole("button", { name: "24 + 19" }));
    await user.click(within(chains).getByRole("button", { name: "+ 20 puis − 1" }));
    await user.click(next());

    expect(await screen.findByRole("heading", { name: "Je retiens" })).toBeInTheDocument();
    expect(api.speak).toHaveBeenCalledWith("Je retiens.");
    await user.click(screen.getByRole("button", { name: "Au quiz !" }));

    // Typed question, wrong answer: the explanation is shown and read.
    expect(await screen.findByText("28 + 9 = ?")).toBeInTheDocument();
    await user.type(screen.getByLabelText("Ta réponse"), "39{Enter}");
    expect(await screen.findByText("Pas tout à fait. Regarde :")).toBeInTheDocument();
    expect(api.answerLessonQuiz).toHaveBeenCalledWith("ce1-add-9-19-29", 0, { kind: "number", value: 39 });
    expect(api.speak).toHaveBeenCalledWith("28 + 10 = 38, puis j'enlève 1 : 28 + 9 = 37.");
    await user.click(screen.getByRole("button", { name: "Suivant" }));

    // QCM, right answer.
    await user.click(await screen.findByRole("button", { name: "65" }));
    expect(await screen.findByText("46 + 20 = 66, puis j'enlève 1 : 46 + 19 = 65.")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Suivant" }));

    expect(await screen.findByRole("heading", { name: "Leçon terminée !" })).toBeInTheDocument();
    expect(screen.getByText("1 bonne réponse sur 2")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: /Je m'entraîne/ }));
    await waitFor(() => expect(router.state.location.pathname).toBe(`/exercices/${shortLesson.skillIds[0]}`));
  });

  it("an unknown lesson offers to go back", async () => {
    const api = fakeApi({ getLesson: () => Promise.reject(new Error("404")) });
    const router = renderApp(api, "/lecons/nothing");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: "Retour" }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/lecons"));
  });
});
