# TODO: move this to nixpkgs
# This file aims to be an up-to-date replacement on master for the nixpkgs derivation.
{ lib
, pkg-config
, rustPlatform
, fetchFromGitHub
, stdenv
, buildPackages
, apple-sdk
, installShellFiles
, installShellCompletions ? stdenv.buildPlatform.canExecute stdenv.hostPlatform
, installManPages ? stdenv.buildPlatform.canExecute stdenv.hostPlatform
, buildNoDefaultFeatures ? false
, buildFeatures ? [ ]
, withNoDefaultFeatures ? buildNoDefaultFeatures
, withFeatures ? buildFeatures
,
} @ args:
let
  version = "0.0.10";
  hash = "sha256-Y9SuxqI8wvoF0+X6CLNDlSFCwlSU8R73NYF/LjACP18=";
  cargoHash = "sha256-1WJIIsTzbChWqvdBSR/OpLC1iR8FgLmypJFQEtpalbw=";
  noDefaultFeatures =
    lib.warnIf
      (args ? buildNoDefaultFeatures)
      "buildNoDefaultFeatures is deprecated in favour of withNoDefaultFeatures and will be removed in the next release"
      withNoDefaultFeatures;

  features =
    lib.warnIf
      (args ? buildFeatures)
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

  nativeBuildInputs =
    [ pkg-config ]
    ++ lib.optional (installManPages || installShellCompletions) installShellFiles;

  buildInputs = lib.optional stdenv.hostPlatform.isDarwin apple-sdk;

  doCheck = false;
  auditable = false;

  # unit tests only
  cargoTestFlags = [ "--lib" ];

  postInstall =
    let
      emulator = stdenv.hostPlatform.emulator buildPackages;
    in
    ''
      mkdir -p $out/share/{completions,man}
      ${emulator} "$out"/bin/comodoro man "$out"/share/man
      ${emulator} "$out"/bin/comodoro completion bash > "$out"/share/completions/comodoro.bash
      ${emulator} "$out"/bin/comodoro completion elvish > "$out"/share/completions/comodoro.elvish
      ${emulator} "$out"/bin/comodoro completion fish > "$out"/share/completions/comodoro.fish
      ${emulator} "$out"/bin/comodoro completion powershell > "$out"/share/completions/comodoro.powershell
      ${emulator} "$out"/bin/comodoro completion zsh > "$out"/share/completions/comodoro.zsh
    ''
    + lib.optionalString installManPages ''
      installManPage "$out"/share/man/*
    ''
    + lib.optionalString installShellCompletions ''
      installShellCompletion "$out"/share/completions/comodoro.{bash,fish,zsh}
    '';

  meta = with lib; {
    description = "CLI to manage emails";
    mainProgram = "comodoro";
    homepage = "https://github.com/pimalaya/comodoro";
    changelog = "https://github.com/pimalaya/comodoro/blob/v${version}/CHANGELOG.md";
    license = licenses.mit;
    maintainers = with maintainers; [ soywod ];
  };
}
