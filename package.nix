# TODO: move this to nixpkgs
# This file aims to be a replacement for the nixpkgs derivation.

{
  lib,
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
  withNoDefaultFeatures ? buildNoDefaultFeatures,
  withFeatures ? buildFeatures,
}@args:

let
  version = "1.0.0";
  hash = "";
  cargoHash = "";

  noDefaultFeatures =
    lib.warnIf (args ? buildNoDefaultFeatures)
      "buildNoDefaultFeatures is deprecated in favour of withNoDefaultFeatures and will be removed in the next release"
      withNoDefaultFeatures;

  features =
    lib.warnIf (args ? buildFeatures)
      "buildFeatures is deprecated in favour of withFeatures and will be removed in the next release"
      withFeatures;
in

rustPlatform.buildRustPackage rec {
  inherit cargoHash version;

  pname = "comodoro";

  src = fetchFromGitHub {
    inherit hash;
    owner = "pimalaya";
    repo = "comodoro";
    rev = "v${version}";
  };

  useFetchCargoVendor = true;

  buildNoDefaultFeatures = noDefaultFeatures;
  buildFeatures = features;

  nativeBuildInputs = [
    pkg-config
  ] ++ lib.optional (installManPages || installShellCompletions) installShellFiles;

  buildInputs = lib.optional stdenv.hostPlatform.isDarwin apple-sdk;

  doCheck = false;

  postInstall =
    let
      emulator = stdenv.hostPlatform.emulator buildPackages;
    in
    ''
      mkdir -p $out/share/{completions,man}
      ${emulator} "$out"/bin/comodoro manuals "$out"/share/man
      ${emulator} "$out"/bin/comodoro completions -d "$out"/share/completions bash elvish fish powershell zsh
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
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [ soywod ];
  };
}
