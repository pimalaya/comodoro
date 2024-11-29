# TODO: move this to nixpkgs
# This file aims to be a replacement for the nixpkgs derivation.

{ lib
, pkg-config
, rustPlatform
, fetchFromGitHub
, stdenv
, apple-sdk
, installShellFiles
, installShellCompletions ? stdenv.buildPlatform.canExecute stdenv.hostPlatform
, installManPages ? stdenv.buildPlatform.canExecute stdenv.hostPlatform
, buildNoDefaultFeatures ? false
, buildFeatures ? [ ]
}:

let
  version = "0.0.10";
  hash = "sha256-Y9SuxqI8wvoF0+X6CLNDlSFCwlSU8R73NYF/LjACP18=";
  cargoHash = "sha256-1WJIIsTzbChWqvdBSR/OpLC1iR8FgLmypJFQEtpalbw=";
in

rustPlatform.buildRustPackage rec {
  inherit cargoHash version;
  inherit buildNoDefaultFeatures buildFeatures;

  pname = "comodoro";

  src = fetchFromGitHub {
    inherit hash;
    owner = "pimalaya";
    repo = "comodoro";
    rev = "v${version}";
  };

  nativeBuildInputs = [ pkg-config ]
    ++ lib.optional (installManPages || installShellCompletions) installShellFiles;

  buildInputs = lib.optional stdenv.hostPlatform.isDarwin apple-sdk;

  doCheck = false;
  auditable = false;

  # unit tests only
  cargoTestFlags = [ "--lib" ];

  postInstall = ''
    mkdir -p $out/share/{completions,man}
  '' + lib.optionalString (stdenv.buildPlatform.canExecute stdenv.hostPlatform) ''
    "$out"/bin/comodoro man "$out"/share/man
  '' + lib.optionalString installManPages ''
    installManPage "$out"/share/man/*
  '' + lib.optionalString (stdenv.buildPlatform.canExecute stdenv.hostPlatform) ''
    "$out"/bin/comodoro completion bash > "$out"/share/completions/comodoro.bash
    "$out"/bin/comodoro completion elvish > "$out"/share/completions/comodoro.elvish
    "$out"/bin/comodoro completion fish > "$out"/share/completions/comodoro.fish
    "$out"/bin/comodoro completion powershell > "$out"/share/completions/comodoro.powershell
    "$out"/bin/comodoro completion zsh > "$out"/share/completions/comodoro.zsh
  '' + lib.optionalString installShellCompletions ''
    installShellCompletion "$out"/share/completions/comodoro.{bash,fish,zsh}
  '';

  meta = rec {
    description = "CLI to manage emails";
    mainProgram = "comodoro";
    homepage = "https://github.com/pimalaya/comodoro";
    changelog = "${homepage}/blob/v${version}/CHANGELOG.md";
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [ soywod ];
  };
}
