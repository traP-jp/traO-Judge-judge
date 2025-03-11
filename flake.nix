{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        aarch64-pkgs = import nixpkgs {
          inherit system;
          crossSystem = {
            system = "aarch64-linux";
          };
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [
            pkgs.protobuf_27
          ];
        };
        formatter = pkgs.nixfmt-rfc-style;
        packages.aarch64-crossPlatform =
          with aarch64-pkgs;
          rustPlatform.buildRustPackage {
            name = "traO-Judge-judge";
            src = ./.;
            hostPlatform = "aarch64-linux";
            buildPlatform = system;
            cargoLock = {
              lockFile = ./Cargo.lock;
              outputHashes = {
                "async-sqlx-session-0.4.0" = "sha256-/iNCvfyqJP72z7TXj0p+epGCLYNClVAxWTcpGJ5mrmc=";
              };
            };
            env = {
              OPENSSL_DIR = "${openssl_3.dev}";
              OPENSSL_LIB_DIR = "${openssl_3.out}/lib";
              OPENSSL_INCLUDE_DIR = "${openssl_3.dev}/include";
              CODEGEN_SKIP = "true";
            };
          };
      }
    );
}
