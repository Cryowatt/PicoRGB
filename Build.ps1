[CmdletBinding()]
param (
    [Parameter()]
    [string]
    $PicoDrivePath
)

docker compose build --progress plain sdk && docker compose run --rm sdk

if (Test-Path $PicoDrivePath) {
    Copy-Item .\bin\*.uf2 $PicoDrivePath
}
