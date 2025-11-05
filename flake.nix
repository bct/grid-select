{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
  };

  outputs = { self, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        inherit (nixpkgs) lib;

        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "grid-select";
          inherit ((lib.importTOML (self + "/Cargo.toml")).package) version;

          src = self;

          cargoLock.lockFile = self + "/Cargo.lock";

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            libxkbcommon
          ];
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            pkg-config
            libxkbcommon
          ];
        };
      }
    );
}
