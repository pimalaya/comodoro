{
  pimalaya ? import (fetchTarball "https://github.com/pimalaya/nix/archive/master.tar.gz"),
  ...
}@args:

let
  comodoro = import ./default.nix (
    removeAttrs args [
      "crossPkgs"
      "isStatic"
      "target"
    ]
  );

in
pimalaya.mkDefault (
  {
    src = ./.;
    version = "2.0.0";
    mkPackage = (
      {
        lib,
        pkgs,
        rustPlatform,
        defaultFeatures,
        features,
        buildPackages,
      }:

      pkgs.callPackage ./package.nix {
        inherit lib rustPlatform;
        buildPackages = buildPackages // {
          inherit comodoro;
        };
        installShellCompletions = false;
        installManPages = false;
        buildNoDefaultFeatures = !defaultFeatures;
        buildFeatures = lib.splitString "," features;
      }
    );
  }
  // removeAttrs args [ "pimalaya" ]
)
