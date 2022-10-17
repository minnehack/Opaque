{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
  flake-utils.lib.eachDefaultSystem (system:
  let
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.default = pkgs.rustPlatform.buildRustPackage {
      pname = "Opaque";
      version = "0.1.0";
      src = ./.;
      cargoLock.lockFile = ./Cargo.lock;
      nativeBuildInputs = [ pkgs.libmysqlclient.dev ];
    };
    devShells.default = pkgs.mkShell {
      buildInputs = [
        pkgs.rustc
        pkgs.cargo
        pkgs.rust-analyzer
        pkgs.clippy
        pkgs.diesel-cli
        pkgs.mysql
        pkgs.rustfmt
      ];
      shellHook = ''
        export DATABASE_URL=mysql://minnehack:minnehack@localhost/mh_reg_test
        '';
    };
  });
}

