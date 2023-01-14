{ pkgs, nixpkgs, system, makeRustPlatform, rust-overlay }:
let
  rustPkgs = import nixpkgs {
    inherit system;
    overlays = [ (import rust-overlay) ];
  };

  rustVersion = "1.68.0";
  wasmUnknownUknown = "wasm32-unknown-unknown";
  wasmWasi = "wasm32-wasi";

  rustWithWasmTarget = rustPkgs.rust-bin.default.${rustVersion}.default.override {
    targets = [ wasmUnknownUknown ];
  };

  rustPlatformWasm = makeRustPlatform {
    cargo = rustWithWasmTarget;
    rustc = rustWithWasmTarget;
  };

  common = {
    version = "0.1.0";
    src = ./.;

    cargoLock = {
      lockFile = ./Cargo.lock;
    };

    nativeBuildInputs = [ pkgs.pkg-config ];
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  };
in {
  wasm = rustPlatformWasm.buildRustPackage (common // {
    pname = "pzzld";

    buildPhase = ''
      trunk build --release
    '';  
    installPhase = ''
      mkdir -p $out/lib
      cp dist/ $out/lib/
    '';  
  });
}