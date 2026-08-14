# TODO: move this to nixpkgs
# This file aims to be a replacement for the nixpkgs derivation.

{
  buildFeatures ? [ ],
  buildNoDefaultFeatures ? false,
  buildPackages,
  dbus,
  fetchFromGitHub,
  installManPages ? stdenv.buildPlatform.canExecute stdenv.hostPlatform,
  installShellCompletions ? stdenv.buildPlatform.canExecute stdenv.hostPlatform,
  installShellFiles,
  lib,
  pkg-config,
  rustPlatform,
  stdenv,
}:

let
  notify = !buildNoDefaultFeatures || builtins.elem "notify" buildFeatures;
  dbus' =
    # undefined reference in _dbus_atomic_* functions
    if stdenv.hostPlatform.isLinux && stdenv.hostPlatform.isAarch64 then
      dbus.overrideAttrs (old: {
        env = (old.env or { }) // {
          NIX_CFLAGS_COMPILE = (old.env.NIX_CFLAGS_COMPILE or "") + " -mno-outline-atomics";
        };
      })
    else
      dbus;

in
rustPlatform.buildRustPackage (finalAttrs: {
  __structuredAttrs = true;

  inherit buildNoDefaultFeatures;

  pname = "comodoro";
  version = "2.0.0";
  cargoHash = "";

  src = fetchFromGitHub {
    owner = "pimalaya";
    repo = finalAttrs.pname;
    tag = "v${finalAttrs.version}";
    hash = "";
  };

  nativeBuildInputs = [
    pkg-config
    installShellFiles
  ];

  # On Windows, D-Bus is provided by vendors
  buildInputs = lib.optional (notify && !stdenv.hostPlatform.isWindows) dbus';

  buildFeatures =
    buildFeatures
    # On Windows, D-Bus is provided by vendors
    ++ lib.optional (notify && stdenv.hostPlatform.isWindows) "vendored";

  postInstall =
    let
      exe =
        if stdenv.buildPlatform.canExecute stdenv.hostPlatform then
          "$out/bin/${finalAttrs.pname}"
        else
          lib.getExe buildPackages.${finalAttrs.pname};
    in
    ''
      mkdir -p $out/share/{completions,man}
      ${exe} manuals "$out"/share/man
      ${exe} completions -d "$out"/share/completions bash elvish fish powershell zsh
    ''
    + lib.optionalString installManPages ''
      installManPage "$out"/share/man/*
    ''
    + lib.optionalString installShellCompletions ''
      installShellCompletion --cmd ${finalAttrs.pname} \
        --bash "$out"/share/completions/${finalAttrs.pname}.bash \
        --fish "$out"/share/completions/${finalAttrs.pname}.fish \
        --zsh "$out"/share/completions/_${finalAttrs.pname}
    '';

  # Disable impure integration tests: they bind sockets and spawn processes
  cargoTestFlags = [ "--bins" ];

  meta = {
    description = "CLI to manage timers";
    mainProgram = finalAttrs.pname;
    homepage = "https://github.com/pimalaya/${finalAttrs.pname}";
    changelog = "https://github.com/pimalaya/${finalAttrs.pname}/releases/${finalAttrs.src.tag}";
    license = with lib.licenses; [
      asl20
      mit
    ];
    maintainers = with lib.maintainers; [ soywod ];
  };
})
