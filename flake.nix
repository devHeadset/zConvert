{
  description = "zconvert";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        buildInputs = with pkgs; [ rustc cargo ffmpeg unzip unrar gnutar ];
      in {
        packages.zconvert = pkgs.rustPlatform.buildRustPackage {
          pname = "zconvert";
          version = "0.1.0";
          src = ./.;
          cargoLock = { lockFile = ./Cargo.lock; };
          buildInputs = buildInputs;
        };

        devShells.default = pkgs.mkShell {
          packages = buildInputs;
        };

        defaultPackage = self.packages.${system}.zconvert;
      });
}
