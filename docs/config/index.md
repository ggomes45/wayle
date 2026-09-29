---
title: Config reference
outline: [2]
---

<div v-pre>

# Config reference

Every config file lives at `~/.config/wayle/config.toml`. Each page below covers one section. Every field has a default; start with an empty file and add only what you want to change.

::: tip
Editor intellisense via JSON Schema. Install [Tombi](https://marketplace.visualstudio.com/items?itemName=tombi-toml.tombi) for VSCode or the `tombi` LSP for Neovim, Helix, or Zed. The schema is written to `~/.config/wayle/schema.json` on startup.
:::

## Top-level sections

| Section | What it controls |
|---|---|
| [`bar`](/config/bar) | Bar chrome: per-monitor layout, spacing, colors, and button styling. |
| [`general`](/config/general) | Shell-wide settings that don't belong to any specific module. |
| [`osd`](/config/osd) | On-screen display overlay for transient events like volume and brightness. |
| [`styling`](/config/styling) | Theme, palette, and rounding tokens applied shell-wide. Changes recompile the stylesheet. |
| [`wallpaper`](/config/wallpaper) | Wallpaper rendering, cycling, and per-monitor overrides. |

## Bar modules

Modules appear inside `[[bar.layout]]` arrays. Each row links to the full reference.

| Module | Purpose |
|---|---|
| [`battery`](/config/modules/battery) | Battery level, charging state, and a dropdown with power-profile controls. |
| [`brightness`](/config/modules/brightness) | Backlight control bar module. |
| [`clock`](/config/modules/clock) | Time display with a calendar dropdown. |
| [`custom`](/config/modules/custom) | User-defined module that runs a shell command and renders the output in the bar. |
| [`dashboard`](/config/modules/dashboard) | Quick-access button with a distro icon; opens the dashboard dropdown. |
| [`hyprland-workspaces`](/config/modules/hyprland-workspaces) | Hyprland workspace indicators with click-to-switch. |
| [`keyboard-input`](/config/modules/keyboard-input) | Active keyboard layout indicator. |
| [`media`](/config/modules/media) | Now-playing title and playback controls for the active MPRIS player. |
| [`microphone`](/config/modules/microphone) | Microphone input level and mute toggle. |
| [`network`](/config/modules/network) | Network connection status with a dropdown for switching connections. |
| [`notifications`](/config/modules/notifications) | Notification center: icon in the bar, dropdown with history, DND toggle. |
| [`separator`](/config/modules/separator) | A vertical rule between bar modules. |
| [`systray`](/config/modules/systray) | System tray icons via the StatusNotifierItem protocol. |
| [`volume`](/config/modules/volume) | Output volume control with a dropdown for device and app volumes. |

## Shared types

Every named type referenced across the config (`Color`, `ClickAction`, `Spacing`, and others) is documented on the [types page](/config/types).

</div>
