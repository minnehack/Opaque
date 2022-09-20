{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    # deploy-rs.url = "github:serokell/deploy-rs";
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
        pkgs.rust-analyzer
        pkgs.clippy
      ];
      shellHook = ''
        DATABASE_URL=mysql://root:root@localhost/mh_reg; export DATABASE_URL
      '';
    };
  });
}

