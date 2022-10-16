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
    devShell = pkgs.mkShell {
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
        DATABASE_URL=mysql://root:root@localhost/mh_reg; export DATABASE_URL
      '';
    };
  });
}

