{
  description = "d-ll-m: LLM-powered TTRPG platform";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    spacetimedb = {
      url = "github:clockworklabs/SpacetimeDB";
      # Don't follow nixpkgs to avoid breaking their build
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, spacetimedb }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # SpacetimeDB packages from their flake
        stdbPkgs = spacetimedb.packages.${system} or null;

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust
            rustToolchain
            cargo-watch
            cargo-edit

            # SpacetimeDB (if available for this system)
          ] ++ pkgs.lib.optionals (stdbPkgs != null) [
            stdbPkgs.spacetime
          ] ++ [
            # Build dependencies
            pkg-config
            openssl
            glib
            gtk3
            webkitgtk_4_1
            libsoup_3
            cairo
            pango
            gdk-pixbuf
            atk

            # Dev tools
            just

            # Profiling
            tracy

            # Wasm optimization (wasm-opt for SpacetimeDB modules)
            binaryen
          ];

          shellHook = ''
            echo "d-ll-m dev shell"
            echo "Rust: $(rustc --version)"
            ${if stdbPkgs != null then ''
              echo "SpacetimeDB: $(spacetime -V)"
            '' else ''
              echo "SpacetimeDB: not available for this system"
            ''}
          '';

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };
      }
    );
}
