{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, ... }@inputs:
    inputs.flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        inherit (pkgs) lib;

        toml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        name = toml.package.name;
        version = toml.package.version;
      in {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            # openssl
            # pkg-config
            rust-analyzer
            cargo-leptos # does work. Make sure '/home/ole/.cargo/bin/cargo-leptos' does not exist
            lld
            dart-sass
            sqlite.dev
            sqlx-cli
            rustfmt
            stylance-cli
            clippy
            vscode-langservers-extracted
            binaryen
            cargo-generate
            tailwindcss
            tailwindcss-language-server
            vscode-extensions.bradlc.vscode-tailwindcss
          ];

          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          DATABASE_URL = "sqlite:tmp/db.sqlite";
        };
        # } // (lib.mkMerge [
        #   (pkgs.callPackage ./pkg.nix { inherit name version; })
        #   (pkgs.callPackage ./pkg_crane.nix { inherit name version inputs; })
        # ]));
      } // (pkgs.callPackage ./pkg.nix { inherit name version; }));
  # });
}
