{
	inputs = {
		gitignore = {
			url = "github:hercules-ci/gitignore.nix";
			inputs.nixpkgs.follows = "nixpkgs";
		};
		fenix = {
			url = "github:nix-community/fenix/monthly";
            inputs.nixpkgs.follows = "nixpkgs";
		};
	};

	outputs = { self, nixpkgs, flake-utils, gitignore, fenix }: flake-utils.lib.eachDefaultSystem (system:
		let
			pkgs = import nixpkgs { inherit system; };
			stdenv = pkgs.stdenv;
			lib = pkgs.lib;
			toolchain = fenix.packages.${system}.default.toolchain;
			rustPlatform = pkgs.makeRustPlatform { cargo = toolchain; rustc = toolchain; };
			datrieTestUnits = [ "test_byte_alpha" "test_byte_list" "test_file" "test_iterator" "test_nonalpha" "test_null_trie" "test_store-retrieve" "test_term_state" "test_walk" ];
		in rec {
			packages = {
				default = packages.datrie-rs.override {
					nativeBuildInputs = [ pkgs.llvmPackages.bintools ];
					buildFeatures = [ "cffi" ];
					buildNoDefaultFeatures = false;
					postInstall = ''
						ln -s $out/lib/libdatrie.so $out/lib/libdatrie.so.1
					'';
				};
				defaultDebug = packages.default.override { buildType = "debug"; };
				datrie-rs = lib.makeOverridable rustPlatform.buildRustPackage {
					pname = "datrie-rs";
					version = "0.0.1";
					src = gitignore.lib.gitignoreSource ./.;

					buildFeatures = [];
					buildNoDefaultFeatures = true;

					cargoLock = {
						lockFile = ./Cargo.lock;
					};

					doCheck = false;

					meta = {
						description = "A reimplementation of double-array structure for representing trie, ported from libdatrie to Rust";
						license = lib.licenses.lgpl21Only;
					};
				};
				datrieTest = pkgs.libdatrie.overrideAttrs {
					pname = "libdatrie-test";

					buildFlags = "check";
					outputs = [ "out" ];

					installPhase = ''
						runHook preInstall

						mkdir -p $out/bin
						(cd tests/.libs; cp ${lib.escapeShellArgs datrieTestUnits} $out/bin)

						runHook postInstall
					'';
					postInstall = "";
				};
			};
			checks = builtins.listToAttrs (builtins.map (test: {
				name = test;
				value = pkgs.runCommand test {
					LD_LIBRARY_PATH = lib.makeLibraryPath [ packages.defaultDebug ];
				} ''
					${packages.datrieTest}/bin/${test} > $out
				'';
			}) datrieTestUnits);
			apps.dropinTest = let
				testProgram = pkgs.writeShellScript "dropintest.t" ''
					export SHARNESS_TEST_OUTDIR=/tmp

					test_description='Run datrie tests'
					. ${pkgs.sharness}/share/sharness/sharness.sh

					for t in ${lib.escapeShellArgs datrieTestUnits}; do
					  test_expect_success $t "
						LD_LIBRARY_PATH=${lib.makeLibraryPath [ packages.defaultDebug ]} ${packages.datrieTest}/bin/$t
					  "
					done

					test_done
				'';
			in {
				type = "app";
				program = "${testProgram}";
			};
			devShells.default = pkgs.mkShell {
				buildInputs = [ pkgs.llvmPackages.bintools pkgs.cargo fenix.packages.${system}.complete.toolchain ];
			};
		}
	);
}
