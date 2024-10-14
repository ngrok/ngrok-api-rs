{
  description = "A Rust project using naersk";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    # Note: fenix packages are cached via cachix:
    #       cachix use nix-community
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            inputs.fenix.overlays.default
          ];
        };
        lib = nixpkgs.lib;
        toolchain = pkgs.fenix.complete.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustc"
          "rustfmt"
          "rust-analyzer"
        ];
        # Make sure that cargo semver-checks uses the stable toolchain rather
        # than the nightly one that we normally develop with.
        semver-checks = with pkgs; symlinkJoin {
          name = "cargo-semver-checks";
          paths = [ cargo-semver-checks ];
          buildInputs = [ makeWrapper ];
          postBuild = ''
            wrapProgram $out/bin/cargo-semver-checks \
              --prefix PATH : ${rustc}/bin \
              --prefix PATH : ${cargo}/bin
          '';
        };
        extract-version = with pkgs; writeShellScriptBin "extract-crate-version" ''
          ${cargo}/bin/cargo metadata --format-version 1 --no-deps | \
            ${jq}/bin/jq -r ".packages[] | select(.name == \"$1\") | .version"
        '';
      in
      {
        devShell = pkgs.mkShell {
          packages = with pkgs; [
            # Needed to build the ngrok-api crate
            pkg-config
            openssl

            toolchain
            semver-checks
            extract-version
            cargo-udeps
          ];
          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          LD_LIBRARY_PATH = lib.makeLibraryPath [ pkgs.openssl ];
        };
      }
    );
}
