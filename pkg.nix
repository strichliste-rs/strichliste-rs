{ rustPlatform, lib, pkgs, name, version, config, ... }:
let
  package = rustPlatform.buildRustPackage rec {
    inherit name version;
    pname = name;

    src = ./.;

    nativeBuildInputs = with pkgs; [
      cargo-leptos
      lld
      binaryen
      dart-sass
      sqlx-cli
      makeWrapper
      tailwindcss
    ];

    cargoHash = "sha256-CscEKhiG6N9CAF9W7eVEkU6etSxTv7SPUCCOoeu0BKM=";

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
