{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    flake-utils,
    rust-overlay,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import inputs.nixpkgs {
          inherit system overlays;
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            libllvm
            (rust-bin.stable.latest.default.override {
              extensions = ["rust-src" "rust-analyzer"];
            })
          ];
        };
      }
    );
}
