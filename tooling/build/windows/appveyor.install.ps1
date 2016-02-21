$scriptPath = split-path -parent $MyInvocation.MyCommand.Definition
. $scriptPath\common.ps1

Function install_msys2($install_dir, $arch, $ver) {
    $MBASH ="msys64\usr\bin\sh --login -c"
    echo Start-FileDownload "https://kent.dl.sourceforge.net/project/msys2/Base/${arch}/msys2-base-${arch}-${ver}.tar.xz" -FileName "msys2.tar.xz"
    Start-FileDownload "https://kent.dl.sourceforge.net/project/msys2/Base/${arch}/msys2-base-${arch}-${ver}.tar.xz" -FileName "msys2.tar.xz"
    7z x msys2.tar.xz
    7Z x msys2.tar > NUL
    $mbash="msys64\usr\bin\sh --login -c"
#    $mbash ""
#    $mbash "for i in {1..3}; do pacman --noconfirm -Suy mingw-w64-%MSYS2_ARCH%-{ragel,freetype,icu,gettext} libtool pkg-config gcc make autoconf automake perl && break || sleep 15; done"
}

Function install_rust($install_dir, $target_rs_triple, $rustc_ver) {
    Start-FileDownload "https://static.rust-lang.org/dist/rust-${env:rustc_ver}-${target_rs_triple}.exe" -FileName rust.exe
    rust.exe /VERYSILENT /NORESTART /DIR="$install_dir"
}

install_msys2(${env:MSYS2_DIR}, ${env:MSYS2_ARCH}, ${env:MSYS2_BASEVER})
install_rust(${env:RUST_DIR}, ${env:TARGET}, ${env:RUSTC_V})
