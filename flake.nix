{
  description = "Building GitHub PRs in Hydra.";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1.533189.tar.gz";
  };

  outputs =
    { nixpkgs
    , self
    , ...
    }@inputs:
    let
      nameValuePair = name: value: { inherit name value; };
      genAttrs = names: f: builtins.listToAttrs (map (n: nameValuePair n (f n)) names);

      pkgsFor = pkgs: system:
        import pkgs { inherit system; };

      allSystems = [ "x86_64-linux" "aarch64-darwin" "aarch64-linux" "x86_64-darwin" ];
      forAllSystems = f: genAttrs allSystems
        (system: f {
          inherit system;
          pkgs = pkgsFor nixpkgs system;
        });
    in
    {
      pkgs = forAllSystems ({ pkgs, ... }: pkgs);

      generator = forAllSystems
        ({ pkgs, ... }: import ./jobset-generator { inherit pkgs; });

      devShell = forAllSystems
        ({ pkgs, ... }:
          pkgs.mkShell {
            buildInputs = with pkgs;
              [
                cargo
                clippy
                codespell
                jq
                nixpkgs-fmt
                rustfmt
                shellcheck
                (terraform_1.withPlugins (p: [
                  p.hydra
                ]))
              ];
          }
        );
    };
}
