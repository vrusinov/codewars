# SPDX-FileCopyrightText: 2025 Vladimir Rusinov
# SPDX-License-Identifier: Apache-2.0
{
  description = "Codewars";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

          # Use a plain mkShell dev environment (not FHS). The FHS environment
          # was causing host /lib OpenSSL to be visible to helper binaries
          # (git-remote-https/curl) which created ABI mismatches. A regular
          # mkShell ensures tools are linked to the Nix store libraries.
          devShell = pkgs.mkShell {
            name = "dev-shell";
            buildInputs = with pkgs; [
              zlib
              go_1_24
              nodejs_24  # For pre-commit hooks
              # Provide a system Ruby 3.3 so rbenv does not need to compile
              # Ruby 3.3 during pre-commit installs.
              pkgs.ruby_3_3
              # do not include rbenv; allow pre-commit to use the
              # system Ruby (ruby_3_3) instead of attempting to compile
              # Ruby via rbenv/ruby-build inside the dev shell.
              cargo
              zsh
              gcc
              pkg-config
              libffi
              openssl_3
              git
              curl
              cacert
              # Ruby build dependencies (help rbenv/ruby-build compile extensions)
              libyaml
              gdbm
              readline
              bzip2
              # C++ runtime needed by some pre-commit python wheels (re2)
              pkgs.stdenv.cc.libcxx
              python312
              # (pre-commit is intentionally left to be installed by the user
              # or invoked via `nix run` if desired)
              # Provide the pre-commit binary from nix to avoid using pip
              # inside the shell (works around PEP 668 restrictions).
              pre-commit
            ];
            shellHook = ''
              # Ensure OpenSSL and CA certs from Nix are used by tools that
              # consult the environment (e.g. curl, git).
              export SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt
              export REQUESTS_CA_BUNDLE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt
              export GIT_SSL_CAINFO=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt
              # Make the Nix-provided C++ runtime visible to Python virtual
              # environments created by pre-commit (fixes ImportError:
              # libstdc++.so.6 not found for binary extensions like re2).
              ${if pkgs.stdenv.cc.libcxx != null then "export LD_LIBRARY_PATH=${pkgs.stdenv.cc.libcxx}/lib:$LD_LIBRARY_PATH\n" else ""}
              # Ensure the GCC lib dir used by the dev-shell is visible to
              # subprocesses (this locates the libstdc++.so.6 that gcc knows
              # about and prepends its directory to LD_LIBRARY_PATH).
              export LD_LIBRARY_PATH="$(gcc -print-file-name=libstdc++.so.6 2>/dev/null | xargs dirname 2>/dev/null):$LD_LIBRARY_PATH"
            '';
          };
      in
      {
        devShells.default = devShell;
      });
}
