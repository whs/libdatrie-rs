{
  description = "Rust implementation of libdatrie";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    aeneas.url = "github:aeneasverif/aeneas";
  };

  outputs = { self, nixpkgs, flake-utils, aeneas }:
    (flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        inherit (pkgs) lib;

        datrie = pkgs.rustPlatform.buildRustPackage {
          pname = "libdatrie-rs";
          version = "1.0.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          outputs = [ "out" "utils" ];
          nativeBuildInputs = [ pkgs.cargo-c pkgs.pkg-config ];
          buildInputs = lib.optionals pkgs.stdenv.hostPlatform.isDarwin [ pkgs.libiconv ];

          doCheck = false;
          buildPhase = ''
            runHook preBuild
            cargo cbuild --frozen --release
            cargo build --frozen --release --features bin
            runHook postBuild
          '';
          installPhase = ''
            runHook preInstall
            cargo cinstall --frozen --release --destdir="$out" --prefix=/ --libdir=lib
            install -Dm0755 target/release/trietool "$utils/bin/trietool"

            mkdir -p "$utils/nix-support"
            printf '%s\n' "$out" > "$utils/nix-support/propagated-user-env-packages"
            runHook postInstall
          '';

          meta = {
            description = "Rust port of libdatrie";
            homepage = "https://github.com/whs/libdatrie-rs";
            license = lib.licenses.lgpl21Plus;
          };
        };
      in {
        packages = {
          default = datrie;
          libdatrie-rs = datrie;
          libdatrie-rs-utils = datrie.utils;
        };

        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.rustup
            pkgs.cargo-c
            pkgs.cargo-fuzz
            pkgs.lean4
            aeneas.packages.${system}.aeneas
            aeneas.packages.${system}.charon
            pkgs.lean-lsp-mcp
            pkgs.stdenv.cc
          ];
        };
      })) // {
        overlays.default = final: _prev: {
          libdatrie = self.packages.${final.stdenv.hostPlatform.system}.libdatrie-rs;
        };
      };
}
