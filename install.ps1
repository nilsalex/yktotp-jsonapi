cargo build --release

Copy-Item ".\manifest\regedit.reg.tmpl" -Destination ".\manifest\regedit.reg"
$content = (Get-Content ".\manifest\regedit.reg" -Raw)
$location = (Get-Location).ToString().Replace('\', '\\')
$content.Replace('<GIT_ROOT>', $location) | Set-Content -Path ".\manifest\regedit.reg"
