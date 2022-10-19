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
        if command -v podman; then
          containerCommand=podman
        elif command -v docker; then
          containerCommand=docker
        else
          printf '%s\n' 'Install Podman or Docker to continue.'
          exit 1
        fi

        $containerCommand load < ${mysqlEnv}
        $containerCommand container rm $($containerCommand container stop \
            $($containerCommand ps -a -q --filter ancestor=localhost/mysql:latest))
        $containerCommand run --detach --rm -it -p 3306:3306 localhost/mysql:latest

        mysql --host=127.0.0.1 --user=mysql --password=mysql < up.sql

        export DATABASE_URL=mysql://mysql:mysql@127.0.0.1/mh_reg
        '';
    };
  });
}
