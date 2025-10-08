#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import sys
import os
import time
from PIL import Image, ImageDraw, ImageFont
from textwrap import wrap

libdir = os.path.join(os.path.dirname(os.path.realpath(__file__)), '..', 'lib')
if os.path.exists(libdir):
    sys.path.append(libdir)

from waveshare_epd import epd7in5_V2

# Quotation text
quotation = """
The mother dead these fourteen years did incubate in her own bosom the creature who would carry her off. 
The father never speaks her name, the child does not know it. He has a sister in this world that he will not see again. 
He watches, pale and unwashed. He can neither read nor write and in him broods already a taste for mindless violence. 
All history present in that visage, the child the father of the man.
"""

def create_text_image(text, width=800, height=480, font_path='/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf', font_size=20):
    # Create a blank white image
    image = Image.new('1', (width, height), 255)  # '1' for 1-bit image, 255 for white background
    draw = ImageDraw.Draw(image)

    # Load a font
    font = ImageFont.truetype(font_path, font_size)

    # Wrap text to fit the image width
    margin = 10
    max_line_width = width - 2 * margin
    lines = []
    for paragraph in text.strip().split('\n'):
        wrapped = wrap(paragraph, width=80)  # Adjust width if necessary
        lines.extend(wrapped)
        lines.append('')  # Add a blank line between paragraphs

    # Draw text
    y = margin
    for line in lines:
        if y + font_size > height:
            break  # Stop drawing if we run out of space
        draw.text((margin, y), line, font=font, fill=0)  # 0 is black
        y += font_size + 2

    return image

def main():
    try:
        epd = epd7in5_V2.EPD()
        epd.init()
        epd.Clear()

        # Create image with text
        image = create_text_image(quotation)

        # Send to display
        epd.display(epd.getbuffer(image))
        time.sleep(2)

        epd.sleep()

    except IOError as e:
        print("IOError:", e)
    except KeyboardInterrupt:
        print("Interrupted")
        epd.sleep()
        sys.exit()

if __name__ == '__main__':
    main()
