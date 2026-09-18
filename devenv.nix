{ pkgs, lib, ... }:

{
  packages = with pkgs; [
    pkg-config
    cmake
    clang
    gtk4
    gtk4-layer-shell
    gtksourceview5
    libpulseaudio
    fftw
    pipewire
    systemd
    udev
    libxkbcommon
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-src" "rust-analyzer" ];
  };

  env.LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

  enterShell = ''
    echo "wayle dev shell ready. rustc: $(rustc --version)"
  '';
}
