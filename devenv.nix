{ pkgs, lib, config, inputs, ... }: {
  config = {
    languages.rust = {
      enable = true;
      channel = "stable";
    };
  };
}
