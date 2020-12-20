{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "snake";
  buildInputs = with pkgs; [
    alsaLib
    glslang
    vulkan-loader
    x11
    httplz
    # i guess there is a better way to select a custom crate version…
    (rustPlatform.buildRustPackage rec {
      pname = "wasm-bindgen-cli";
      version = "0.2.69";

      src =
        let
          tarball = fetchFromGitHub {
            owner = "rustwasm";
            repo = "wasm-bindgen";
            rev = version;
            sha256 = "1psylk3hlx0ahwib3ph8qdk0jwlp8qzc6dv61002rj7ns28vs4mx";
          };
        in runCommand "source" { } ''
          cp -R ${tarball} $out
          chmod -R +w $out
          cp ${./wasm-bindgen-Cargo.lock} $out/Cargo.lock
        '';

      buildInputs = [ openssl ];
      nativeBuildInputs = [ pkgconfig ];

      cargoSha256 = "1i2bqp259z2hcvxfrpk79bmvzvdzsaz7iiz6884icgc8zs5wsans";
      cargoBuildFlags = [ "-p" pname ];
    })
    gnumake
  ];
  nativeBuildInputs = [ pkgs.pkg-config ];
}
