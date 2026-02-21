cp screenshot.png screenshot.bak.png
magick screenshot.png -channel A -threshold 99% +channel -trim +repage screenshot.png
magick screenshot.png -bordercolor transparent -border 50x25 screenshot.png
