import { afterEach, describe, expect, it, vi } from "vitest";
import { playVoice, setVoiceEnabled } from "./useSpeech";

describe("voice in the background", () => {
  afterEach(() => {
    setVoiceEnabled(false);
    Object.defineProperty(document, "visibilityState", { value: "visible", configurable: true });
    vi.unstubAllGlobals();
  });

  it("a reading still being prepared is dropped when the application goes to the background", async () => {
    const audio = vi.fn();
    vi.stubGlobal("Audio", audio);
    setVoiceEnabled(true);
    let deliver: (wav: Blob) => void = () => undefined;
    const reading = playVoice(() => new Promise<Blob>((resolve) => (deliver = resolve)));

    Object.defineProperty(document, "visibilityState", { value: "hidden", configurable: true });
    document.dispatchEvent(new Event("visibilitychange"));
    deliver(new Blob());
    await reading;

    expect(audio).not.toHaveBeenCalled();
  });
});
