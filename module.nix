self: system:
{ lib, config, pkgs, ... }:
let
  inherit (lib) mkOption types mkEnableOption mkIf;

  cfg = config.services.strichliste-rs;

  mkSubmoduleOption = sub-cfg:
    mkOption {
      default = { };
      type = types.submodule { options = sub-cfg; };
    };

  mkSoundListOption = options:
    mkOption { type = types.listOf types.path; } // options;

  statedirDefaultDir = "/var/lib/strichliste-rs";
in {
  options.services.strichliste-rs = {
    enable = mkEnableOption "enable strichliste-rs service";

    package = mkOption {
      type = types.package;
      default = self.packages.${system}.default;
    };

    address = mkOption {
      type = types.str;
      description = "The address that strichliste-rs is going to listen on";
    };

    port = mkOption {
      type = types.port;
      description = "The port that strichliste-rs is going to listen on";
    };

    dataDir = mkOption {
      type = types.path;
      default = statedirDefaultDir;
      description =
        "The data directory. If not default, permissions must be granted manually.";
    };

    settings = mkSubmoduleOption {
      accounts = mkSubmoduleOption {
        upper_limit = mkOption {
          type = types.ints.unsigned;
          description =
            "The upper account limit in cents. If set to 0, it will be disabled.";
        };

        lower_limit = mkOption {
          type = types.int // { check = (value: value <= 0); };
          description = "The lower account limit in cents. Must be <= 0.";
        };
      };

      sounds = mkSubmoduleOption {
        failed = mkSoundListOption {
          description = "Sounds that play when a transaction fails";
          default = [ ./public/sounds/wobble.wav ];
        };

        generic = mkSoundListOption {
          description = "Sounds that play when a transaction succeeds";
          default = [ ./public/sounds/kaching.wav ];
        };

        articles = mkOption {
          description = "Sounds that play when a specific article is bought";
          type = types.attrsOf (types.listOf types.path);
          default = { };
          example = ''
            {
              Spezi = [
                ./public/sounds/spezi_1.wav
              ];

              Bier = [
                ./public/sounds/bier.wav
                ./public/sounds/bier_1.wav
                ./public/sounds/bier_2.wav
                ./public/sounds/bier_3.wav
                ./public/sounds/bier_4.wav
                ./public/sounds/bier_5.wav
              ];
            }
          '';
        };
      };
    };
  };

  config = let config-file = pkgs.writers.writeYAML "config.yaml" cfg.settings;
  in mkIf cfg.enable {
    systemd.services."strichliste-rs" = lib.mkMerge [
      {
        description = "Strichliste-rs: A digital tally sheet";
        wantedBy = [ "multi-user.target" ];
        serviceConfig = {
          Environment = [
            "LEPTOS_SITE_ROOT=${cfg.package}/bin/site"
            "LEPTOS_SITE_ADDR=${cfg.address}:${toString cfg.port}"
          ];
          ExecStart =
            "${cfg.package}/bin/strichliste-rs -d ${cfg.dataDir} -c ${config-file}";
          Restart = "on-failure";

          User = "strichliste-rs";
          DynamicUser = true;
        };
      }
      (lib.mkIf (cfg.dataDir == statedirDefaultDir) {
        serviceConfig.StateDirectory = "strichliste-rs";
      })
    ];
  };
}
