/// WAV file (16-bit PCM, mono) holding `samples`: 16-bit little-endian samples, as given by the
/// system voice. A trailing odd byte is dropped.
pub fn wav_from_pcm16(samples: &[u8], sample_rate: u32) -> Vec<u8> {
    let data = &samples[..samples.len() - samples.len() % 2];
    let data_len = u32::try_from(data.len()).unwrap_or(u32::MAX - 36);
    let mut wav = Vec::with_capacity(44 + data.len());
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36 + data_len).to_le_bytes());
    wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16u32.to_le_bytes()); // size of the format chunk
    wav.extend_from_slice(&1u16.to_le_bytes()); // PCM
    wav.extend_from_slice(&1u16.to_le_bytes()); // mono
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&(sample_rate * 2).to_le_bytes()); // bytes per second
    wav.extend_from_slice(&2u16.to_le_bytes()); // bytes per frame
    wav.extend_from_slice(&16u16.to_le_bytes()); // bits per sample
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_len.to_le_bytes());
    wav.extend_from_slice(data);
    wav
}

#[cfg(test)]
mod tests {
    use super::wav_from_pcm16;

    #[test]
    fn header_describes_16_bit_mono_pcm() {
        let samples: Vec<u8> = [0i16, 1000, -1000, i16::MAX]
            .iter()
            .flat_map(|s| s.to_le_bytes())
            .collect();
        let wav = wav_from_pcm16(&samples, 22_050);

        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(u32::from_le_bytes(wav[4..8].try_into().unwrap()), 36 + 8);
        assert_eq!(&wav[8..16], b"WAVEfmt ");
        assert_eq!(u16::from_le_bytes([wav[20], wav[21]]), 1);
        assert_eq!(u16::from_le_bytes([wav[22], wav[23]]), 1);
        assert_eq!(u32::from_le_bytes(wav[24..28].try_into().unwrap()), 22_050);
        assert_eq!(u32::from_le_bytes(wav[28..32].try_into().unwrap()), 44_100);
        assert_eq!(u16::from_le_bytes([wav[34], wav[35]]), 16);
        assert_eq!(&wav[36..40], b"data");
        assert_eq!(u32::from_le_bytes(wav[40..44].try_into().unwrap()), 8);
        assert_eq!(&wav[44..], samples.as_slice());
    }

    #[test]
    fn a_trailing_odd_byte_is_dropped() {
        let wav = wav_from_pcm16(&[1, 2, 3], 16_000);

        assert_eq!(wav.len(), 44 + 2);
        assert_eq!(&wav[44..], &[1, 2]);
    }
}
