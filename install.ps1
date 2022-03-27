cargo build --release

$RegistryPath = 'HKCU:\Software\Google\Chrome\NativeMessagingHosts\de.nilsalex.yktotp'
$Name = '(Default)'
$location = Get-Location
$Value = "$location\manifest\de.nilsalex.yktotp-windows.json"

If (-NOT(Test-Path $RegistryPath))
{
    New-Item -Path $RegistryPath -Force | Out-Null
}
New-ItemProperty -Path $RegistryPath -Name $Name -Value $Value -PropertyType String -Force
