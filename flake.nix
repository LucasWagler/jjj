{
  description = "Description for the project";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      perSystem =
        {
          config,
          self',
          inputs',
          pkgs,
          system,
          ...
        }:
        {
          packages.default = pkgs.rustPlatform.buildRustPackage {
            pname = "jjj";
            version = "0.0.0";

            src = ./.;

            useFetchCargoVendor = true;
            cargoHash = "sha256-SLf6ZJhtg13M2rh04iRJ2NKD+6Iiv4iMlLEmIcCh+HQ=";
          };
          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              rustc
              cargo
              rust-analyzer
              lldb
            ];
          };
        };
    };
}
