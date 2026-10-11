#!/usr/bin/env bash
# Generates the same sentences with every candidate French voice, to choose by ear
# (docs/adr/0002-voix-piper.md). Output: vendor/piper/samples/<voice>.wav
#
#   scripts/compare-voices.sh
set -euo pipefail

SCRIPTS="$(cd "$(dirname "$0")" && pwd)"
ROOT="$SCRIPTS/../vendor/piper"
# fr_FR-tom-medium is left out: AGPL-3.0 licence (docs/adr/0002-voix-piper.md).
CANDIDATES=(fr_FR-siwis-medium fr_FR-upmc-medium fr_FR-gilles-low)
TEXT="Bonjour ! Combien font sept plus huit ? Écris le mot : éléphant. Bravo, c'est la bonne réponse ! Les élèves dessinent des étoiles."

for voice in "${CANDIDATES[@]}"; do
  "$SCRIPTS/fetch-piper.sh" "$voice" >/dev/null
done

case "$(uname -s)" in
  Darwin) ENGINE="$ROOT/macos"; export DYLD_LIBRARY_PATH="$ENGINE" ;;
  *) ENGINE="$ROOT/linux"; export LD_LIBRARY_PATH="$ENGINE" ;;
esac

mkdir -p "$ROOT/samples"
for voice in "${CANDIDATES[@]}"; do
  speakers=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1])).get('num_speakers',1))" "$ROOT/voices/$voice.onnx.json")
  for ((speaker = 0; speaker < speakers; speaker++)); do
    suffix=""; [[ "$speakers" -gt 1 ]] && suffix="-speaker$speaker"
    out="$ROOT/samples/$voice$suffix.wav"
    # speaker_id only for multi-speaker voices: Piper rejects it otherwise.
    python3 -c "import json,sys;d={'text':sys.argv[1],'output_file':sys.argv[3]};n=int(sys.argv[4]);d.update({'speaker_id':int(sys.argv[2])} if n>1 else {});print(json.dumps(d))" \
      "$TEXT" "$speaker" "$out" "$speakers" \
      | "$ENGINE/piper" --model "$ROOT/voices/$voice.onnx" --espeak_data "$ENGINE/espeak-ng-data" --json-input --quiet >/dev/null
    echo "♪ $out"
  done
done
echo "Listen to the files in $ROOT/samples, then record the choice in docs/adr/0002-voix-piper.md."
