{ rustPlatform, lib, pkgs, name, version, config, ... }:
let
  cfg = config.services.strichliste-rs;
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

    cargoHash = "sha256-SCDmDIUWkCJnrvxEFwQ0k0ohs6+mcA7wSO12mlMGAoA=";

    buildPhase = ''
      cargo leptos build --release -vvv
    '';

    installPhase = ''
      mkdir -p $out/bin
      cp target/release/${name} $out/bin/
      cp -r target/site $out/bin/
      wrapProgram $out/bin/${name} \
        --set LEPTOS_SITE_ROOT $out/bin/site \
        --set LEPTOS_SITE_ADDR ${cfg.address}:${toString cfg.port}
    '';

    meta = with lib; {
      description = "A ditigal tally-sheet";
      license = licenses.gpl2;
      platforms = platforms.all;
    };
  };

in package
