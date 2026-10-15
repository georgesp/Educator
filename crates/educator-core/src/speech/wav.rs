//! Cutting a reading at its first pause (decision of 2026-10-15, ADR 0004): a word alone is read
//! by the voice in a short carrier, « Le mot : jaune. », where it is rendered far more steadily than
//! alone (« jaune, » was heard « jeuné », « beau. » « beu »); the child only hears what follows the
//! pause of the colon. Works on any WAV of 16-bit PCM (Piper, a system voice): mono or several
//! channels, any rate. A file that cannot be read, or without such a pause, is kept whole.

/// Length of the windows of the measure.
const WINDOW_MS: usize = 10;

/// A window is silent below this share of the loudest window.
const SILENCE_RATIO: f64 = 0.02;

/// Shortest pause taken for the colon: the closure of a stop consonant (« auto », « bateau ») lasts
/// 60 to 110 ms, the pause of the colon 230 to 530 ms (measured with `fr_FR-siwis-medium`).
const MIN_PAUSE_MS: usize = 150;

/// Silence kept before the word, so that its first sound is never cut.
const MARGIN_MS: usize = 120;

/// The reading from its first pause (see the module documentation), or the reading whole.
pub fn keep_after_first_pause(wav: &[u8]) -> Vec<u8> {
    cut(wav).unwrap_or_else(|| wav.to_vec())
}

fn cut(wav: &[u8]) -> Option<Vec<u8>> {
    let format = parse(wav)?;
    let frame = usize::from(format.channels) * 2;
    let samples = &wav[format.data_start..format.data_start + format.data_len];
    let frames = samples.len() / frame;
    let window = (format.sample_rate as usize * WINDOW_MS / 1000).max(1);

    // Loudness of each window (root mean square of the first channel).
    let loudness: Vec<f64> = (0..frames / window)
        .map(|w| {
            let sum: f64 = (w * window..(w + 1) * window)
                .map(|f| {
                    let s = i16::from_le_bytes([samples[f * frame], samples[f * frame + 1]]);
                    f64::from(s) * f64::from(s)
                })
                .sum();
            (sum / window as f64).sqrt()
        })
        .collect();
    let loudest = loudness.iter().copied().fold(0.0, f64::max);
    if loudest <= 0.0 {
        return None;
    }
    let silent: Vec<bool> = loudness
        .iter()
        .map(|l| *l < loudest * SILENCE_RATIO)
        .collect();

    // The first pause long enough, once the speech has started, before the end of the reading.
    let min_pause = MIN_PAUSE_MS / WINDOW_MS;
    let speech = silent.iter().position(|s| !s)?;
    let mut w = speech;
    while w < silent.len() {
        if !silent[w] {
            w += 1;
            continue;
        }
        let start = w;
        while w < silent.len() && silent[w] {
            w += 1;
        }
        if w < silent.len() && w - start >= min_pause {
            let keep_from = (w - start).min(MARGIN_MS / WINDOW_MS);
            let first_frame = (w - keep_from) * window;
            return Some(with_data(wav, &format, &samples[first_frame * frame..]));
        }
    }
    None
}

struct Format {
    channels: u16,
    sample_rate: u32,
    data_start: usize,
    data_len: usize,
}

/// The format and the samples of a RIFF/WAVE file of 16-bit PCM.
fn parse(wav: &[u8]) -> Option<Format> {
    if wav.len() < 12 || &wav[0..4] != b"RIFF" || &wav[8..12] != b"WAVE" {
        return None;
    }
    let u16_at = |i: usize| Some(u16::from_le_bytes(wav.get(i..i + 2)?.try_into().ok()?));
    let u32_at = |i: usize| Some(u32::from_le_bytes(wav.get(i..i + 4)?.try_into().ok()?));

    let (mut channels, mut sample_rate, mut bits) = (None, None, None);
    let mut at = 12;
    while at + 8 <= wav.len() {
        let id = &wav[at..at + 4];
        let size = u32_at(at + 4)? as usize;
        let body = at + 8;
        if id == b"fmt " {
            if u16_at(body)? != 1 {
                return None; // not PCM
            }
            channels = Some(u16_at(body + 2)?);
            sample_rate = Some(u32_at(body + 4)?);
            bits = Some(u16_at(body + 14)?);
        } else if id == b"data" {
            let len = size.min(wav.len().saturating_sub(body));
            let channels = channels.filter(|c| *c > 0)?;
            if bits? != 16 {
                return None;
            }
            return Some(Format {
                channels,
                sample_rate: sample_rate.filter(|r| *r > 0)?,
                data_start: body,
                data_len: len - len % (usize::from(channels) * 2),
            });
        }
        at = body + size + size % 2;
    }
    None
}

/// The same file with other samples: the chunks before the data are kept, the sizes rewritten.
fn with_data(wav: &[u8], format: &Format, samples: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(format.data_start + samples.len());
    out.extend_from_slice(&wav[..format.data_start]);
    out.extend_from_slice(samples);
    let data_size = u32::try_from(samples.len()).unwrap_or(u32::MAX);
    out[format.data_start - 4..format.data_start].copy_from_slice(&data_size.to_le_bytes());
    let riff_size = u32::try_from(out.len() - 8).unwrap_or(u32::MAX);
    out[4..8].copy_from_slice(&riff_size.to_le_bytes());
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: u32 = 22050;

    /// A mono WAV: `(milliseconds, loud)` segments, a tone when loud.
    fn wav(segments: &[(usize, bool)]) -> Vec<u8> {
        let mut samples: Vec<u8> = Vec::new();
        for &(ms, loud) in segments {
            for i in 0..RATE as usize * ms / 1000 {
                let value: i16 = if loud {
                    (8000.0 * (i as f64 * 0.12).sin()) as i16
                } else {
                    3
                };
                samples.extend_from_slice(&value.to_le_bytes());
            }
        }
        let mut out = Vec::new();
        out.extend_from_slice(b"RIFF");
        out.extend_from_slice(&(36 + samples.len() as u32).to_le_bytes());
        out.extend_from_slice(b"WAVEfmt ");
        out.extend_from_slice(&16u32.to_le_bytes());
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&RATE.to_le_bytes());
        out.extend_from_slice(&(RATE * 2).to_le_bytes());
        out.extend_from_slice(&2u16.to_le_bytes());
        out.extend_from_slice(&16u16.to_le_bytes());
        out.extend_from_slice(b"data");
        out.extend_from_slice(&(samples.len() as u32).to_le_bytes());
        out.extend_from_slice(&samples);
        out
    }

    fn duration_ms(wav: &[u8]) -> usize {
        let format = parse(wav).unwrap();
        format.data_len / 2 * 1000 / RATE as usize
    }

    #[test]
    fn the_carrier_is_cut_at_the_pause_of_the_colon() {
        // Silence, « Le mot », the colon, « jaune » (with the closure of a stop), final silence.
        let reading = wav(&[
            (200, false),
            (450, true),
            (300, false),
            (150, true),
            (80, false),
            (300, true),
            (250, false),
        ]);

        let word = keep_after_first_pause(&reading);

        // 120 ms of silence kept, then the word whole, its closure included, and the end.
        let expected = 120 + 150 + 80 + 300 + 250;
        assert!(
            duration_ms(&word).abs_diff(expected) <= 20,
            "{}",
            duration_ms(&word)
        );
        assert_eq!(&word[0..4], b"RIFF");
        let riff = u32::from_le_bytes(word[4..8].try_into().unwrap()) as usize;
        assert_eq!(word.len() - 8, riff);
    }

    #[test]
    fn a_reading_without_a_long_pause_is_kept_whole() {
        let reading = wav(&[
            (200, false),
            (300, true),
            (90, false),
            (300, true),
            (300, false),
        ]);

        assert_eq!(reading, keep_after_first_pause(&reading));
    }

    #[test]
    fn what_is_not_a_pcm_wav_is_kept_whole() {
        assert_eq!(b"not a wav".to_vec(), keep_after_first_pause(b"not a wav"));
        assert_eq!(Vec::<u8>::new(), keep_after_first_pause(&[]));
    }
}
