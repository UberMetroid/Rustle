{
  description = "Minimalist Nix-built container for Rustle";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    shared-assets = {
      url = "github:UberMetroid/shared-assets?ref=v3.0.3";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, shared-assets, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };
        rustPlatform = pkgs.makeRustPlatform {
          rustc = rustVersion;
          cargo = rustVersion;
        };

        # 1. Build the WASM frontend & backend combined in single root
        app = rustPlatform.buildRustPackage {
          pname = "rustle";
          version = "0.1.26";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "shared-core-3.0.0" = "sha256-ozJJ4XDZOA3BbTBrHhN3gi/2xRIuVnAq940QyNltMl8=";
              "shared-frontend-3.0.0" = "sha256-ozJJ4XDZOA3BbTBrHhN3gi/2xRIuVnAq940QyNltMl8=";
            };
          };

          nativeBuildInputs = [
            rustVersion
            pkgs.wasm-bindgen-cli
            pkgs.trunk
            pkgs.tailwindcss
          ];

          buildPhase = ''
            export HOME=$TMPDIR
            mkdir -p frontend/shared-assets
            cp -r ${shared-assets}/* frontend/shared-assets/
            # Build frontend assets
            cd frontend
            trunk build --release
            cd ..
            # Build backend server binary
            cargo build --release --bin server
          '';

          installPhase = ''
            mkdir -p $out/bin
            mkdir -p $out/dist
            cp target/release/server $out/bin/server
            cp -r frontend/dist/* $out/dist/
          '';
        };

        # 2. Create the layered Docker container image
        dockerImage = pkgs.dockerTools.buildLayeredImage {
          name = "rustle-nix";
          tag = "latest";
          
          config = {
            Cmd = [ "${app}/bin/server" ];
            WorkingDir = "/app";
            Env = [
              "PORT=4409"
            ];
            ExposedPorts = {
              "4409/tcp" = {};
            };
            User = "nobody:nobody";
          };

          extraCommands = ''
            mkdir -p app
            cp -r ${app}/dist app/dist
          '';
        };

      in {
        packages = {
          inherit app dockerImage;
          default = dockerImage;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustVersion
            pkgs.trunk
            pkgs.wasm-bindgen-cli
            pkgs.tailwindcss
          ];
        };
      }
    );
}
