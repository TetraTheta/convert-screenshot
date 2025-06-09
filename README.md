# ConvertScreenshot

Perform various action to game screenshot.
* Blur UID area
* Crop image
* Resize image
* Convert to WebP format

## Usage

```plaintext
C:\TestDirectory> cs -h
Convert game screenshot

Usage: cs [OPTIONS] [TARGET]

Arguments:
  [TARGET]  Target directory (default: current working directory) [default: C:\TestDirectory]

Options:
      --crop-height <CROP_HEIGHT>  Manual override: crop height in pixel
      --crop-pos <CROP_POS>        Manual override: crop position [possible values: bottom, center, full]
  -g, --game <GAME>                Game that the screenshots are taken from [default: none] [possible values: none,
                                   wuwa]
  -o, --operation <OPERATION>      Operation to take on to the screenshots. If you specify anything other than 'Full' or
                                   'CreateDirectory', you must also set '-g|--game' to other than 'None' [default: full]
                                   [possible values: all, background, center, create-directory, foreground0,
                                   foreground1, foreground2, foreground3, foreground4, foreground5, full]
      --uid-area <UID_AREA>        Manual override: UID area as 'x,y'
      --uid-pos <UID_POS>          Manual override: UID position as 'x,y'
      --width-from <WIDTH_FROM>    Manual override: Width of original image
      --width-to <WIDTH_TO>        Manual override: Width of converted image
  -h, --help                       Print help
  -V, --version                    Print version
```

## Default Config

Config file, `cs.toml` will be created at the same directory where `cs.exe` is located.

```toml
[general]
folder_background = "CS-Background"
folder_center = "CS-Center"
folder_foreground0 = "CS-Foreground-0"
folder_foreground1 = "CS-Foreground-1"
folder_foreground2 = "CS-Foreground-2"
folder_foreground3 = "CS-Foreground-3"
folder_foreground4 = "CS-Foreground-4"
folder_foreground5 = "CS-Foreground-5"
folder_full = "CS-Full"

[game.wuwa]
crop_background_height = 360
crop_center_height = 200
crop_foreground0_height = 310
crop_foreground1_height = 420
crop_foreground2_height = 505
crop_foreground3_height = 580
crop_foreground4_height = 655
crop_foreground5_height = 730
uid_area = "1440,22"
uid_position = "144,1059"
```
