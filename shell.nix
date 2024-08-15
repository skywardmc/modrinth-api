{
  pkgs ? import nixpkgs {
    inherit system;
    config = { };
    overlays = [ ];
  },
  nixpkgs ? <nixpkgs>,
  system ? builtins.currentSystem,
}:
pkgs.mkShell {
  packages = [
    # rust
    pkgs.rustc
    pkgs.cargo
    pkgs.clippy
    pkgs.rust-analyzer

    # nix
    pkgs.nil
    pkgs.nixfmt-rfc-style
    pkgs.statix

    # repo tools
    pkgs.actionlint
  ];

  env = {
    RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
  };
}
