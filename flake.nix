{
  description = "Build git-clean";

  inputs = {
    crate2nix.url = "github:nix-community/crate2nix/0.14.0";
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs-stable.url = "github:NixOS/nixpkgs/23.11";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    { flake-utils
    , nixpkgs
    , nixpkgs-stable
    , self
    , ...
    } @ inputs:
    flake-utils.lib.eachDefaultSystem (system:
    let
      stable-packages = final: _prev: {
        stable = import nixpkgs-stable {
          system = final.system;
          config.allowUnfree = true;
        };
      };
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          (import inputs.rust-overlay)
          stable-packages
        ];
      };

      lib = pkgs.lib;

      generatedBuild = import ./Cargo.nix {
        inherit pkgs;
        defaultCrateOverrides = with pkgs;
          defaultCrateOverrides
          // {
            git-clean = attrs: {
              buildInputs = lib.optionals stdenv.isDarwin [
                darwin.apple_sdk.frameworks.Security
              ];
            };
          };
      };

      git-clean = generatedBuild.rootCrate.build;
    in
    {
      checks =
        let
          packages = lib.mapAttrs' (n: lib.nameValuePair "package-${n}") self.packages;
          devShells = lib.mapAttrs' (n: lib.nameValuePair "devShell-${n}") self.devShells;
        in
        packages // devShells;

      packages = {
        default = git-clean;
        git-clean = git-clean;
      };

      devShells.default = pkgs.mkShell {
        packages =
          [
            pkgs.cargo-nextest
            pkgs.crate2nix
            pkgs.just
            pkgs.rust-analyzer
            pkgs.rust-bin.stable.latest.default
          ]
          ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv
            pkgs.darwin.apple_sdk.frameworks.Security
          ];
      };
    });
}

