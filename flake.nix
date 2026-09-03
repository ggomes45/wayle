{
  description = "Dev shell for wayle (Wayland desktop shell)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            cmake
            clang
            rustToolchain
          ];

        buildInputs = with pkgs; [
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

          env = {
            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          };

          shellHook = ''
            echo "wayle dev shell ready. rustc: $(rustc --version)"
          '';
        };
      });
}
