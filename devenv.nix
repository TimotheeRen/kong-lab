{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  env.GREET = "devenv";

  packages = [ 
    pkgs.k3d 
    pkgs.kubernetes-helm
    pkgs.fluxcd
    pkgs.kubectl
    pkgs.k9s
    pkgs.protobuf
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
    components = ["cargo" "rust-analyzer" "rust-src" "rustfmt" "rustc"];
  };
}
