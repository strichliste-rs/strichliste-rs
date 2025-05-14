{ rustPlatform, lib, pkgs, ... }:

rustPlatform.buildRustPackage rec {
  name = "ladon";
  pname = name;
  version = "1.0";

  src = ./.;

  nativeBuildInputs = with pkgs; [
    cargo-leptos
    lld
    binaryen
    dart-sass
    sqlx-cli
    makeWrapper
  ];

  cargoHash = "sha256-AMuSaJljSt4pKE8jTNigJWoFILHL03JxSZCfkoNBv14=";
  useFetchCargoVendor = true;

  buildPhase = ''
    cargo leptos build --release -vvv
  '';

  installPhase = ''
    mkdir -p $out/bin
    cp target/release/${name} $out/bin/
    cp -r target/site $out/bin/
    wrapProgram $out/bin/${name} \
      --set LEPTOS_SITE_ROOT $out/bin/site
  '';

  meta = with lib; {
    description = "A program";
    license = licenses.gpl2;
    platforms = platforms.all;
  };
}
