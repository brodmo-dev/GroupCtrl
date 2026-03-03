# macOS

Everything works through Keychain.

- Signing
    - Install Developer ID intermediate certificate from https://www.apple.com/certificateauthority/
    - Install Developer ID Application certificate (KeePass)
- Notarization
    - `xcrun notarytool store-credentials dev`
    - Team ID: 7P73434GLV
    - App-specific password for notarytool from https://account.apple.com

Then `just release`
