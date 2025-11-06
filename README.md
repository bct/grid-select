grid-select is a graphical menu for selecting an item from a small number of options. It is based on XMonad's [GridSelect][xmonad-gridselect], but can be used with any Wayland compositor.

[xmonad-gridselect]: (https://xmonad.github.io/xmonad-docs/xmonad-contrib/XMonad-Actions-GridSelect.html)

## Examples

Select from a list of random strings:

```sh
tr -dc a-z </dev/urandom | fold -w5 | head -n10 | grid-select
```

Select a Roman numeral, returning the corresponding decimal number:

```sh
echo -ne "1,i\n2,ii\n3,iii\n4,iv\n5,v" | grid-select -d ,
```

Switch to a selected hyprland workspace:

```bash
# this assumes that you're using ID-based workspaces that have been given a defaultName
# e.g. config lines like:
#
#   workspace = 1, defaultName:mon

dispatcher=workspace

# open the grid select to choose a workspace.
# only look at workspace rules with a defaultName
json=$(hyprctl workspacerules -j | jq '.[] | select(.defaultName != null)')

# produce a list of "workspaceId,defaultName" pairs
delimited_workspaces=$(echo "$json" | jq '"\(.workspaceString),\(.defaultName)"' -r)

# prompt for a workspace
workspace_id=$(echo "$delimited_workspaces" | grid-select -d ,)

# was a workspace selected?
if [ -n "$workspace_id" ]; then
    # switch to the selected workspace.
    hyprctl dispatch "$dispatcher" "$workspace_id"
fi
```

## Installation

### Nix

Install to your profile without flakes:

```
nix profile install github:bct/grid-select
```

Or with flakes:

```nix
# flake.nix

{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    grid-select.url = "github:bct/grid-select";
    grid-select.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =  { nixpkgs, grid-select, ... }: {
    nixosConfigurations.yourhostname = nixpkgs.lib.nixosSystem rec {
      system = "x86_64-linux";
      modules = [
        {
          environment.systemPackages = [
            grid-select.defaultPackage.${system}
          ];
        }
      ];
    };
  };
}
```

## Configuration

```toml
# ~/.config/grid-select/config.toml

# the width of each item in the grid
item_width = 80

# the height of each item in the grid
item_height = 40

# the space between items in the grid
item_margin = 5

font_size = 16
font_name = "TeX Gyre Adventor"

# colours are specified in hex format, RRGGBB
active_bg_colour = "000000"
active_fg_colour = "00cc00"

# you can specify a single bg_colour, or multiple.
# bg_colour = "336699"
# if multiple are specified, grid-select will cycle through the colours
# so that the (i % n)th item is rendered with the nth background.
bg_colour = ["336699", "996633"]
fg_colour = "000000"

border_width = 1
border_colour = "336699"
```

## Developing

`nix develop` to switch into a shell with all development dependencies installed.

`seq 25 | cargo run` to quickly test the build.
