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
  in rec {
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

          port = lib.mkOption {
            type = lib.types.int;
            default = 8001;
          };

          address = lib.mkOption {
            type = lib.types.str;
            default = "0.0.0.0";
          };

          user = lib.mkOption {
            type = lib.types.str;
            default = "opaque";
          };

          group = lib.mkOption {
            type = lib.types.str;
            default = "opaque";
          };

          limits = lib.mkOption {
            type = lib.types.str;
            default = ''{ file = "101MiB" }'';
          };

          dataDir = lib.mkOption {
            type = lib.types.str;
            default = "/var/lib/opaque";
          };

          database = lib.mkOption {
            type = lib.types.str;
          };
        };

        config = lib.mkIf cfg.enable {
          users = {
            users.opaque = {
              group = "${cfg.group}";
              isSystemUser = true;
            };
            groups.opaque = {};
          };

          systemd.services.opaque = {
            wantedBy = [ "multi-user.target" ];
            serviceConfig.User = "${cfg.user}";
            serviceConfig.ExecStartPre = [
              "+${pkgs.coreutils}/bin/mkdir -p ${cfg.dataDir}"
              "+${pkgs.coreutils}/bin/chown opaque:opaque ${cfg.dataDir}"
            ];
            serviceConfig.ExecStart = "${packages.default}/bin/opaque";
            serviceConfig.Restart = "always";
            serviceConfig.Environment = [
              # use single quotes here to not conflict with toml syntax for
              # strings
              "ROCKET_PORT='${builtins.toString cfg.port}'"
              "ROCKET_ADDRESS='${cfg.address}'"
              "ROCKET_LIMITS='${cfg.limits}'"
              "ROCKET_DATABASES='${cfg.database}'"
              "OPAQUE_DATA_DIR='${cfg.dataDir}'"
            ];
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
      shellHook = ''source ./flake-scripts/shellHook.sh ${mysqlEnv}'';
    };
  });
}
