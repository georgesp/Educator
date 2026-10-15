import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { act, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { setMuted } from "../audio/sounds";
import { BackendProvider } from "../app/BackendContext";
import { ProfileProvider } from "../app/ProfileContext";
import { fakeApi } from "../test/renderWithApp";
import { SoundToggle } from "./SoundToggle";

describe("SoundToggle", () => {
  afterEach(() => setMuted(false));

  it("follows the setting changed elsewhere (Options)", () => {
    setMuted(false);
    render(
      <QueryClientProvider client={new QueryClient()}>
        <BackendProvider api={fakeApi()}>
          <ProfileProvider>
            <SoundToggle />
          </ProfileProvider>
        </BackendProvider>
      </QueryClientProvider>,
    );
    expect(screen.getByRole("button", { name: "Couper les sons" })).toHaveAttribute("aria-pressed", "false");

    act(() => setMuted(true));

    expect(screen.getByRole("button", { name: "Remettre les sons" })).toHaveAttribute("aria-pressed", "true");
  });
});
