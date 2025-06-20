{ rustPlatform, lib, pkgs, name, version, ... }:

rustPlatform.buildRustPackage rec {
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
