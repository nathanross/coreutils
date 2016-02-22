$scriptPath = split-path -parent $MyInvocation.MyCommand.Definition
. $scriptPath\appveyor.common.ps1

Function install_msys2($download_dir, $arch, $ver) {
    #$download_loc ="https://kent.dl.sourceforge.net/project/msys2/Base/${arch}/msys2-base-${arch}-${ver}.tar.xz"
    $download_loc = "http://downloads.sourceforge.net/project/msys2/Base/${arch}/msys2-base-${arch}-${ver}.tar.xz?r=&ts=1456082848&use_mirror=iweb"
    if (-not (Test-Path "${download_dir}\msys2.tar.xz")) {
        echo "downloading from $download_loc"
        Start-FileDownload $download_loc -FileName "${download_dir}\msys2.tar.xz"
    } else {
        echo "using cached msys2 package"
    }
    echo "extracting"
    7z x "${download_dir}\msys2.tar.xz"
    7Z x "msys2.tar" | Out-Null
    echo "starting msys"
    mbash("")
    mbash("for i in {1..3}; do pacman --noconfirm -Suy mingw-w64-${arch}-{ragel,freetype,icu,gettext} libtool pkg-config gcc make autoconf automake perl && break || sleep 15; done")
}

Function install_rust($download_dir, $install_dir, $target_rs_triple, $rustc_ver) {
    $download_loc = "https://static.rust-lang.org/dist/rust-${rustc_ver}-${target_rs_triple}.exe"
    echo "downloading from $download_loc"
    if (-not (Test-Path "${download_dir}\rust.exe")) {
        Start-FileDownload $download_loc -FileName "${download_dir}\rust.exe"
    }
    echo "installing rust"
    cd ${download_dir}
    .\rust.exe /VERYSILENT /NORESTART /DIR="$install_dir" | Out-Null
}

Function mktmpdir($tmpdir_path) {
    New-Item -ItemType Directory -Force -Path $tmpdir_path | Out-Null
}

mktmpdir ${env:DIR_TEMP}
mktmpdir ${env:DIR_BUILD_CACHE} 
install_msys2 ${env:DIR_MSYS2_DOWNLOAD} ${env:MSYS2_ARCH} ${env:MSYS2_BASEVER}
install_rust ${env:DIR_RUST_DOWNLOAD} ${env:DIR_RUST_INSTALL} ${env:TARGET} ${env:RUSTC_V}
