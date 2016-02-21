$scriptPath = split-path -parent $MyInvocation.MyCommand.Definition
. $scriptPath\appveyor.common.ps1

Function install_msys2($temp_dir, $install_dir, $arch, $ver) {
    #$download_loc ="https://kent.dl.sourceforge.net/project/msys2/Base/${arch}/msys2-base-${arch}-${ver}.tar.xz"
    $download_loc = "http://downloads.sourceforge.net/project/msys2/Base/${arch}/msys2-base-${arch}-${ver}.tar.xz?r=&ts=1456082848&use_mirror=iweb"
    echo "downloading from $download_loc"    
    Start-FileDownload $download_loc -FileName "msys2.tar.xz"
    7z x "msys2.tar.xz"
    7Z x "msys2.tar" | Out-Null
    mbash("")
    mbash("for i in {1..3}; do pacman --noconfirm -Suy mingw-w64-%MSYS2_ARCH%-{ragel,freetype,icu,gettext} libtool pkg-config gcc make autoconf automake perl && break || sleep 15; done")
}

Function install_rust($temp_dir, $install_dir, $target_rs_triple, $rustc_ver) {
    $download_loc = "https://static.rust-lang.org/dist/rust-${rustc_ver}-${target_rs_triple}.exe"
    echo "downloading from $download_loc"
    (New-Object System.Net.WebClient).DownloadFile($download_loc, "rust.exe")
#    ./rust.exe /VERYSILENT /NORESTART /DIR="$install_dir"
}

Function mktmpdir($tmpdir_path) {
    New-Item -ItemType Directory -Force -Path $tmpdir_path
}

mktmpdir ${env:TEMP_DIR}
install_msys2 ${env:TEMP_DIR} ${env:MSYS2_DIR} ${env:MSYS2_ARCH} ${env:MSYS2_BASEVER}
install_rust ${env:TEMP_DIR} ${env:RUST_DIR} ${env:TARGET} ${env:RUSTC_V}
