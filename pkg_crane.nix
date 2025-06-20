{ rustPlatform, lib, pkgs, name, version, inputs, ... }:

let
  craneLib = inputs.crane.mkLib pkgs;
  src = lib.cleanSourceWith {
    src = ./.; # The original, unfiltered source
    filter = path: type:
      (lib.hasSuffix ".html" path) || (lib.hasSuffix "tailwind.config.js" path)
      ||
      # Example of a folder for images, icons, etc
      (lib.hasInfix "/assets/" path) || (lib.hasInfix "/css/" path) ||
      # Default filter from crane (allow .rs files)
      (craneLib.filterCargoSources path type);
  };
  commonArgs = {
    inherit src version;
    strictDeps = true;
    pname = name;
    buildInputs = with pkgs; [
      cargo-leptos
      lld
      binaryen
      dart-sass
      sqlx-cli
      makeWrapper
      tailwindcss
    ];
  };

  artifacts = craneLib.buildDepsOnly commonArgs;
  package = craneLib.buildPackage (commonArgs // {
    cargoArtifacts = artifacts;

    buildPhaseCommand = "cargo leptos build --release -vvv";

    nativeBuildInputs = commonArgs.buildInputs;

    installPhaseCommand = ''
      # mkdir -p $out/bin
      # cp target/server/release/${name} $out/bin/
      # cp -r target/site $out/bin/
      # wrapProgram $out/bin/${name} \
      #   --set LEPTOS_SITE_ROOT $out/bin/site


      # WHERE IS MY site/ DIR?
      mkdir -p $out/bin
      cp target/release/${name} $out/bin
      echo LS target/
      ls target/
    '';
  });

in {
  packages = { crane = package; };
  apps.default = inputs.flake-utils.lib.mkApp { drv = package; };
}
