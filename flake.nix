{
  description = "A flake for developing Taconic Security Model tools";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    nixpkgs-unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      flake-utils,
      naersk,
      nixpkgs,
      nixpkgs-unstable,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        inherit (pkgs.stdenv) isDarwin;

        # this overlay gives us pre-built binaries for the rust
        # toolchain, so we have reproducible builds
        overlays = [ (import rust-overlay) ];

        pkgs = (import nixpkgs) { inherit system overlays; };
        pkgs-unstable = (import nixpkgs-unstable) { inherit system overlays; };

        myrust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-analyzer"
            "rust-src"
          ];
        };

        basedeps = [
          pkgs.pkg-config
          myrust
          pkgs.cargo-watch
          pkgs-unstable.nodePackages.mermaid-cli
          pkgs.nodePackages.prettier
          pkgs.pandoc
          pkgs.librsvg
          pkgs.ungoogled-chromium
        ];

      in
      with pkgs;
      {

        # For `nix build` & `nix run`:
        packages.default = naersk'.buildPackage {
          nativeBuildInputs = basedeps;
          src = ./.;
        };

        # For `nix develop`:
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = basedeps;
          shellHook = ''
            export PUPPETEER_EXECUTABLE_PATH="${pkgs.ungoogled-chromium}/bin/chromium"
          '';
        };
      }
    );
}
