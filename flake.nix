{
  description = "CLI to manage your time.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    gitignore = {
      url = "github:hercules-ci/gitignore.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, gitignore, fenix, naersk }:
    let
      inherit (gitignore.lib) gitignoreSource;

      mkToolchain = buildPlatform:
        fenix.packages.${buildPlatform}.minimal.toolchain;

      mkToolchainWithTarget = buildPlatform: targetPlatform:
        with fenix.packages.${buildPlatform}; combine [
          minimal.rustc
          minimal.cargo
          targets.${targetPlatform}.latest.rust-std
        ];

      mkDevShells = buildPlatform:
        let
          pkgs = import nixpkgs { system = buildPlatform; };
          toolchain = fenix.packages.${buildPlatform}.default.withComponents [
            "cargo"
            "clippy"
            "rustc"
            "rustfmt"
          ];
        in
        {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              # Nix env
              rnix-lsp
              nixpkgs-fmt

              # Rust env
              toolchain
              rust-analyzer
            ];
          };
        };

      mkPackage = pkgs: buildPlatform: targetPlatform: package:
        let
          toolchain =
            if isNull targetPlatform
            then mkToolchain buildPlatform
            else mkToolchainWithTarget buildPlatform targetPlatform;
          naersk' = naersk.lib.${buildPlatform}.override {
            cargo = toolchain;
            rustc = toolchain;
          };
          package' = {
            name = "comodoro";
            src = gitignoreSource ./.;
          } // pkgs.lib.optionalAttrs (!isNull targetPlatform) {
            CARGO_BUILD_TARGET = targetPlatform;
          } // package;
        in
        naersk'.buildPackage package';

      mkPackages = buildPlatform:
        let
          pkgs = import nixpkgs { system = buildPlatform; };
          defaultPackage = mkPackage pkgs buildPlatform null { };
          mkPackageWithTarget = mkPackage pkgs buildPlatform;
        in
        {
          default = defaultPackage;
          linux = defaultPackage;
          macos = defaultPackage;
          musl = mkPackageWithTarget "x86_64-unknown-linux-musl" {
            CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
          };
          windows = mkPackageWithTarget "x86_64-pc-windows-gnu" {
            strictDeps = true;
            depsBuildBuild = with pkgs.pkgsCross.mingwW64; [
              stdenv.cc
              windows.pthreads
            ];
          };
        };

      mkApp = drv: flake-utils.lib.mkApp {
        inherit drv;
        name = "comodoro";
      };

      mkApps = buildPlatform: {
        default = mkApp self.packages.${buildPlatform}.default;
        linux = mkApp self.packages.${buildPlatform}.linux;
        macos = mkApp self.packages.${buildPlatform}.macos;
        musl = mkApp self.packages.${buildPlatform}.musl;
        windows =
          let
            pkgs = import nixpkgs { system = buildPlatform; };
            wine = pkgs.wine.override { wineBuild = "wine64"; };
            comodoro = self.packages.${buildPlatform}.windows;
            app = pkgs.writeShellScriptBin "comodoro" ''
              export WINEPREFIX="$(mktemp -d)"
              ${wine}/bin/wine64 ${comodoro}/bin/comodoro.exe $@
            '';
          in
          mkApp app;
      };

    in
    flake-utils.lib.eachDefaultSystem (system: {
      devShells = mkDevShells system;
      packages = mkPackages system;
      apps = mkApps system;
    });
}
