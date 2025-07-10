{
  description = "Flake for Dorippe, a GTK file manager written in rust.";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }:
    let
      allSystems = nixpkgs.lib.genAttrs nixpkgs.lib.platforms.all;

      toSystems = passPkgs: allSystems (system:
        passPkgs (import nixpkgs { inherit system; })
      );
    in
    {
      devShells = toSystems (pkgs: {
        default = pkgs.mkShell {
          name = "Dorippe";

          nativeBuildInputs = with pkgs.buildPackages; [
            pkg-config
            rustc
            cargo
            rustfmt
            clippy
          ];

          buildInputs = with pkgs.buildPackages; [
            dbus
            glib
            pango
            gdk-pixbuf
            graphene
            gtk4
          ];
        };
      });
    };
}
