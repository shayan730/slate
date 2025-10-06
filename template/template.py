#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import sys
import os
import time
from PIL import Image

libdir = os.path.join(os.path.dirname(os.path.realpath(__file__)), '..', 'lib')
if os.path.exists(libdir):
    sys.path.append(libdir)

from waveshare_epd import epd7in5_V2

def main():
    try:
        epd = epd7in5_V2.EPD()
        epd.init()
        epd.Clear()

        # Load image
        image = Image.open('../weather/weather.png')
        image = image.convert('1')  # Convert to 1-bit mode
        image = image.resize((800, 480))  # Match display resolution

        # Display image
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

