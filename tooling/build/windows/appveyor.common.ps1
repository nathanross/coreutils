$scriptPath = split-path -parent $MyInvocation.MyCommand.Definition

$env:MBASH =
$env:MSYS2_BASEVER = "20160205"
$env:MSYS2_ARCH = "x86_64"
$env:MSYS2_DIR="${env:APPVEYOR_BUILD_FOLDER}\tmp\msys2"
$env:RUST_DIR="${env:APPVEYOR_BUILD_FOLDER}\tmp\rust"
$env:PATH="${env:PATH}:${env:RUST_DIR}\bin:${env:MSYS2_DIR}\bin"

mbash($command) {
    $MSYS2_DIR\msys64\usr\bin\sh --login -c "cd $APPVEYOR_BUILD_FOLDER; PATH=$PATH:/mingw64/bin:/mingw32/bin; exec 0</dev/null; $command"
}
