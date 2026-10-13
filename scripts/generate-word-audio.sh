#!/usr/bin/env bash
# Pre-generates the audio of the spelling words into content/audio/words (SUB001404).
# Needs Piper: scripts/fetch-piper.sh first. Only the words added or changed are generated again.
#
#   scripts/generate-word-audio.sh              # every word
#   scripts/generate-word-audio.sh --sample 50  # plus a list of 50 words to listen to (ecoute.txt)
set -euo pipefail
cd "$(dirname "$0")/.."
cargo run --quiet --release -p educator-content --example generate_word_audio -- "$@"
