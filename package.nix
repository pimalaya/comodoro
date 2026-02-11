# TODO: move this to nixpkgs
# This file aims to be a replacement for the nixpkgs derivation.

{
  lib,
  dbus,
  rustPlatform,
  fetchFromGitHub,
  stdenv,
  buildPackages,
  pkg-config,
  apple-sdk,
  installShellFiles,
  installShellCompletions ? stdenv.buildPlatform.canExecute stdenv.hostPlatform,
  installManPages ? stdenv.buildPlatform.canExecute stdenv.hostPlatform,
  buildNoDefaultFeatures ? false,
  buildFeatures ? [ ],
}:

let
  version = "1.0.0";
  hash = "";
  cargoHash = "";

in
rustPlatform.buildRustPackage rec {
  inherit
    cargoHash
    version
    buildNoDefaultFeatures
    buildFeatures
    ;

  pname = "comodoro";

  src = fetchFromGitHub {
    inherit hash;
    owner = "pimalaya";
    repo = "comodoro";
    rev = "v${version}";
  };

  useFetchCargoVendor = true;

  nativeBuildInputs = [
    pkg-config
  ]
  ++ lib.optional (installManPages || installShellCompletions) installShellFiles;

  buildInputs =
    [ ]
    ++ lib.optional stdenv.hostPlatform.isDarwin apple-sdk
    ++ lib.optional (!buildNoDefaultFeatures || builtins.elem "notify" buildFeatures) dbus;

  doCheck = false;

  postInstall =
    let
      emulator = stdenv.hostPlatform.emulator buildPackages;
      exe = stdenv.hostPlatform.extensions.executable;
    in
    lib.optionalString (lib.hasInfix "wine" emulator) ''
      export WINEPREFIX="''${WINEPREFIX:-$(mktemp -d)}"
      mkdir -p $WINEPREFIX
    ''
    + ''
      mkdir -p $out/share/{completions,man}
      ${emulator} "$out"/bin/comodoro${exe} manuals "$out"/share/man
      ${emulator} "$out"/bin/comodoro${exe} completions -d "$out"/share/completions bash elvish fish powershell zsh
    ''
    + lib.optionalString installManPages ''
      installManPage "$out"/share/man/*
    ''
    + lib.optionalString installShellCompletions ''
      installShellCompletion --bash "$out"/share/completions/comodoro.bash
      installShellCompletion --fish "$out"/share/completions/comodoro.fish
      installShellCompletion --zsh "$out"/share/completions/_comodoro
    '';

  meta = rec {
    description = "CLI to manage timers";
    mainProgram = "comodoro";
    homepage = "https://github.com/pimalaya/comodoro";
    changelog = "${homepage}/blob/v${version}/CHANGELOG.md";
    license = lib.licenses.agpl3Plus;
    maintainers = with lib.maintainers; [ soywod ];
  };
}
