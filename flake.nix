# SPDX-FileCopyrightText: 2025 Vladimir Rusinov
# SPDX-License-Identifier: Apache-2.0
{
  description = "Codewars";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        fhs = pkgs.buildFHSEnv {
          name = "fhs-shell";
          targetPkgs = pkgs: [
            pkgs.git
            pkgs.go_1_23
            pkgs.nodejs_24  # For pre-commit hooks
            pkgs.ruby_3_4  # For pre-commit hooks
            pkgs.cargo
            pkgs.zsh
          ];
        };
      in
      {
        devShells.default = fhs.env;
      });
}
