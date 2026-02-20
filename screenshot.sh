cp screenshot.png screenshot.png.bak
magick screenshot.png -channel A -threshold 100% +channel screenshot.png
magick screenshot.png -bordercolor transparent -border 50x25 screenshot.png
