{ nixpkgs, generator, pull_requests, src } @ args:
let
  pkgs = import nixpkgs { };
  jobset_generator = import (generator + "/jobset-generator") { inherit pkgs; };
in
{
  jobsets =
    pkgs.runCommand "jobsets.json"
      {
        buildInputs = [ jobset_generator ];
        template = src + "/.hydra/config.json";
      } ''
      jobset_generator ${pull_requests} "$template"  > $out
    '';
}
