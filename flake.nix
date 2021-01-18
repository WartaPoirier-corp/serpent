{
  description = "SSSS";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
  with import nixpkgs { system = "x86_64-linux"; };
  {
    packages.x86_64-linux.serpent = nixpkgs.buildRustCrate {
      crateName = "serpent";
      src = ./.;
    };

    defaultPackages.x86_64-linux = self.packages.x86_64-linux.serpent;

    devShell.x86_64-linux = pkgs.mkShell {
      name = "serpent";
      buildInputs = with pkgs; [
        alsaLib
        glslang
        vulkan-loader
        x11
        httplz
        wasm-bindgen-cli
        gnumake
      ];
      nativeBuildInputs = [ pkgs.pkg-config ];
    };
  };
}
