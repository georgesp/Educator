# Listening check of the spelling words read by the voice (SUB026706, ADR 0004), Windows: WAV files
# of a sample of words, or of a skill, in a temporary folder (never embedded), with the list to
# listen to. Needs Piper: .\scripts\fetch-piper.ps1 first.
#
#   .\scripts\listen-words.ps1                          # 50 words drawn at random
#   .\scripts\listen-words.ps1 --skill ce1-son-o-au-eau  # the words of a skill
#   .\scripts\listen-words.ps1 --slow                   # also each word read very slowly (0.5)
#   .\scripts\listen-words.ps1 --again stylo            # reads « stylo » again, replacing its cached reading
$ErrorActionPreference = "Stop"
Set-Location (Split-Path $PSScriptRoot -Parent)

# The voice cache of the application (same folder as educator-data::app_data_directory).
$dataDir = if ($env:EDUCATOR_DATA_DIR) { $env:EDUCATOR_DATA_DIR } else { Join-Path $env:APPDATA "Educator" }

cargo run --quiet --release -p educator-content --example listen_words -- --cache (Join-Path $dataDir "speech-cache") @args
