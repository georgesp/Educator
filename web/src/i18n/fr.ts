/**
 * Every text shown to the child or the parent, in French. Components never hard-code text:
 * they import `fr` and pick the entry they need.
 */
export const fr = {
  app: {
    title: "Educator",
    starting: "Démarrage…",
    startFailed: "Oups ! L'application n'a pas pu démarrer.",
    retry: "Réessayer",
  },
  home: {
    greeting: "Bonjour !",
    prompt: "Prêt pour une addition ?",
    startAddition: "Faire une addition",
  },
  addition: {
    title: "Addition",
    loading: "Je prépare ton addition…",
    answerLabel: "Ta réponse",
    check: "Valider",
    listen: "Écouter",
    next: "Nouvelle addition",
    back: "Retour",
    correct: "Bravo ! C'est la bonne réponse.",
    incorrect: (expected: number) => `Presque ! La bonne réponse était ${expected}.`,
    error: "Quelque chose n'a pas marché. Réessaie !",
  },
  status: {
    version: (version: string) => `Version ${version}`,
    voiceReady: (voice: string) => `Voix : ${voice}`,
    voiceUnavailable: "Voix indisponible",
    databaseNotReady: "Base de données en préparation",
  },
} as const;
