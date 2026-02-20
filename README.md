# GroupCtrl

Hotkeys for switching apps

## Features

- App groups: Assign one hotkey for many apps and cycle between them.
- Target app: Select one app per group to always open first. If not running, it will be launched.
- Text config: Human-readable, can be manually edited and version-controlled.
- Permissions: None required.

## Tips

- Add GroupCtrl to `Open at Login` in System Settings
- Hide Dock using `Cmd+Opt+D`
- Use [Hyperkey](https://hyperkey.app/) or [Karabiner](https://karabiner-elements.pqrs.org/)
  to map Caps Lock to `Cmd+Opt+Control`

## Alternatives

- [rcmd](https://lowtechguys.com/rcmd/) —
  Similar, but fundamentally built around dynamic behavior. Paid.
- [FlashSpace](https://github.com/wojciech-kulik/FlashSpace) —
  Workspace manager with app hiding. Requires accessibility permissions.
- [AeroSpace](https://github.com/nikitabobko/AeroSpace) —
  Full tiling window manager. Takes over the entire layout.

I've used all three extensively but was never quite satisfied.
I wanted to build something that combines the simplicity of rcmd
with the flexibility of FlashSpace and the consistency of AeroSpace.

## Roadmap

### Complete Windows port

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
