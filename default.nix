let
  nixpkgs = (import <nixpkgs> { }).fetchFromGitHub {
    owner  = "NixOS";
    repo   = "nixpkgs";
    rev    = "3516b2dad23c59bc66cf2b67d8df45e76fc81f61";
    sha256 = "1xqj8prppf2vnbgklfwm76162z1i8jraivw593vb87hq67lgc4s4";
  };

  mozilla-overlay =
    let
      src = builtins.fetchTarball {
        url = https://github.com/themoritz/nixpkgs-mozilla/archive/1.36.0-nightly.tar.gz;
        sha256 = "07b7hgq5awhddcii88y43d38lncqq9c8b2px4p93r5l7z0phv89d";
      };
    in
      import src;

  pkgs = import nixpkgs { overlays = [ mozilla-overlay ]; };

  nightly-channel = pkgs.rustChannelOf {
    channel = "nightly";
    date = "2019-06-17";
  };

  # Add the rust sources so that racer works properly in the nix shell (if we
  # can get it to compile...)
  rust-with-src =
    nightly-channel.rust.override {
      extensions = ["rust-src"];
    };

  rustPlatform =
    pkgs.makeRustPlatform {
      cargo = nightly-channel.cargo;
      rustc = rust-with-src;
    };

  sat-stuff = pkgs.stdenv.mkDerivation {
    name = "sat-stuff";
    nativeBuildInputs = [ rustPlatform.rust.rustc rustPlatform.rust.cargo ];
  };
in
  sat-stuff
