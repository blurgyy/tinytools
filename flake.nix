{
  description = "A collection of tools that enhance your experience in shell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  nixConfig = {
    extra-substituters = [ "https://cache.blurgy.xyz" ];
    trusted-public-keys = [ "cache.blurgy.xyz:Xg9PvXkUIAhDIsdn/NOUUFo+HHc8htSiGj7O6fUj/W4=" ];
  };

  outputs = { self, nixpkgs, flake-utils, ... }: flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system: let
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ self.overlays.default ];
    };
  in {
    packages = rec {
      inherit (pkgs) tinytools;
      default = tinytools;
    };
  }) // {
    overlays.default = let
      version = "1.1.2";
    in final: prev: {
      tinytools = final.callPackage ./. { inherit version; };
    };
    hydraJobs = self.packages;
  };
}
