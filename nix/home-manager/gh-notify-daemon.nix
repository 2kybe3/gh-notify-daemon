{
  lib,
  config,
  package,
  ...
}:
let
  cfg = config.gh-notify-daemon;
  gh-notify-daemon = cfg.package;
in
{
  options.gh-notify-daemon = {
    enable = lib.mkEnableOption "Enable gh-notify-daemon";

    secretFile = lib.mkOption {
      type = lib.types.path;
      description = ''
        File that contains the classic github Token.
        See https://github.com/settings/tokens.
      '';
    };
    package = lib.mkOption {
      type = lib.types.package;
      default = package;
      description = ''
        gh-notify-daemon package to use.
      '';
    };
  };
  config = lib.mkIf cfg.enable {
    systemd.user.services.gh-notify-daemon = {
      Unit.Description = "gh-notify-daemon";
      Service = {
        Environment = [ "GH_NOTIFY_DAEMON_TOKEN_FILE=${cfg.secretFile}" ];
        ExecStart = "${gh-notify-daemon}/bin/gh-notify-daemon";
      };

      Install.WantedBy = [ "default.target" ];
    };
  };
}
