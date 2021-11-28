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
      ];
      shellHook = ''
        DATABASE_URL=mysql://root:root@localhost/mh_reg_test; export DATABASE_URL
      '';
    };

#     nixosConfigurations.minnehack = nixpkgs.lib.nixosSystem {
#       system = "x86_64-linux";
#       modules =
#         [ ({ pkgs, ... }: {
#           # Let 'nixos-version --json' know about the Git revision
#           # of this flake
#           system.configurationRevision = nixpkgs.lib.mkIf (self ? rev) self.rev;

#           networking.useDHCP = false;
#           networking.firewall.allowedTCPPorts = [ 80 ];

#           services.httpd = {
#             enable = true;
#             adminAddr = "test@example.com";
#           };
#         })
#       ];
#     };
    
#     deploy.nodes.minnehack.profiles.system = {
#       user = "root";
#       path = deploy-rs.lib.x86_64-linux.activate.nixos self.nixosConfigurations.minnehack;
#     };

#     checks = builtins.mapAttrs (system: deployLib: )
  });
}

