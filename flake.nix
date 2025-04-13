{
  description = "ETF is a cli tool to help you eat the frog";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs, ... }:
    let
      system = "x86_64-linux";
    in
    {
      devShells.${system}.default =
        let
          pkgs = import nixpkgs {
            inherit system;
          };
        in
        pkgs.mkShell {
          packages = with pkgs;  [
            cargo
            rustc
            pkg-config
            openssl
          ];
          shellHook = ''
            echo "Developing with Rust!"
          '';
        };


      # packages.${system}.default =
      #   let
      #     pkgs = import nixpkgs {
      #       inherit system;
      #     };
      #   in
      #   pkgs.rustPlatform.buildRustPackage
      #     rec {
      #       pname = "etf";
      #       version = "0.1.0";
      #
      #       src = pkgs.fetchFromGitHub {
      #         owner = "NorthboundPaddler";
      #         repo = "etf";
      #         rev = "v0.";
      #         sha256 = "";
      #       };
      #
      #       cargoLock = {
      #         lockFile = ./Cargo.lock;
      #       };
      #
      #       nativeBuildInputs = [ pkgs.pkg-config ];
      #       buildInputs = [ pkgs.openssl ];
      #
      #       meta = with pkgs.lib; {
      #         description = "";
      #         homepage = "https://github.com/NorthboundPaddler/etf";
      #         license = licenses.gpl3Only; # Replace with the actual license
      #         maintainers = [ maintainers.NorthboundPaddler ];
      #       };
      #     };
    };
}
