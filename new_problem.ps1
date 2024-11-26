param (
  [Parameter(Mandatory = $true)]
  [string]$raw
)

case kebab $raw *>&1 | Out-Null
$kebab = Get-Clipboard

Set-Location .\problems

New-Item -ItemType Directory -Path $kebab *>&1 | Out-Null
Set-Location $kebab

New-Item -ItemType Directory -Path python *>&1 | Out-Null
New-Item -ItemType Directory -Path csharp *>&1 | Out-Null
cargo new --bin rust *>&1 | Out-Null

Set-Location ..
Set-Location ..
