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
        template = src + "/.hydra/inputs.json";
      } ''
      jobset_generator --template $template ${pull_requests} > $out
    '';
}
