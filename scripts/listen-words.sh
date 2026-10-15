#!/usr/bin/env bash
# Listening check of the spelling words read by the voice (SUB026706, ADR 0004): WAV files of a
# sample of words, or of a skill, in a temporary folder (never embedded), with the list to listen to.
# Needs Piper: scripts/fetch-piper.sh first.
#
#   scripts/listen-words.sh                          # 50 words drawn at random
#   scripts/listen-words.sh --sample 20
#   scripts/listen-words.sh --skill ce1-son-o-au-eau  # the words of a skill
#   scripts/listen-words.sh --slow                   # also each word read very slowly (0.5)
#   scripts/listen-words.sh --again stylo            # reads « stylo » again, replacing its cached reading
set -euo pipefail
cd "$(dirname "$0")/.."

# The voice cache of the application (same folder as educator-data::app_data_directory).
if [[ -n "${EDUCATOR_DATA_DIR:-}" ]]; then
  data_dir="$EDUCATOR_DATA_DIR"
elif [[ "$(uname)" == "Darwin" ]]; then
  data_dir="$HOME/Library/Application Support/Educator"
else
  data_dir="${XDG_CONFIG_HOME:-$HOME/.config}/Educator"
fi

cargo run --quiet --release -p educator-content --example listen_words -- --cache "$data_dir/speech-cache" "$@"
