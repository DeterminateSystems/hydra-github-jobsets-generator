{ nixpkgs, generator, pull_requests, generator_config } @ args:
let
  pkgs = import nixpkgs { };
  jobset_generator = import (generator + "/jobset-generator") { inherit pkgs; };
in
{
  jobsets =
    pkgs.runCommand "jobsets.json"
      {
        buildInputs = [ jobset_generator ];
        template = generator_config + "/.hydra/config.json";
      } ''
      jobset-generator ${pull_requests} "$template"  > $out
    '';
}
