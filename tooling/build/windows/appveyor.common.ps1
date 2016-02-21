$scriptPath = split-path -parent $MyInvocation.MyCommand.Definition

$env:MSYS2_BASEVER = "20160205"
$env:MSYS2_ARCH = "x86_64"
$env:REPO_TEMP_DIR= "${env:APPVEYOR_BUILD_FOLDER}\tmp"
$env:MSYS2_DIR = "${env:REPO_TEMP_DIR}\msys2"
$env:RUST_DIR= "${env:REPO_TEMP_DIR}\rust"
$env:PATH="${env:PATH}:${env:RUST_DIR}\bin:${env:MSYS2_DIR}\bin"

Function mbash($command) {
    echo "mbash"
#    ${env:MSYS2_DIR}\msys64\usr\bin\./sh --login -c "cd ${env:APPVEYOR_BUILD_FOLDER}; PATH=${PATH}:/mingw64/bin:/mingw32/bin; exec 0</dev/null; $command"
}
