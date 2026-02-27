# This is all macOS

app_path := "target/dx/GroupCtrl/bundle/macos/bundle/macos/GroupCtrl.app"
signing_identity := "Developer ID Application: Moritz Brödel (7P73434GLV)"

release: icon bundle sign notarize dmg

bundle:
    dx bundle --release

sign:
    codesign --force --options runtime --sign "{{ signing_identity }}" {{ app_path }}

notarize:
    ditto -c -k --keepParent {{ app_path }} target/GroupCtrl.zip
    xcrun notarytool submit target/GroupCtrl.zip --keychain-profile dev --wait
    xcrun stapler staple {{ app_path }}

dmg:
    create-dmg {{ app_path }} target --overwrite || { echo "run: npm install -g create-dmg"; exit 1; }

[working-directory('assets/icons')]
icon:
    rsvg-convert tray-icon.svg | magick png:- tray-icon.rgba
    resvg -w 128 icon.svg icon.png
    mkdir -p icon.iconset
    for size in 16 32 128 256 512; do \
        rsvg-convert -w $size       icon.svg -o icon.iconset/icon_${size}x${size}.png; \
        rsvg-convert -w $((size*2)) icon.svg -o icon.iconset/icon_${size}x${size}@2x.png; \
    done
    iconutil -c icns icon.iconset --output icon.icns
    rm -rf icon.iconset

[working-directory('assets')]
screenshot:
    cp screenshot.png screenshot.bak.png
    magick screenshot.png -channel A -threshold 99% +channel -trim +repage screenshot.png
    magick screenshot.png -bordercolor transparent -border 50x25 screenshot.png
