{
  description = "a small git utility in bash.";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  outputs = {
    self,
    nixpkgs,
  }: {
    defaultPackage.x86_64-linux = with import nixpkgs {system = "x86_64-linux";};
      stdenv.mkDerivation {
        name = "gtea";
        src = self;
        buildPhase = "make clean all";
        installPhase = "mkdir -p $out/bin; PREFIX=$out make install";
      };

    formatter.x86_64-linux = nixpkgs.legacyPackages.x86_64-linux.alejandra;
  };
}
