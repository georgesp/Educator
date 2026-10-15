import { useEffect, useState } from "react";
import { useApi } from "../app/BackendContext";

/**
 * Development page (route /dev/voix, never in the built application): study of the iPad voice
 * (SUB023804). It measures the system voice through the Web Speech API (on iPadOS, WebKit reads it
 * with AVSpeechSynthesizer: the same engine and voices as a native plugin) and checks whether the
 * WebView plays a WAV received after an asynchronous call, as `useSpeech` does today.
 * Texts here are samples, not product copy.
 */

const SAMPLES = [
  { label: "Mot", text: "un stylo." },
  { label: "Phrase", text: "J'écris avec un stylo." },
  { label: "Consigne", text: "Écoute bien le mot, puis écris-le." },
] as const;

const RATES = [
  { label: "Normale (1)", rate: 1 },
  { label: "Consignes (0,9)", rate: 0.9 },
  { label: "Lente (0,5)", rate: 0.5 },
] as const;

type Result = { test: string; detail: string };

/** Results survive a reload of the page (Vite, app relaunch): kept in this WebView only. */
const STORAGE_KEY = "educator.voiceLab.results";

function loadResults(): Result[] {
  try {
    const saved = window.localStorage.getItem(STORAGE_KEY);
    return saved ? (JSON.parse(saved) as Result[]) : [];
  } catch {
    return [];
  }
}

function saveResults(results: Result[]): void {
  try {
    window.localStorage.setItem(STORAGE_KEY, JSON.stringify(results));
  } catch {
    // Storage unavailable: the results stay on screen only.
  }
}

const round = (ms: number) => Math.round(ms);
const sleep = (ms: number) => new Promise<void>((resolve) => setTimeout(resolve, ms));
const hasSpeech = () => typeof window !== "undefined" && "speechSynthesis" in window;

/** Reads `text` with the system voice; resolves with the time to the first sound and the total time. */
function speak(text: string, voice: SpeechSynthesisVoice | undefined, rate: number): Promise<string> {
  if (!hasSpeech()) {
    return Promise.resolve("speechSynthesis absent");
  }
  const startedAt = performance.now();
  const utterance = new SpeechSynthesisUtterance(text);
  utterance.lang = "fr-FR";
  utterance.rate = rate;
  if (voice) {
    utterance.voice = voice;
  }
  return new Promise((resolve) => {
    let firstSound: number | null = null;
    const timeout = setTimeout(() => resolve("pas de fin après 15 s"), 15_000);
    utterance.onstart = () => {
      firstSound = performance.now() - startedAt;
    };
    utterance.onend = () => {
      clearTimeout(timeout);
      const total = performance.now() - startedAt;
      resolve(`début ${firstSound === null ? "?" : round(firstSound)} ms, fin ${round(total)} ms`);
    };
    utterance.onerror = (event) => {
      clearTimeout(timeout);
      resolve(`erreur : ${event.error}`);
    };
    window.speechSynthesis.cancel();
    window.speechSynthesis.speak(utterance);
  });
}

/** 0.4 s beep at 440 Hz, 16-bit mono WAV: the format the core returns today. */
function beepWav(): Blob {
  const sampleRate = 22_050;
  const samples = Math.floor(sampleRate * 0.4);
  const buffer = new ArrayBuffer(44 + samples * 2);
  const view = new DataView(buffer);
  const ascii = (offset: number, text: string) => {
    for (let i = 0; i < text.length; i += 1) {
      view.setUint8(offset + i, text.charCodeAt(i));
    }
  };
  ascii(0, "RIFF");
  view.setUint32(4, 36 + samples * 2, true);
  ascii(8, "WAVE");
  ascii(12, "fmt ");
  view.setUint32(16, 16, true);
  view.setUint16(20, 1, true);
  view.setUint16(22, 1, true);
  view.setUint32(24, sampleRate, true);
  view.setUint32(28, sampleRate * 2, true);
  view.setUint16(32, 2, true);
  view.setUint16(34, 16, true);
  ascii(36, "data");
  view.setUint32(40, samples * 2, true);
  for (let i = 0; i < samples; i += 1) {
    const value = Math.sin((2 * Math.PI * 440 * i) / sampleRate) * 0.3 * 32_767;
    view.setInt16(44 + i * 2, Math.round(value), true);
  }
  return new Blob([buffer], { type: "audio/wav" });
}

/** Asks the core for a WAV (system voice through the plugin on the iPad), then plays it. */
async function playFromCore(load: () => Promise<Blob>): Promise<string> {
  const startedAt = performance.now();
  try {
    const wav = await load();
    const received = performance.now() - startedAt;
    const url = URL.createObjectURL(wav);
    const audio = new Audio(url);
    const ended = new Promise<void>((resolve) => {
      audio.addEventListener("ended", () => resolve(), { once: true });
      audio.addEventListener("error", () => resolve(), { once: true });
    });
    await audio.play();
    await ended;
    URL.revokeObjectURL(url);
    const seconds = Number.isFinite(audio.duration) ? `${audio.duration.toFixed(2)} s` : "?";
    return `WAV reçu en ${round(received)} ms (${Math.round(wav.size / 1024)} Ko, ${seconds}), joué`;
  } catch (error) {
    if (error instanceof Error) {
      return `échec (${error.name} : ${error.message})`;
    }
    return `échec (${JSON.stringify(error)})`;
  }
}

/** Plays the beep after `wait`, as `useSpeech` plays the WAV after the `speak` command. */
async function playWavAfter(wait: () => Promise<unknown>): Promise<string> {
  try {
    await wait();
    const url = URL.createObjectURL(beepWav());
    const audio = new Audio(url);
    audio.addEventListener("ended", () => URL.revokeObjectURL(url), { once: true });
    await audio.play();
    return "joué";
  } catch (error) {
    return error instanceof Error ? `refusé (${error.name} : ${error.message})` : `refusé (${String(error)})`;
  }
}

export function VoiceLabScreen() {
  const api = useApi();
  const [voices, setVoices] = useState<SpeechSynthesisVoice[]>([]);
  const [voiceUri, setVoiceUri] = useState("");
  const [results, setResults] = useState<Result[]>(loadResults);
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    if (!hasSpeech()) {
      return undefined;
    }
    const load = () =>
      setVoices(window.speechSynthesis.getVoices().filter((v) => v.lang.toLowerCase().startsWith("fr")));
    load();
    window.speechSynthesis.addEventListener("voiceschanged", load);
    return () => window.speechSynthesis.removeEventListener("voiceschanged", load);
  }, []);

  useEffect(() => saveResults(results), [results]);

  const voice = voices.find((v) => v.voiceURI === voiceUri);
  const record = (test: string, detail: string) => setResults((previous) => [...previous, { test, detail }]);

  const run = async (test: string, action: () => Promise<string>) => {
    setBusy(true);
    try {
      record(test, await action());
    } finally {
      setBusy(false);
    }
  };

  const report = JSON.stringify(
    {
      userAgent: navigator.userAgent,
      voices: voices.map((v) => ({ name: v.name, lang: v.lang, uri: v.voiceURI, local: v.localService })),
      results,
    },
    null,
    2,
  );

  return (
    <main className="screen demo">
      <h1 className="title">Labo voix (SUB023804)</h1>

      <section>
        <h2>1. Voix françaises de l'appareil ({voices.length})</h2>
        <select value={voiceUri} onChange={(event) => setVoiceUri(event.target.value)}>
          <option value="">Voix par défaut (fr-FR)</option>
          {voices.map((v) => (
            <option key={v.voiceURI} value={v.voiceURI}>
              {v.name} — {v.lang} — {v.voiceURI.split(".").slice(-3, -2).join("")}
            </option>
          ))}
        </select>
      </section>

      <section>
        <h2>2. Lecture directe par le système</h2>
        {RATES.map(({ label: rateLabel, rate }) => (
          <div className="demo-row" key={rate}>
            <strong>{rateLabel}</strong>
            {SAMPLES.map(({ label, text }) => (
              <button
                key={label}
                type="button"
                disabled={busy}
                onClick={() => run(`${label} — ${rateLabel} — ${voice?.name ?? "défaut"}`, () => speak(text, voice, rate))}
              >
                {label}
              </button>
            ))}
          </div>
        ))}
        <div className="demo-row">
          <button
            type="button"
            disabled={busy}
            onClick={() => run("Voix système après 1,5 s", () => sleep(1500).then(() => speak("un stylo.", voice, 1)))}
          >
            Voix système après 1,5 s
          </button>
        </div>
      </section>

      <section>
        <h2>3. Lecture d'un WAV reçu après une attente (contrat actuel)</h2>
        <div className="demo-row">
          <button type="button" disabled={busy} onClick={() => run("WAV tout de suite", () => playWavAfter(() => Promise.resolve()))}>
            Tout de suite
          </button>
          <button type="button" disabled={busy} onClick={() => run("WAV après 300 ms", () => playWavAfter(() => sleep(300)))}>
            Après 300 ms
          </button>
          <button type="button" disabled={busy} onClick={() => run("WAV après 1,5 s", () => playWavAfter(() => sleep(1500)))}>
            Après 1,5 s
          </button>
          <button type="button" disabled={busy} onClick={() => run("WAV après get_status", () => playWavAfter(() => api.getStatus()))}>
            Après un appel au cœur
          </button>
        </div>
      </section>

      <section>
        <h2>5. Voix native par le cœur (commandes speak, speak_word)</h2>
        <p>Chaîne complète : cœur Rust, plugin Swift, WAV, lecture par le front. Essayer aussi en mode silencieux.</p>
        {[
          { label: "Consignes (0,9)", speed: 0.9 },
          { label: "Mots (0,85)", speed: 0.85 },
          { label: "Lente (0,5)", speed: 0.5 },
        ].map(({ label: speedLabel, speed }) => (
          <div className="demo-row" key={speed}>
            <strong>{speedLabel}</strong>
            {SAMPLES.map(({ label, text }) => (
              <button
                key={label}
                type="button"
                disabled={busy}
                onClick={() => run(`Natif ${label} — ${speedLabel}`, () => playFromCore(() => api.speak(text, speed)))}
              >
                {label}
              </button>
            ))}
          </div>
        ))}
        <div className="demo-row">
          <strong>Mot seul (« Le mot : … » coupé)</strong>
          {["jaune", "beau", "un stylo"].map((word) => (
            <button
              key={word}
              type="button"
              disabled={busy}
              onClick={() => run(`Natif speak_word « ${word} »`, () => playFromCore(() => api.speakWord(word)))}
            >
              {word}
            </button>
          ))}
          <button
            type="button"
            disabled={busy}
            onClick={() => run("Natif speak_word « jaune » lent", () => playFromCore(() => api.speakWord("jaune", false, true)))}
          >
            jaune (lent)
          </button>
          <button
            type="button"
            disabled={busy}
            onClick={() => run("Natif speak_word « jaune » + phrase", () => playFromCore(() => api.speakWord("jaune", true)))}
          >
            jaune (phrase)
          </button>
        </div>
        <div className="demo-row">
          <button
            type="button"
            disabled={busy}
            onClick={() =>
              run("État de la voix (get_status)", async () => JSON.stringify(await api.getStatus()))
            }
          >
            État de la voix
          </button>
        </div>
      </section>

      <section>
        <h2>4. Résultats ({results.length})</h2>
        <button type="button" onClick={() => setResults([])}>
          Effacer les résultats
        </button>
        <ul>
          {results.map((r, index) => (
            <li key={index}>
              {r.test} : {r.detail}
            </li>
          ))}
        </ul>
        <p>À copier pour Claude :</p>
        <textarea readOnly rows={10} cols={80} value={report} onFocus={(event) => event.currentTarget.select()} />
      </section>
    </main>
  );
}
