{ pkgs ? import <nixpkgs> {} }:

let moz = pkgs.fetchFromGitHub {
              owner = "mozilla";
              repo = "nixpkgs-mozilla";
              # commit from: 2019-05-15
              rev = "9f35c4b09fd44a77227e79ff0c1b4b6a69dff533";
              sha256 = "18h0nvh55b5an4gmlgfbvwbyqj91bklf1zymis6lbdh75571qaz0";
          };
    rust_channels = (import "${moz.out}/rust-overlay.nix" pkgs pkgs).latest.rustChannels;
    rust = rust_channels.nightly.rust;
    cargo = rust_channels.nightly.cargo;

in

pkgs.stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = [ rust cargo pkgs.pkgconfig ];
  }
  

