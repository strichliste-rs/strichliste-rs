{ rustPlatform, lib, pkgs, name, version, inputs, ... }:
let
  package = rustPlatform.buildRustPackage rec {
    inherit name version;
    pname = name;

    src = lib.fileset.toSource {
      root = ./.;
      fileset = (lib.fileset.unions [
        ./Cargo.toml
        ./Cargo.lock
        ./src
        ./public
        ./migrations
        ./.sqlx
        ./style
      ]);
    };

    nativeBuildInputs = with pkgs; [
      cargo-leptos
      lld
      binaryen
      dart-sass
      sqlx-cli
      makeWrapper
      tailwindcss
    ];

    cargoHash = "sha256-l29KtcjU6B/OIXo+OZuXp6r8eSOshvsOBuNz65Uq+mM=";

    buildPhase = ''
      cargo leptos build --release -vvv
    '';

    installPhase = ''
      mkdir -p $out/bin
      cp target/release/${name} $out/bin/
      cp -r target/site $out/bin/
    '';

    meta = with lib; {
      description = "A ditigal tally-sheet";
      license = licenses.gpl2;
      platforms = platforms.all;
    };
  };

in package
