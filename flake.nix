{
  description = "CLI to manage timers";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";
    gitignore = {
      url = "github:hercules-ci/gitignore.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      # https://github.com/nix-community/fenix/pull/145
      # url = "github:nix-community/fenix";
      url = "github:soywod/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, gitignore, fenix, naersk, ... }:
    let
      inherit (nixpkgs) lib;
      inherit (gitignore.lib) gitignoreSource;

      crossSystems = {
        x86_64-linux = {
          x86_64-linux = {
            rustTarget = "x86_64-unknown-linux-musl";
          };

          aarch64-linux = rec {
            rustTarget = "aarch64-unknown-linux-musl";
            runner = { pkgs, comodoro }: "${pkgs.qemu}/bin/qemu-aarch64 ${comodoro}";
            mkPackage = { system, ... }: package:
              let
                inherit (mkPkgsCross system rustTarget) stdenv;
                cc = "${stdenv.cc}/bin/${stdenv.cc.targetPrefix}cc";
              in
              package // {
                TARGET_CC = cc;
                CARGO_BUILD_RUSTFLAGS = package.CARGO_BUILD_RUSTFLAGS ++ [ "-Clinker=${cc}" ];
              };
          };

          x86_64-windows = {
            rustTarget = "x86_64-pc-windows-gnu";
            runner = { pkgs, comodoro }:
              let wine = pkgs.wine.override { wineBuild = "wine64"; };
              in "${wine}/bin/wine64 ${comodoro}.exe";
            mkPackage = { pkgs, ... }: package:
              let
                inherit (pkgs.pkgsCross.mingwW64) stdenv windows;
                cc = "${stdenv.cc}/bin/${stdenv.cc.targetPrefix}cc";
              in
              package // {
                depsBuildBuild = [ stdenv.cc windows.pthreads ];
                TARGET_CC = cc;
                CARGO_BUILD_RUSTFLAGS = package.CARGO_BUILD_RUSTFLAGS ++ [ "-Clinker=${cc}" ];
              };
          };
        };

        aarch64-linux.aarch64-linux = {
          rustTarget = "aarch64-unknown-linux-musl";
        };

        x86_64-darwin.x86_64-darwin = {
          rustTarget = "x86_64-apple-darwin";
          mkPackage = { pkgs, ... }: package:
            let inherit (pkgs.darwin.apple_sdk.frameworks) Cocoa;
              inherit (pkgs.darwin.apple_sdk_11_0.frameworks) AppKit Security;
            in
            package // {
              buildInputs = [ Cocoa ];
              NIX_LDFLAGS = [
                "-F${AppKit}/Library/Frameworks"
                "-framework AppKit"
                "-F${Security}/Library/Frameworks"
                "-framework Security"
              ];
            };
        };

        aarch64-darwin.aarch64-darwin = {
          rustTarget = "aarch64-apple-darwin";
          mkPackage = { pkgs, ... }: package:
            let inherit (pkgs.darwin.apple_sdk.frameworks) Cocoa;
            in package // {
              buildInputs = [ Cocoa ];
            };
        };
      };

      eachBuildSystem = lib.genAttrs (builtins.attrNames crossSystems);

      mkPkgsCross = buildSystem: crossSystem: import nixpkgs {
        system = buildSystem;
        crossSystem.config = crossSystem;
      };

      mkToolchain = import ./rust-toolchain.nix fenix;

      mkApp = { pkgs, buildSystem, targetSystem ? buildSystem }:
        let
          comodoro = lib.getExe self.packages.${buildSystem}.${targetSystem};
          wrapper = crossSystems.${buildSystem}.${targetSystem}.runner or (_: comodoro) { inherit pkgs comodoro; };
          program = lib.getExe (pkgs.writeShellScriptBin "comodoro" "${wrapper} $@");
          app = { inherit program; type = "app"; };
        in
        app;

      mkApps = buildSystem:
        let
          pkgs = import nixpkgs { system = buildSystem; };
          mkApp' = targetSystem: _: mkApp { inherit pkgs buildSystem targetSystem; };
          defaultApp = mkApp { inherit pkgs buildSystem; };
          apps = builtins.mapAttrs mkApp' crossSystems.${buildSystem};
        in
        apps // { default = defaultApp; };

      mkPackage = { pkgs, buildSystem, targetSystem ? buildSystem }:
        let
          targetConfig = crossSystems.${buildSystem}.${targetSystem};
          toolchain = mkToolchain.fromTarget {
            inherit pkgs buildSystem;
            targetSystem = targetConfig.rustTarget;
          };
          rust = naersk.lib.${buildSystem}.override {
            cargo = toolchain;
            rustc = toolchain;
          };
          mkPackage' = targetConfig.mkPackage or (_: p: p);
          comodoro = "./comodoro";
          runner = targetConfig.runner or (_: comodoro) { inherit pkgs comodoro; };
          package = mkPackage' { inherit pkgs; system = buildSystem; } {
            name = "comodoro";
            src = gitignoreSource ./.;
            strictDeps = true;
            doCheck = false;
            auditable = false;
            nativeBuildInputs = with pkgs; [ pkg-config ];
            CARGO_BUILD_TARGET = targetConfig.rustTarget;
            CARGO_BUILD_RUSTFLAGS = [ "-Ctarget-feature=+crt-static" ];
            postInstall = ''
              export WINEPREFIX="$(mktemp -d)"

              mkdir -p $out/bin/share/{completions,man}

              cd $out/bin
              ${runner} man ./share/man
              ${runner} completion bash > ./share/completions/comodoro.bash
              ${runner} completion elvish > ./share/completions/comodoro.elvish
              ${runner} completion fish > ./share/completions/comodoro.fish
              ${runner} completion powershell > ./share/completions/comodoro.powershell
              ${runner} completion zsh > ./share/completions/comodoro.zsh

              tar -czf comodoro.tgz comodoro* share
              mv comodoro.tgz ../

              ${pkgs.zip}/bin/zip -r comodoro.zip comodoro* share
              mv comodoro.zip ../
            '';
          };
        in
        rust.buildPackage package;

      mkPackages = buildSystem:
        let
          pkgs = import nixpkgs { system = buildSystem; };
          mkPackage' = targetSystem: _: mkPackage { inherit pkgs buildSystem targetSystem; };
          defaultPackage = mkPackage { inherit pkgs buildSystem; };
          packages = builtins.mapAttrs mkPackage' crossSystems.${buildSystem};
        in
        packages // { default = defaultPackage; };

      mkDevShells = buildSystem:
        let
          pkgs = import nixpkgs { system = buildSystem; };
          rust-toolchain = mkToolchain.fromFile { inherit buildSystem; };
          defaultShell = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [ pkg-config ];
            buildInputs = with pkgs; [
              # Nix
              nixd
              nixpkgs-fmt

              # Rust
              rust-toolchain
              cargo-watch

              # Email env
              gnupg
              gpgme
              msmtp
              notmuch
            ];
          };
        in
        { default = defaultShell; };

    in
    {
      apps = eachBuildSystem mkApps;
      packages = eachBuildSystem mkPackages;
      devShells = eachBuildSystem mkDevShells;
    };
}
