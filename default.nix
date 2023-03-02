{ lib, rustPlatform
, version
}: rustPlatform.buildRustPackage {
  pname = "tinytools";
  inherit version;
  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;

  shellHook = ''
    [[ "$-" == *i* ]] && exec "$SHELL"
  '';

  meta = {
    homepage = "https://github.com/blurgyy/tinytools";
    description = "A collection of tools that enhance your experience in shell";
    license = lib.licenses.mit;
  };
}
