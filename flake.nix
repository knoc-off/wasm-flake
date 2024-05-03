{
  description = "wasm-app";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustPlatform = pkgs.makeRustPlatform {
          cargo = pkgs.cargo;
          rustc = pkgs.rustc;
        };
        commonBuildInputs = with pkgs; [
          openssl.dev
          pkg-config
          zlib.dev
        ] ++ lib.optionals stdenv.isDarwin (with darwin.apple_sdk.frameworks; [
          libiconv
          CoreServices
          SystemConfiguration
        ]);
      in
      rec {
        packages.actix-web-example = rustPlatform.buildRustPackage rec {
          pname = "app";
          version = "0.1.0";
          src = ./.;

          nativeBuildInputs = with pkgs; [
            (rust-bin.stable."1.76.0".default.override {
              extensions = [ "rust-src" ];
              targets = [ "wasm32-unknown-unknown" ];
            })
            pkg-config
            trunk
            binaryen
            dart-sass
            tailwindcss
            wasm-bindgen-cli
          ];

          buildInputs = commonBuildInputs ++ [
            pkgs.super-tiny-icons
          ];

          cargoLock.lockFile = ./Cargo.lock;

          buildPhase = ''
            runHook preBuild
            mkdir -p $TMPDIR/output
            mkdir -p icons
            cp -r ${pkgs.super-tiny-icons}/* icons/
            #trunk build --release --offline --dist $TMPDIR/output
            trunk build --release --offline --dist $TMPDIR/output --public-url /
            runHook postBuild
          '';

          installPhase = ''
            runHook preInstall
            mkdir -p $out/lib
            cp -r $TMPDIR/output/* $out/lib/
            runHook postInstall
          '';
        };

        # Python webserver script
        packages.webserver = pkgs.writeShellScriptBin "webserver" ''
          ${pkgs.python3}/bin/python -m http.server --directory ${packages.actix-web-example}/lib 8080
        '';

        defaultPackage = packages.actix-web-example;

        apps.webserver = flake-utils.lib.mkApp {
          drv = packages.webserver;
        };

        defaultApp = apps.webserver;

        devShell = pkgs.mkShell {
          inherit commonBuildInputs;
          nativeBuildInputs = with pkgs; [
            cargo-edit
            cargo-generate
            (rust-bin.stable."1.76.0".default.override {
              extensions = [ "rust-src" ];
              targets = [ "wasm32-unknown-unknown" ];
            })
            rust-analyzer
            sccache
            pkg-config
            trunk
            tailwindcss
            binaryen
            dart-sass
            wasm-bindgen-cli
          ];
          buildInputs = [
            pkgs.super-tiny-icons
          ];
          shellHook = ''
            mkdir -p icons
            cp -r ${pkgs.super-tiny-icons}/* icons/
          '';
          RUST_BACKTRACE = 1;
        };
      }
    );
}

