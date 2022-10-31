# Opaque
All encompasing solution for the website, registration, checkin, judging, etc.

# Deployment Instructions

`nix` is the preferred way to deploy opaque. This repository comes with a nix
flake. Add this repository as an input to your system flake at
`/etc/nixos/flake.nix`, and add the nixos module it comes with to the module
list. A minimal example:

```nix
{
  description = "NixOS configuration";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    opaque.url = "github:minnehack/Opaque";
  };

  outputs = { self, nixpkgs, opaque, ... }: {
    # 'rec' here so we can refer to the system attribute from the module list
    nixosConfigurations.casper = nixpkgs.lib.nixosSystem rec {

      system = "x86_64-linux";

      modules = [
        opaque.nixosModules.${system}.default
        ({ ... }: {
          services.opaque.enable = true;
          services.opaque.database =
            ''{ mh_reg = { url = "mysql://mysql:mysql@127.0.0.1/mh_reg" } }'';
        })
      ];
    };
  };
}
```

You'll need to run [init.sql](./migrations/init.sql) from this repository on your database
before you run opaque, in order to set up the schema. Change the first line
of that file to use the desired database (`mh_reg` in the example), then
run it:

`mysql -h <domain> -u <user> -p < init.sql`

Opaque comes with several configuration options-- the only one you'll
generally need to set is `services.opaque.database`.

- `services.opaque.enable`

Starts opaque and enables it on startup.

Example: true

- `services.opaque.port`

Which port opaque will expose itself on.

Default: 8001

- `services.opaque.address`

Which address(es) opaque will attempt to bind to, and repond to requests on.

Default: "0.0.0.0" (any address)

- `services.opaque.user`

Which user opaque will run as.

Default: "opaque"

- `services.opaque.group`

Which group opaque will run under; this should potentially be a list, but isn't
currently.

Default: "opaque"

- `services.opaque.limits`

Defines limits on things like file uploads.

Default: "{ file = "101MiB" }"

- `services.opaque.dataDir`

Where opaque will store data (like uploads).

Default: "/var/lib/opaque"

- `services.opaque.database`

Points opaque to the database.

Example: "{ mh_reg = { url = "mysql://mysql:mysql@127.0.0.1/mh_reg" } }"
