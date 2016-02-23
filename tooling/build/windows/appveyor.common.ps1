$scriptPath = split-path -parent $MyInvocation.MyCommand.Definition

$env:MSYS2_BASEVER = "20160205"
$env:MSYS2_ARCH = "x86_64"
$env:DIR_TEMP= "c:\temp"
$env:DIR_BUILD_CACHE= "c:\cache"
$env:DIR_MSYS2_DOWNLOAD= "${env:DIR_TEMP}"
$env:DIR_RUST_DOWNLOAD= "${env:DIR_BUILD_CACHE}"
$env:DIR_RUST_INSTALL= "${env:DIR_TEMP}\rust"

if (${env:TARGET}.endsWith("gnu")) {
   $env:DIR_MINGW_INSTALL= "${env:APPVEYOR_BUILD_FOLDER}\msys64"
} else {
   $env:DIR_MINGW_INSTALL= "c:\msys64"      
}

$env:MINGW_DIR_USED = "${env:APPVEYOR_BUILD_FOLDER}"

$env:PATH="${env:PATH}:${env:RUST_INSTALL}\bin:${env:DIR_MINGW_INSTALL}\bin:${env:DIR_MINGW_INSTALL}\usr\bin"

Function unixify($winpath) {
    return $winpath -replace 'c:','/c' -replace '\\','/'
}

Function mbash($command) {
    echo "mingw call: ${command}"
    cd "${env:DIR_MSYS2_INSTALL}\usr\bin\"
    $unixpath_appveyor_build_folder = unixify ${env:APPVEYOR_BUILD_FOLDER}
    $unixpath_dir_rust_install = unixify ${env:DIR_RUST_INSTALL}
    $unixpath_mingw_install = unixify ${env:DIR_MINGW_INSTALL}
    .\"sh" --login -c "cd ${unixpath_appveyor_build_folder}; export PATH=`$PATH:${unixpath_dir_rust_install}/bin:${unixpath_mingw_install}/mingw32/bin:${unixpath_mingw_install}/mingw64/bin; exec 0</dev/null; $command; exit $?"
}

