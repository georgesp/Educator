import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import type { SpeechStatusResponse } from "../api/types";
import { BackendProvider } from "../app/BackendContext";
import { fakeApi } from "../test/renderWithApp";
import { StatusBar } from "./StatusBar";

function renderWithSpeech(speech: SpeechStatusResponse) {
  const api = fakeApi({
    getStatus: vi.fn().mockResolvedValue({ version: "0.1.0", database: { isReady: true }, speech }),
  });
  render(
    <QueryClientProvider client={new QueryClient()}>
      <BackendProvider api={api}>
        <StatusBar />
      </BackendProvider>
    </QueryClientProvider>,
  );
}

describe("StatusBar", () => {
  it("names the voice when it is ready", async () => {
    renderWithSpeech({ isAvailable: true, voice: "Thomas", problem: null, voiceToInstall: false });

    expect(await screen.findByText("Voix : Thomas")).toBeInTheDocument();
  });

  it("tells the parent where to install a French voice on the iPad", async () => {
    renderWithSpeech({ isAvailable: false, voice: null, problem: "No French voice", voiceToInstall: true });

    expect(await screen.findByText(/installe une voix française/)).toBeInTheDocument();
  });

  it("says the voice is unavailable otherwise", async () => {
    renderWithSpeech({ isAvailable: false, voice: null, problem: "No Piper", voiceToInstall: false });

    expect(await screen.findByText("Voix indisponible")).toBeInTheDocument();
  });
});
