import "@testing-library/jest-dom/vitest";
import { cleanup } from "@testing-library/react";
import { afterEach } from "vitest";
import { setVoiceEnabled } from "../audio/useSpeech";

afterEach(() => {
  cleanup();
  // The voice is off by default: the tests that read aloud turn it on themselves.
  setVoiceEnabled(false);
});
