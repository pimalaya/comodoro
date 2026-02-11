{
  pimalaya ? import (fetchTarball "https://github.com/pimalaya/nix/archive/master.tar.gz"),
  ...
}@args:

pimalaya.mkShell (
  builtins.removeAttrs args [ "pimalaya" ]
  // {
    extraBuildInputs = "nixd,nixfmt-rfc-style,dbus";
  }
)
