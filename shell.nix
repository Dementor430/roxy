let
  rust-overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust-overlay ]; };
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    openssl
    openssl.dev
    (rust-bin.stable.latest.default.override {
      extensions = [ "rust-src" ];
    })
  ];

  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}
