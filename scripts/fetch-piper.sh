#!/usr/bin/env bash
# Downloads the Piper engine and a French voice into vendor/piper (macOS and Linux).
# Windows: scripts/fetch-piper.ps1. Layout and choices: docs/adr/0002-voix-piper.md.
#
#   scripts/fetch-piper.sh                      # default voice (fr_FR-siwis-medium)
#   scripts/fetch-piper.sh fr_FR-upmc-medium    # another voice
#
# Idempotent: what is already there is not downloaded again.
set -euo pipefail

VOICE="${1:-fr_FR-siwis-medium}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)/vendor/piper"
PIPER_RELEASE="https://github.com/rhasspy/piper/releases/download/2023.11.14-2"
PHONEMIZE_RELEASE="https://github.com/rhasspy/piper-phonemize/releases/download/2023.11.14-4"
VOICES_URL="https://huggingface.co/rhasspy/piper-voices/resolve/main"

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

download() { # url sha256(or empty) destination
  echo "↓ $1"
  curl -fsSL --retry 3 -o "$3" "$1"
  if [[ -n "$2" ]]; then
    local actual
    actual="$(shasum -a 256 "$3" | cut -d' ' -f1)"
    if [[ "$actual" != "$2" ]]; then
      echo "✗ Checksum mismatch for $1 (expected $2, got $actual)" >&2
      exit 1
    fi
  fi
}

install_macos() {
  local dir="$ROOT/macos"
  if [[ -x "$dir/piper" && -f "$dir/libpiper_phonemize.1.dylib" ]]; then
    echo "✓ Piper engine already in $dir"
    return
  fi

  # The 2023.11.14-2 release only ships an Intel (x86_64) build of Piper for macOS — the
  # "aarch64" archive is the same Intel binary — and its native libraries are missing.
  # They are taken from piper-phonemize (same release train). Apple Silicon runs it with Rosetta 2.
  download "$PIPER_RELEASE/piper_macos_x64.tar.gz" \
    ced85c0a3df13945b1e623b878a48fdc2854d5c485b4b67f62857cf551deaf8b "$TMP/piper.tar.gz"
  download "$PHONEMIZE_RELEASE/piper-phonemize_macos_x64.tar.gz" \
    9ec6e300c0d012a663758bc45a097b47ee759761a3b91c7742de042af789d84b "$TMP/phonemize.tar.gz"

  rm -rf "$dir" && mkdir -p "$dir"
  tar -xzf "$TMP/piper.tar.gz" -C "$TMP"
  tar -xzf "$TMP/phonemize.tar.gz" -C "$TMP"
  cp "$TMP/piper/piper" "$dir/"
  cp -R "$TMP/piper/espeak-ng-data" "$dir/"
  find "$TMP/piper-phonemize/lib" -maxdepth 1 -name '*.dylib' -exec cp -P {} "$dir/" \;
  chmod +x "$dir/piper"
  xattr -dr com.apple.quarantine "$dir" 2>/dev/null || true

  if [[ "$(uname -m)" == "arm64" ]] && ! arch -x86_64 /usr/bin/true 2>/dev/null; then
    echo "⚠ Rosetta 2 is required to run Piper on this Mac:"
    echo "    softwareupdate --install-rosetta --agree-to-license"
  fi
  echo "✓ Piper engine installed in $dir"
}

install_linux() {
  local dir="$ROOT/linux"
  if [[ -x "$dir/piper" ]]; then
    echo "✓ Piper engine already in $dir"
    return
  fi
  if [[ "$(uname -m)" != "x86_64" ]]; then
    echo "✗ Only Linux x86_64 is supported by this script." >&2
    exit 1
  fi

  download "$PIPER_RELEASE/piper_linux_x86_64.tar.gz" \
    a50cb45f355b7af1f6d758c1b360717877ba0a398cc8cbe6d2a7a3a26e225992 "$TMP/piper.tar.gz"
  rm -rf "$dir" && mkdir -p "$dir"
  tar -xzf "$TMP/piper.tar.gz" -C "$TMP"
  cp -R "$TMP/piper/." "$dir/"
  echo "✓ Piper engine installed in $dir"
}

install_voice() { # fr_FR-siwis-medium → fr/fr_FR/siwis/medium
  local name="$1" locale speaker quality
  IFS='-' read -r locale speaker quality <<< "$name"
  if [[ -z "$locale" || -z "$speaker" || -z "$quality" ]]; then
    echo "✗ Voice name expected as <locale>-<name>-<quality>, e.g. fr_FR-siwis-medium" >&2
    exit 1
  fi

  mkdir -p "$ROOT/voices"
  local base="$VOICES_URL/${locale%%_*}/$locale/$speaker/$quality"
  if [[ -f "$ROOT/voices/$name.onnx" && -f "$ROOT/voices/$name.onnx.json" ]]; then
    echo "✓ Voice $name already in $ROOT/voices"
    return
  fi

  download "$base/$name.onnx" "" "$TMP/$name.onnx"
  download "$base/$name.onnx.json" "" "$TMP/$name.onnx.json"
  download "$base/MODEL_CARD" "" "$TMP/$name.MODEL_CARD"
  mv "$TMP/$name.onnx" "$TMP/$name.onnx.json" "$TMP/$name.MODEL_CARD" "$ROOT/voices/"
  echo "✓ Voice $name installed in $ROOT/voices (licence: $ROOT/voices/$name.MODEL_CARD)"
}

case "$(uname -s)" in
  Darwin) install_macos ;;
  Linux) install_linux ;;
  *) echo "✗ Unsupported OS: use scripts/fetch-piper.ps1 on Windows." >&2; exit 1 ;;
esac
install_voice "$VOICE"
