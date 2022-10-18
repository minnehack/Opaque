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
              pkgs.dockerTools.usrBinEnv
              pkgs.dockerTools.binSh
              # pkgs.dockerTools.caCertificates
              pkgs.dockerTools.fakeNss
              pkgs.gnused
              pkgs.hostname
              pkgs.mysql
              pkgs.bash
              pkgs.coreutils
            ];
            pathsToLink = [ "/bin" ];
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

            mysql_install_db --user=mysql --ldata=/var/lib/mysql --basedir="${pkgs.mysql}"

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
        podman load < ${mysqlEnv}
		podman container rm $(podman container stop \
			$(podman ps -a -q --filter ancestor=localhost/mysql:latest))
        podman run --detach --rm -it -p 3306:3306 localhost/mysql:latest
        '';
    };
  });
}
