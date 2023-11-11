{
  description = "LambdaBuffers Cardano Demo";
  inputs = {
    lbf.url = "github:mlabs-haskell/lambda-buffers?ref=bladyjoker/small-fix-catastrophic";
    haskell-nix.follows = "lbf/haskell-nix";
    nixpkgs.follows = "lbf/nixpkgs";
    pre-commit-hooks.follows = "lbf/pre-commit-hooks";
    hci-effects.follows = "lbf/hci-effects";
    ctl.follows = "lbf/ctl";
    iohk-nix.follows = "lbf/iohk-nix";
    flake-parts.follows = "lbf/flake-parts";
    plutarch.follows = "lbf/plutarch";
    cardano-haskell-packages.url = "github:input-output-hk/cardano-haskell-packages?ref=repo";
    cardano-haskell-packages.flake = false;

  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        ./pkgs.nix
        ./settings.nix
        ./pre-commit.nix
        ./hercules-ci.nix
        api/build.nix
        ./validation/demo-plutarch/build.nix
        #        ./validation/demo-plutustx/build.nix
      ];
      debug = true;
      systems = [ "x86_64-linux" "x86_64-darwin" ];
    };
}
