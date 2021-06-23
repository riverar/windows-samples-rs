$TemporaryPath = [Guid]::NewGuid().ToString("n")
if(!(Test-Path $TemporaryPath)) {
    New-Item -ItemType Directory -Path $TemporaryPath | Out-Null
}

$ProjectReunionUrl = [uri]"https://github.com/microsoft/ProjectReunion/releases/download/v0.8.0-rc/ProjectReunion.NuGetPackages.0.8.0-rc.zip"
Invoke-WebRequest -UseBasicParsing $ProjectReunionUrl -OutFile "$TemporaryPath/bundle.zip"

Expand-Archive .\$TemporaryPath\bundle.zip -DestinationPath .\$TemporaryPath -Force 
Expand-Archive .\$TemporaryPath\Microsoft.ProjectReunion.Foundation.0.8.0-rc.nupkg .\$TemporaryPath\foundation -Force
Expand-Archive .\$TemporaryPath\Microsoft.ProjectReunion.WinUI.0.8.0-rc.nupkg .\$TemporaryPath\winui -Force

@(
    [tuple]::Create("x86", "i686"),
    [tuple]::Create("x64", "x86_64"),
    [tuple]::Create("arm64", "aarch64")
) | ForEach-Object {
    $srcArch = $_.item1;
    $dstArch = $_.item2;
    $libPath = ".\.windows\lib\$dstArch\"
    $winmdPath = ".\.windows\winmd"

    New-Item -Path $libPath -ItemType Directory -Force | Out-Null
    New-Item -Path $winmdPath -ItemType Directory -Force | Out-Null
    Move-Item .\$TemporaryPath\foundation\lib\win10-$srcArch\* $libPath -Force
    Move-Item .\$TemporaryPath\foundation\runtimes\lib\native\$srcArch\* $libPath -Force
    Move-Item .\$TemporaryPath\foundation\lib\native\*.winmd $winmdPath -Force
    Move-Item .\$TemporaryPath\winui\lib\uap10.0\*.winmd $winmdPath -Force
}

Remove-Item .\$TemporaryPath -Recurse -Force
