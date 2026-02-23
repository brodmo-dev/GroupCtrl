RELEASE_PATH := target/dx/GroupCtrl/release
MACOS_APP_PATH := $(RELEASE_PATH)/macos/GroupCtrl.app
MACOS_ZIP_PATH := $(RELEASE_PATH)/macos/GroupCtrl.zip
SIGNING_IDENTITY := Developer ID Application: Moritz Brödel (7P73434GLV)

macos-bundle:
	dx bundle --release
	cp assets/icon.icns $(MACOS_APP_PATH)/Contents/Resources/icon.icns  # Dioxus bug

macos-sign:
	codesign --force --options runtime --sign "$(SIGNING_IDENTITY)" $(MACOS_APP_PATH)

macos-notarize:
	ditto -c -k --keepParent $(MACOS_APP_PATH) $(MACOS_ZIP_PATH)
	xcrun notarytool submit $(MACOS_ZIP_PATH) --keychain-profile dev --wait
	xcrun stapler staple $(MACOS_APP_PATH)
	rm $(MACOS_ZIP_PATH)

macos-release: macos-bundle macos-sign macos-notarize

screenshot:
	cd assets && \
	cp screenshot.png screenshot.bak.png && \
	magick screenshot.png -channel A -threshold 99% +channel -trim +repage screenshot.png && \
	magick screenshot.png -bordercolor transparent -border 50x25 screenshot.png

icon:
	cd assets && \
	rsvg-convert tray-icon.svg | magick png:- tray-icon.rgba && \
	mkdir -p icon.iconset && \
	rsvg-convert -w 16   -h 16   icon.svg -o icon.iconset/icon_16x16.png && \
	rsvg-convert -w 32   -h 32   icon.svg -o icon.iconset/icon_16x16@2x.png && \
	rsvg-convert -w 32   -h 32   icon.svg -o icon.iconset/icon_32x32.png && \
	rsvg-convert -w 64   -h 64   icon.svg -o icon.iconset/icon_32x32@2x.png && \
	rsvg-convert -w 128  -h 128  icon.svg -o icon.iconset/icon_128x128.png && \
	rsvg-convert -w 256  -h 256  icon.svg -o icon.iconset/icon_128x128@2x.png && \
	rsvg-convert -w 256  -h 256  icon.svg -o icon.iconset/icon_256x256.png && \
	rsvg-convert -w 512  -h 512  icon.svg -o icon.iconset/icon_256x256@2x.png && \
	rsvg-convert -w 512  -h 512  icon.svg -o icon.iconset/icon_512x512.png && \
	rsvg-convert -w 1024 -h 1024 icon.svg -o icon.iconset/icon_512x512@2x.png && \
	iconutil -c icns icon.iconset --output icon.icns && \
	rm -rf icon.iconset
