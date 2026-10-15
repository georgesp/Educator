import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "@fontsource/lexend/400.css";
import "@fontsource/lexend/600.css";
import "@fontsource/lexend/800.css";
import "@fontsource/barlow-condensed/700-italic.css";
import "@fontsource/barlow-condensed/800-italic.css";
// Cosmos theme (SUB001513) and the font adapted to dyslexia (SUB001512), embedded like the others.
import "@fontsource/fredoka/500.css";
import "@fontsource/fredoka/700.css";
import "@fontsource/opendyslexic/400.css";
import "@fontsource/opendyslexic/700.css";
import App from "./App";
import "./styles.css";

// Safari on the iPad applies `:active` only to pages that listen to touches (SUB023811): without
// this empty listener, a pressed button would give no feedback under the finger.
document.addEventListener("touchstart", () => {}, { passive: true });

createRoot(document.getElementById("root") as HTMLElement).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
