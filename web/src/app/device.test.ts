import { afterEach, describe, expect, it, vi } from "vitest";
import { answerInputMode, isIpad, isTouchScreen } from "./device";

function screenWith(coarse: boolean) {
  vi.stubGlobal(
    "matchMedia",
    vi.fn((query: string) => ({ matches: coarse && query === "(pointer: coarse)", media: query })),
  );
}

describe("touch screen (SUB023810)", () => {
  afterEach(() => vi.unstubAllGlobals());

  it("on the iPad the answer fields never open the system keyboard", () => {
    screenWith(true);

    expect(isTouchScreen()).toBe(true);
    expect(answerInputMode("numeric")).toBe("none");
    expect(answerInputMode("text")).toBe("none");
  });

  it("on the computer the fields keep their keyboard", () => {
    screenWith(false);

    expect(isTouchScreen()).toBe(false);
    expect(answerInputMode("numeric")).toBe("numeric");
    expect(answerInputMode("text")).toBe("text");
  });
});

function deviceWith(userAgent: string, maxTouchPoints: number) {
  vi.stubGlobal("navigator", { userAgent, maxTouchPoints });
}

describe("iPad (SUB023814)", () => {
  afterEach(() => vi.unstubAllGlobals());

  it("is recognised, even when it introduces itself as a Mac", () => {
    deviceWith("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko)", 5);
    expect(isIpad()).toBe(true);
    deviceWith("Mozilla/5.0 (iPad; CPU OS 18_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko)", 5);
    expect(isIpad()).toBe(true);
  });

  it("is not the Mac nor the Windows computer", () => {
    deviceWith("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko)", 0);
    expect(isIpad()).toBe(false);
    deviceWith("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Edg/130.0", 0);
    expect(isIpad()).toBe(false);
  });
});
