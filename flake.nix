{
  description = "Vow dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
        with pkgs; {
          devShells.default = mkShell {
            buildInputs = [
              # (rust-bin.stable.latest.default.override {extensions = ["rust-src"];})
              (rust-bin.selectLatestNightlyWith (toolchain:
                toolchain.default.override {
                  extensions = ["rust-src"];
                }))
              cargo-feature
            ];
          };
        }
    );
}
