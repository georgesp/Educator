# Pre-generates the audio of the spelling words into content\audio\words (SUB001404), Windows.
# Needs Piper: .\scripts\fetch-piper.ps1 first. Only the words added or changed are generated again.
#
#   .\scripts\generate-word-audio.ps1
#   .\scripts\generate-word-audio.ps1 --sample 50
$ErrorActionPreference = "Stop"
Set-Location (Split-Path $PSScriptRoot -Parent)
cargo run --quiet --release -p educator-content --example generate_word_audio -- @args
