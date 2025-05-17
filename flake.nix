{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, ... }@inputs:
    inputs.flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
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
          # shellHook = ''
          #   if [ -f ".direnv/stylance_pid" ]; then
          #     pid=$(cat ".direnv/stylance_pid")
          #     if ps -p $pid > /dev/null
          #     then
          #       echo "Stylance is running: $pid"
          #     else
          #       _=$(stylance -w ./ & echo $! > ".direnv/stylance_pid")&
          #   else
          #     _=$(stylance -w ./ & echo $! > ".direnv/stylance_pid")&
          #   fi
          # '';
        };

        # packages.default = package;
        packages.default = pkgs.callPackage ./pkg.nix { };
      });
}
