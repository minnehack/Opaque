{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
  flake-utils.lib.eachDefaultSystem (system:
  let
    pkgs = nixpkgs.legacyPackages.${system};
    mysqlEnv = pkgs.dockerTools.buildImage {
          name = "mysql";
          tag = "latest";

          copyToRoot = pkgs.buildEnv {
            name = "image-root";
            paths = [
              pkgs.gnused
              pkgs.hostname
              pkgs.mysql
            ];
            pathsToLink = [ "/bin" "/share" ];
          };

          runAsRoot = ''
            #!${pkgs.runtimeShell}

            ${pkgs.dockerTools.shadowSetup}

            groupadd -r mysql
            useradd -r -g mysql mysql

            mkdir -p /run/mysqld
            chown mysql:mysql /run/mysqld

            mkdir -p /var/lib/mysql
            chown mysql:mysql /var/lib/mysql

            mkdir /tmp
            chmod 777 /tmp

            hostname 127.0.0.1
            printf '%s\n' '127.0.0.1' localhost >> /etc/hosts

            mysql_install_db --user=mysql --ldata=/var/lib/mysql

            printf '%s\n%s\n%s\n' 'CREATE DATABASE mh_reg;' \
                                  "GRANT ALL ON mh_reg.* to 'mysql'@'%' IDENTIFIED BY 'mysql';" \
                                  'FLUSH PRIVILEGES;' > /init.sql
            chown mysql:mysql /init.sql
          '';
          
          config = {
            User = "mysql";
            Cmd = [ "/bin/mysqld" "--init-file=/init.sql" ];
          };
      };
  in {
    packages.default = pkgs.rustPlatform.buildRustPackage {
      pname = "Opaque";
      version = "0.1.0";
      src = ./.;
      cargoLock.lockFile = ./Cargo.lock;
      nativeBuildInputs = [ pkgs.libmysqlclient.dev ];
    };
    nixosModules.default = ({ config, lib, pkgs, ... }:
      let
        cfg = config.services.opaque;
      in {
        options.services.opaque = {
          enable = lib.mkEnableOption "Opaque server";
          configFile = lib.mkOption {
            type = lib.types.path;
          };
          dataDir = lib.mkOption {
            type = lib.types.path;
            default = /var/lib/opaque;
          };
        };

        config = lib.mkIf cfg.enable {
          systemd.services.opaque = {
            wantedBy = [ "multi-user.target" ];
            serviceConfig.ExecStart = "${pkgs.opaque}/bin/opaque";
          };
        };
      });
    devShells.default = pkgs.mkShell {
      buildInputs = [
        pkgs.rustc
        pkgs.cargo
        pkgs.rust-analyzer
        pkgs.clippy
        pkgs.sqlx-cli
        pkgs.mysql
        pkgs.rustfmt
      ];
      # source instead of running it normally here since it needs to export
      # an environment variable
      #
      # could also set this in directly the shell hook if wanted, but i would
      # rather the logic be contained in that script
      shellHook = ''
        source ./flake-scripts/shellHook.sh ${mysqlEnv}
        '';
    };
  });
}
