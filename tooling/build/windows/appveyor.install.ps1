$scriptPath = split-path -parent $MyInvocation.MyCommand.Definition
#. $scriptPath\appveyor.common.ps1

$env:MSYS2_BASEVER = "20160205"
$env:MSYS2_ARCH = "x86_64"
$env:DIR_TEMP= "c:\temp"
$env:DIR_MSYS2_DOWNLOAD= "${env:DIR_TEMP}\msys_download"
$env:DIR_RUST_DOWNLOAD= "${env:DIR_TEMP}\rust_download"
$env:DIR_RUST_INSTALL= "${env:DIR_TEMP}\rust"

if (${env:TARGET}.endsWith("gnu")) {
   $env:DIR_MSYS2_INSTALL="${env:APPVEYOR_BUILD_FOLDER}\msys64"
} else {
   $env:DIR_MSYS2_INSTALL="c:\msys64"      
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
    $unixpath_msys2_install = unixify ${env:DIR_MSYS2_INSTALL}
    .\"sh" --login -c "cd ${unixpath_appveyor_build_folder}; export PATH=`$PATH:${unixpath_dir_rust_install}/bin:${unixpath_msys2_install}/mingw32/bin:${unixpath_msys2_install}/mingw64/bin; exec 0</dev/null; $command; exit $?"
}

Function install_msys2($download_dir, $arch, $ver) {
    #$download_loc ="https://kent.dl.sourceforge.net/project/msys2/Base/${arch}/msys2-base-${arch}-${ver}.tar.xz"
    $download_loc = "http://downloads.sourceforge.net/project/msys2/Base/${arch}/msys2-base-${arch}-${ver}.tar.xz?r=&ts=1456082848&use_mirror=iweb"
    echo "downloading from $download_loc"
    if (-not (Test-Path "${download_dir}\msys2.tar.xz")) { 
        Start-FileDownload $download_loc -FileName "${download_dir}\msys2.tar.xz"
    } else {
        echo "using recent msys2 image from cache"
    }
    echo "extracting"
    7z x "${download_dir}\msys2.tar.xz"
    7Z x "msys2.tar" | Out-Null
    echo "starting msys"
    mbash("")
    $pkglist="mingw-w64-${arch}-{ragel,freetype,icu,gettext} libtool pkg-config gcc make autoconf automake perl"
    mbash("for i in {1..3}; do pacman --noconfirm -Suy $pkglist && break || sleep 15; done")
}

Function install_rust($download_dir, $install_dir, $target_rs_triple, $rustc_ver) {
    $download_loc = "https://static.rust-lang.org/dist/rust-${rustc_ver}-${target_rs_triple}.exe"
    $dated_ver = $rustc_ver
    if ($rustc_ver -eq "nightly" -or $rustc_ver -eq "beta") {
       $today = Get-Date -UFormat "%Y-%m-%d"
       $dated_ver = "${rustc_ver}.${today}"
    }
    if (-not (Test-Path "${download_dir}\rust.${dated_ver}.exe")) {
        echo "downloading from $download_loc"
        rm ${download_dir}\rust*
        Start-FileDownload $download_loc -FileName "${download_dir}\rust.$dated_ver.exe"
    } else {
        echo "using recent rust image from cache"
    }
    echo "installing rust"
    cd ${download_dir}
    echo "running $rust.dated_ver.exe"
    ls
    .\"rust.$dated_ver.exe" /VERYSILENT /NORESTART /DIR="$install_dir" | Out-Null
}

Function mktmpdir($tmpdir_path) {
    New-Item -ItemType Directory -Force -Path $tmpdir_path | Out-Null
}

mktmpdir ${env:DIR_TEMP}
mktmpdir ${env:DIR_MSYS2_DOWNLOAD}
mktmpdir ${env:DIR_RUST_DOWNLOAD}
if (${env:TARGET}.endsWith("gnu")) {
   install_msys2 ${env:DIR_MSYS2_DOWNLOAD} ${env:MSYS2_ARCH} ${env:MSYS2_BASEVER}
}
install_rust ${env:DIR_RUST_DOWNLOAD} ${env:DIR_RUST_INSTALL} ${env:TARGET} ${env:RUSTC_V}
