# Downloads the Piper engine and a French voice into vendor\piper (Windows).
# macOS / Linux: scripts/fetch-piper.sh. Layout and choices: docs/adr/0002-voix-piper.md.
#
#   .\scripts\fetch-piper.ps1                          # default voice (fr_FR-siwis-medium)
#   .\scripts\fetch-piper.ps1 -Voice fr_FR-upmc-medium
#
# Idempotent: what is already there is not downloaded again.
param([string]$Voice = "fr_FR-siwis-medium")

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"
# Windows PowerShell 5.1 may default to TLS 1.0, which GitHub and Hugging Face refuse.
[Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12

$Root = Join-Path (Split-Path $PSScriptRoot -Parent) "vendor\piper"
$PiperRelease = "https://github.com/rhasspy/piper/releases/download/2023.11.14-2"
$VoicesUrl = "https://huggingface.co/rhasspy/piper-voices/resolve/main"
$Tmp = Join-Path ([System.IO.Path]::GetTempPath()) ("educator-piper-" + [guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Force -Path $Tmp | Out-Null

function Get-File([string]$Url, [string]$Sha256, [string]$Destination) {
    Write-Host "Downloading $Url"
    Invoke-WebRequest -Uri $Url -OutFile $Destination -UseBasicParsing
    if ($Sha256) {
        $actual = (Get-FileHash -Algorithm SHA256 $Destination).Hash.ToLowerInvariant()
        if ($actual -ne $Sha256) { throw "Checksum mismatch for $Url (expected $Sha256, got $actual)" }
    }
}

try {
    $engine = Join-Path $Root "windows"
    if (Test-Path (Join-Path $engine "piper.exe")) {
        Write-Host "OK Piper engine already in $engine"
    }
    else {
        $zip = Join-Path $Tmp "piper.zip"
        Get-File "$PiperRelease/piper_windows_amd64.zip" "f3c58906402b24f3a96d92145f58acba6d86c9b5db896d207f78dc80811efcea" $zip
        Expand-Archive -Path $zip -DestinationPath $Tmp -Force
        if (Test-Path $engine) { Remove-Item -Recurse -Force $engine }
        New-Item -ItemType Directory -Force -Path $engine | Out-Null
        Copy-Item -Recurse -Path (Join-Path $Tmp "piper\*") -Destination $engine
        Write-Host "OK Piper engine installed in $engine"
    }

    $parts = $Voice.Split("-")
    if ($parts.Count -ne 3) { throw "Voice name expected as <locale>-<name>-<quality>, e.g. fr_FR-siwis-medium" }
    $locale, $speaker, $quality = $parts
    $voices = Join-Path $Root "voices"
    New-Item -ItemType Directory -Force -Path $voices | Out-Null
    $model = Join-Path $voices "$Voice.onnx"
    if ((Test-Path $model) -and (Test-Path "$model.json")) {
        Write-Host "OK Voice $Voice already in $voices"
    }
    else {
        $base = "$VoicesUrl/$($locale.Split('_')[0])/$locale/$speaker/$quality"
        Get-File "$base/$Voice.onnx" "" (Join-Path $Tmp "$Voice.onnx")
        Get-File "$base/$Voice.onnx.json" "" (Join-Path $Tmp "$Voice.onnx.json")
        Get-File "$base/MODEL_CARD" "" (Join-Path $Tmp "$Voice.MODEL_CARD")
        Move-Item -Force (Join-Path $Tmp "$Voice.*") $voices
        Write-Host "OK Voice $Voice installed in $voices (licence: $voices\$Voice.MODEL_CARD)"
    }
}
finally {
    Remove-Item -Recurse -Force $Tmp -ErrorAction SilentlyContinue
}
