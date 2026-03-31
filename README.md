# gh-notify-daemon

A simple github notification daemon

## Features
- Polls GitHub notifications
- Integrates with Home Manager

## Installation (NixOS + Home Manager)

### 1. Add the flake

```
gh-notify-daemon = {
  url = "git+https://git.kybe.xyz/2kybe3/gh-notify-daemon";
  inputs.nixpkgs.follows = "nixpkgs";
};
```

### 2. Enable the module

```
modules = [
    inputs.gh-notify-daemon.homeManagerModules.gh-notify-daemon
    ./home
];
```

### 3. Configure secrets (sops-nix example)

```
{self,config,...}:{
  sops.secrets.gh-notify-daemon = {
    sopsFile = "${self}/secrets/gh-notify-daemon.yaml";
    key = "token";
  };

  gh-notify-daemon = {
    enable = true;
    secretFile = config.sops.secrets.gh-notify-daemon.path;
  };
}
```
