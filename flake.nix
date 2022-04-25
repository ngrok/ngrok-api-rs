{
  description = "A Rust project using naersk";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, naersk, flake-utils }:
    let
      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      name = cargoToml.package.name;
    in
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        naersk-lib = naersk.lib.${system};
        defaultPackage = naersk-lib.buildPackage {
          pname = name;
          root = ./.;
          nativeBuildInputs = with pkgs; [
            pkg-config
            openssl
          ];
        };
      in
      rec {
        inherit defaultPackage;

        packages = builtins.listToAttrs [{ inherit name; value = defaultPackage; }];

        devShell = pkgs.mkShell {
          inputsFrom = [ defaultPackage ];
          buildInputs = with pkgs; [
            pkgs.rust-analyzer
            pkgs.cargo
            pkgs.rustc
          ];
          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        };
      }
    );
}
