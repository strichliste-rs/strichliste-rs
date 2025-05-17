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
        # craneLib = inputs.crane.mkLib pkgs;

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

        packages.default = pkgs.callPackage ./pkg.nix { inherit name version; };
        # packages.default = let
        #   toml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        #   name = toml.package.name;
        #   version = toml.package.version;

        #   src = craneLib.cleanCargoSource ./.;

        #   commonArgs = {
        #     inherit src version;
        #     pname = name;
        #     buildInputs = with pkgs; [ cargo-leptos binaryen lld ];
        #   };

        #   artifacts = craneLib.buildDepsOnly commonArgs;
        # in craneLib.buildPackage commonArgs // {
        #   cargoArtifacts = artifacts;

        #   buildInputs = with pkgs; [
        #     pkgs.cargo-leptos
        #     pkgs.binaryen
        #     tailwindcss
        #   ];
        #   buildPhaseCommand = "cargo leptos build --release -vvv";

        #   nativeBuildInputs = with pkgs; [ makeWrapper ];
        #   installPhaseCommand = ''
        #     mkdir -p $out/bin
        #     cp target/server/release/${name} $out/bin/
        #     echo out is $out
        #     echo target ls
        #     ls target/
        #     cp -r target/site $out/bin/
        #     wrapProgram $out/bin/${name} \
        #       --set LEPTOS_SITE_ROOT $out/bin/site
        #   '';
        # };
      });
}
