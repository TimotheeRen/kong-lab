{
  pkgs,
  ...
}:

{
  env.GREET = "devenv";

  packages = with pkgs; [ 
    k3d
    kubernetes-helm
    fluxcd
    kubectl
    k9s
    protobuf
    grpcurl
    telepresence2
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
    components = ["cargo" "rust-analyzer" "rust-src" "rustfmt" "rustc"];
  };
}
