self:
{ lib, config, pkgs, }:
let
  inherit (lib) mkOption types mkEnableOption mkIf;

  cfg = config.services.strichliste-rs;

  mkSubmoduleOption = sub-cfg:
    mkOption {
      default = { };
      type = types.submodule { options = sub-cfg; };
    };
in {
  options.services.strichliste-rs = {
    enable = mkEnableOption "enable strichliste-rs service";

    package = mkOption {
      type = types.package;
      default = self.packages.${config.pkgs.system}.default;
    };

    address = mkOption {
      type = types.str;
      description = "The address that strichliste-rs is going to listen on";
    };

    port = mkOption {
      type = types.port;
      description = "The port that strichliste-rs is going to listen on";
    };

    dataDir = mkOption { type = types.path; };
  };

  config = mkIf cfg.enable {
    systemd.services."strichliste-rs" = {
      description = "Strichliste-rs: A digital tally sheet";
      wantedBy = [ "multi-user.target" ];
      serviceConfig = {
        ExecStart = "${cfg.package}/bin/strichliste-rs -d ${cfg.dataDir}";
        Restart = "on-failure";

        # might need to explicitely add the users
        User = "strichliste-rs";
        Group = "strichliste-rs";
      };
    };
  };
}
