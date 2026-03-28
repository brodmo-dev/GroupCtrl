<h1>
  <sub><img src="assets/icons/icon.png" width="35" alt="GroupCtrl icon" /></sub>
  GroupCtrl
</h1>

Instant app switching with shared hotkeys

<img alt="GroupCtrl screenshot" src="assets/screenshot.png?v=4" width="500">

`brew install brodmo/tap/groupctrl` or
download from [Releases](https://github.com/brodmo-dev/GroupCtrl/releases/latest)

## Features

- Shared hotkeys: Create an app group with multiple related apps and assign a single hotkey to it.
  Hit once to switch to the most recent app, hit again to cycle to next running.
- Fixed targets: Optionally select one app per group to always open first. If not running, it will be launched.
- App launcher: Hold a group hotkey to launch apps that aren't running.
  The launcher will open immediately if no app in the group is running.
- Text config: Easy to manually edit and version-control, if you're so inclined.

<!--- With the launcher open, hit to cycle which app to launch, enter to confirm. --->

## Tips

- Add GroupCtrl to `Open at Login` in System Settings
- Use [Hyperkey](https://hyperkey.app/) or [Karabiner](https://karabiner-elements.pqrs.org/)
  to map Caps Lock to `Cmd+Opt+Control`
- Config path is `~/.config/groupctrl/config.yaml`

### Useful macOS shortcuts

- `Cmd+Opt+D` to hide Dock
- `Cmd+Backtick` to switch between windows of an app
- `F11` to show desktop

## Alternatives

- [rcmd](https://lowtechguys.com/rcmd/) – Similar functionality, but shared hotkeys aren't practical
- [FlashSpace](https://github.com/wojciech-kulik/FlashSpace) – Workspace manager with app hiding
- [AeroSpace](https://github.com/nikitabobko/AeroSpace) – Tiling window manager

I've used all three extensively but was never quite satisfied.
I wanted to build something that combines the simplicity of rcmd
with the flexibility of FlashSpace and the consistency of AeroSpace.

## Roadmap

- [x] Add launcher pop-up
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
