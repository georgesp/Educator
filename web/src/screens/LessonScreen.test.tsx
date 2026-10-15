import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { setVoiceEnabled } from "../audio/useSpeech";
import { fr } from "../i18n/fr";
import { fakeApi, quizSeries, renderApp, shortLesson, summaryOf } from "../test/renderWithApp";

const next = () => screen.getByRole("button", { name: /Suivant/ });

/** The slides of the short lesson, done by the child, up to « Je retiens ». */
async function goThroughTheSlides(user: ReturnType<typeof userEvent.setup>) {
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
}

describe("LessonsScreen", () => {
  it("lists the lessons of the grade and opens one", async () => {
    const api = fakeApi();
    const router = renderApp(api, "/lecons");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: /Mathématiques/ }));
    expect(await screen.findByRole("heading", { name: "Leçons de maths" })).toBeInTheDocument();
    const lesson = await screen.findByRole("button", { name: /Ajouter 9, 19 ou 29/ });
    expect(lesson.querySelector(".big-button-family")).toHaveClass("family-2");
    // A single family: no filter to show.
    expect(screen.queryByRole("group", { name: "Choisis une famille" })).not.toBeInTheDocument();
    await user.click(lesson);
    expect(api.listLessons).toHaveBeenCalledWith("CP", "Math", "p-lea");
    expect(screen.queryByText(/En attendant, voici celles du CE1/)).not.toBeInTheDocument();
    await waitFor(() => expect(router.state.location.pathname).toBe("/lecons/maths/ce1-add-9-19-29"));
  });

  it("says when the lessons come from CE1, or when there is none yet", async () => {
    const lessons = { subject: "Math" as const, grade: "CM1" as const, lessonsGrade: "CE1" as const, currentPeriod: 2, lessons: [] };
    renderApp(fakeApi({ listLessons: () => Promise.resolve(lessons) }), "/lecons/maths");

    expect(await screen.findByText(/En attendant, voici celles du CE1/)).toBeInTheDocument();
    expect(screen.getByText("Les leçons arrivent bientôt.")).toBeInTheDocument();
  });
});

describe("LessonScreen", () => {
  it("shows one slide at a time with Previous and Next", async () => {
    renderApp(fakeApi(), "/lecons/maths/ce1-add-9-19-29");
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
    renderApp(fakeApi(), "/lecons/maths/ce1-add-9-19-29");
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
    renderApp(fakeApi(), "/lecons/maths/ce1-add-9-19-29");
    const user = userEvent.setup();
    await user.click(await screen.findByRole("button", { name: /Suivant/ }));
    await user.click(await screen.findByRole("button", { name: "Graduation 77" }));
    await user.click(screen.getByRole("button", { name: "Graduation 76" }));
    await user.click(next());

    const calculations = await screen.findByRole("group", { name: "Les calculs" });
    const chains = screen.getByRole("group", { name: "Les chemins" });
    expect(next()).toBeDisabled();

    await user.click(within(calculations).getByRole("button", { name: "24 + 9" }));
    expect(screen.getByRole("status")).toHaveTextContent("Quel chemin fait + 9 en tout ?");
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
    const router = renderApp(api, "/lecons/maths/ce1-add-9-19-29");
    const user = userEvent.setup();

    await goThroughTheSlides(user);

    expect(await screen.findByRole("heading", { name: "Je retiens" })).toBeInTheDocument();
    // Nothing is read by itself: the slide is read on click.
    expect(api.speak).not.toHaveBeenCalledWith("Je retiens.");
    await user.click(screen.getByRole("button", { name: /^Écouter$/ }));
    expect(api.speak).toHaveBeenCalledWith("Je retiens.");
    await user.click(screen.getByRole("button", { name: "Au quiz !" }));

    // Typed question, wrong answer: the explanation is shown, and read on click.
    expect(await screen.findByText("28 + 9 = ?")).toBeInTheDocument();
    await user.type(screen.getByLabelText("Ta réponse"), "39{Enter}");
    expect(await screen.findByText("Pas tout à fait. Regarde :")).toBeInTheDocument();
    expect(api.startLessonQuiz).toHaveBeenCalledWith("p-lea", "ce1-add-9-19-29");
    expect(api.answerSeries).toHaveBeenCalledWith("quiz", 0, { kind: "number", value: 39 }, expect.any(Number));
    expect(api.speak).not.toHaveBeenCalledWith("28 + 10 = 38, puis j'enlève 1 : 28 + 9 = 37.");
    const feedback = screen.getByText("Pas tout à fait. Regarde :").closest(".answer-feedback") as HTMLElement;
    await user.click(within(feedback).getByRole("button", { name: "Écouter" }));
    expect(api.speak).toHaveBeenCalledWith("28 + 10 = 38, puis j'enlève 1 : 28 + 9 = 37.");
    await user.click(screen.getByRole("button", { name: "Suivant" }));

    // QCM, right answer.
    await user.click(await screen.findByRole("button", { name: "65" }));
    expect(await screen.findByText("46 + 20 = 66, puis j'enlève 1 : 46 + 19 = 65.")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Suivant" }));

    expect(await screen.findByRole("heading", { name: "Leçon terminée !" })).toBeInTheDocument();
    expect(screen.getByText("1 bonne réponse sur 2")).toBeInTheDocument();
    expect(screen.getByText("+30 points")).toBeInTheDocument();
    // Progress kept on every slide, forgotten once the lesson is finished.
    expect(api.saveLessonProgress).toHaveBeenCalledWith("p-lea", "ce1-add-9-19-29", 3);
    expect(api.saveLessonProgress).toHaveBeenLastCalledWith("p-lea", "ce1-add-9-19-29", null);
    await user.click(screen.getByRole("button", { name: /Je m'entraîne/ }));
    await waitFor(() => expect(router.state.location.pathname).toBe(`/exercices/maths/${shortLesson.skillIds[0]}`));
    // The quiz ended with its last answer: nothing to abandon.
    expect(api.abandonSeries).not.toHaveBeenCalled();
  });

  it("a quiz left before its last answer is recorded as abandoned", async () => {
    const api = fakeApi();
    renderApp(api, "/lecons/maths/ce1-add-9-19-29");
    const user = userEvent.setup();

    await goThroughTheSlides(user);
    await user.click(await screen.findByRole("button", { name: "Au quiz !" }));
    await user.type(await screen.findByLabelText("Ta réponse"), "37{Enter}");
    await screen.findByRole("button", { name: "Suivant" });
    await user.click(screen.getByRole("button", { name: fr.lessons.quit }));

    await waitFor(() => expect(api.abandonSeries).toHaveBeenCalledWith("quiz"));
  });

  it("offers to practise each skill of a lesson that has several", async () => {
    const api = fakeApi({
      getLesson: () => Promise.resolve({ ...shortLesson, skillIds: ["cp-add-sum-10", "cp-add-tables"], quiz: shortLesson.quiz.slice(0, 1) }),
      startLessonQuiz: vi.fn().mockResolvedValue({ ...quizSeries, exercises: quizSeries.exercises.slice(0, 1) }),
      answerSeries: vi.fn().mockResolvedValue({
        correct: true,
        expected: { kind: "number", value: 37 },
        expectedLabel: "37",
        explanation: "Bravo.",
        help: null,
        correction: null,
        summary: summaryOf(1, 1, 50, 170, "ce1-add-9-19-29"),
      }),
    });
    const router = renderApp(api, "/lecons/maths/ce1-add-9-19-29");
    const user = userEvent.setup();

    // Through the slides (the interactive ones done), then the one-question quiz.
    await goThroughTheSlides(user);
    await user.click(await screen.findByRole("button", { name: "Au quiz !" }));
    await user.type(await screen.findByLabelText("Ta réponse"), "37{Enter}");
    await user.click(await screen.findByRole("button", { name: "Suivant" }));

    expect(await screen.findByRole("button", { name: /Je m'entraîne : Additions jusqu'à 10/ })).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: /Je m'entraîne : Tables d'addition/ }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/exercices/maths/cp-add-tables"));
  });

  it("an unknown lesson offers to go back", async () => {
    const api = fakeApi({ getLesson: () => Promise.reject(new Error("404")) });
    const router = renderApp(api, "/lecons/maths/nothing");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: "Retour" }));
    await waitFor(() => expect(router.state.location.pathname).toBe("/lecons/maths"));
  });
});

describe("LessonScreen — lot 2", () => {
  it("offers to resume a lesson left in the middle", async () => {
    const api = fakeApi({ getLesson: vi.fn().mockResolvedValue({ ...shortLesson, resumeSlide: 3 }) });
    renderApp(api, "/lecons/maths/ce1-add-9-19-29");
    const user = userEvent.setup();

    expect(await screen.findByRole("heading", { name: "Tu avais commencé cette leçon" })).toBeInTheDocument();
    expect(api.getLesson).toHaveBeenCalledWith("ce1-add-9-19-29", "p-lea");
    await user.click(screen.getByRole("button", { name: /Je reprends/ }));
    expect(await screen.findByRole("heading", { name: "Je retiens" })).toBeInTheDocument();
  });

  it("below half of right answers, « Revoir la leçon » comes first", async () => {
    const api = fakeApi({
      getLesson: vi.fn().mockResolvedValue({ ...shortLesson, resumeSlide: 3 }),
      answerSeries: vi.fn(async (_s: string, index: number) => ({
        correct: false,
        expected: { kind: "number" as const, value: 1 },
        expectedLabel: "1",
        explanation: "Explication.",
        help: null,
        correction: null,
        summary: index === 1 ? summaryOf(0, 2, 20, 140, "ce1-add-9-19-29") : null,
      })),
    });
    renderApp(api, "/lecons/maths/ce1-add-9-19-29");
    const user = userEvent.setup();

    await user.click(await screen.findByRole("button", { name: /Je reprends/ }));
    await user.click(await screen.findByRole("button", { name: "Au quiz !" }));
    await user.type(await screen.findByLabelText("Ta réponse"), "1{Enter}");
    await user.click(await screen.findByRole("button", { name: "Suivant" }));
    await user.click(await screen.findByRole("button", { name: "64" }));
    await user.click(await screen.findByRole("button", { name: "Suivant" }));

    expect(await screen.findByText(/Tu peux revoir la leçon/)).toBeInTheDocument();
    const actions = screen.getAllByRole("button", { name: /Revoir la leçon|Je m'entraîne/ });
    expect(actions[0]).toHaveAccessibleName(/Revoir la leçon/);
    expect(actions[0]).toHaveClass("big-button-primary");
  });
});
