{
  description = "wordle-tui — a beautiful terminal word-guessing game built with ratatui";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = manifest.name;
          version = manifest.version;
          src = self;

          # Use the committed lockfile for reproducible, hash-free builds.
          cargoLock.lockFile = ./Cargo.lock;

          meta = with pkgs.lib; {
            description = manifest.description;
            homepage = manifest.homepage;
            license = licenses.mit;
            mainProgram = "wordle-tui";
          };
        };

        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/wordle-tui";
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            rustc
            clippy
            rustfmt
            rust-analyzer
          ];
        };
      }
    );
}
