{
  description = "Clean unused files from qbittorrent";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
let
  pkgs = import nixpkgs { system = "x86_64-linux"; };
in {

    packages.x86_64-linux.clean-unused-files = pkgs.callPackage ./pkg.nix {};

    nixosModules.clean-unused-files = import ./module.nix self;

    hydraJobs = {
      inherit (self) packages;
    };
    
  };
}
