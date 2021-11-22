{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
  flake-utils.lib.eachDefaultSystem (system:
  let
    overlays = [ (import rust-overlay) ];
    pkgs = import nixpkgs {
      inherit system overlays;
    };
  in {
    devShell = pkgs.mkShell {
      buildInputs = [
        (pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
        pkgs.diesel-cli
        pkgs.mysql
      ];
      shellHook = ''
        DATABASE_URL=mysql://root:root@localhost/mh_reg_test; export DATABASE_URL
      '';
    };
  });
}

