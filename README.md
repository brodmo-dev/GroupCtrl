<h1>
  <sub><img src="assets/icons/icon.png" width="35" alt="GroupCtrl icon" /></sub>
  GroupCtrl
</h1>

Instant app switching with shared hotkeys

[Download](https://github.com/brodmo-dev/GroupCtrl/releases/latest)

<img alt="GroupCtrl screenshot" src="assets/screenshot.png?v=4" width="500">

## Features

- Hotkey sharing: Create an app group with multiple related apps and assign a single hotkey to it.
  Hit once to switch to the most recent app, hit again to cycle to next running.
- Fixed target: Optionally select one app per group to always open first. If not running, it will be launched.
- Text config: Easy to manually edit and version-control, if you're so inclined.

<!--- App launcher: Hold a hotkey to launch any app in the group. The launcher will open immediately if no app in the group is running.-->

## Tips

- Add GroupCtrl to `Open at Login` in System Settings
- Use [Hyperkey](https://hyperkey.app/) or [Karabiner](https://karabiner-elements.pqrs.org/)
  to map Caps Lock to `Cmd+Opt+Control`
- Config path is `~/.config/groupctrl/config.yaml`

### Useful macOS shortcuts

- `Cmd+Opt+D` to hide Dock
- `Cmd+Backtick` to switch between windows of an app
- `F11` to show desktop

<!--- Use Vim binds (j/k) to quickly navigate between apps in the launcher-->

## Alternatives

- [rcmd](https://lowtechguys.com/rcmd/) – Similar functionality, but no practical hotkey sharing
- [FlashSpace](https://github.com/wojciech-kulik/FlashSpace) – Workspace manager with app hiding
- [AeroSpace](https://github.com/nikitabobko/AeroSpace) – Tiling window manager

I've used all three extensively but was never quite satisfied.
I wanted to build something that combines the simplicity of rcmd
with the flexibility of FlashSpace and the consistency of AeroSpace.

## Roadmap

- [ ] Add launcher pop-up
- [ ] Complete Windows port
    - [ ] Custom app picker
    - [ ] Windows app enumeration for picker
    - [ ] Windows app metadata extraction
    - [ ] Windows app launching
    - [ ] Windows window tracking
    - [ ] UWP app support

## Development

- Requires `npm`
- Run with `cargo run`

### Hot reload (macOS only)

- One-time: `cargo install dioxus-cli`
- `dx serve` & `npm run watch`
