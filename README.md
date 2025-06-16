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
  [TARGET]  Target directory (default: current working directory) [default: E:\REPO-HDD\convert-screenshot\.bin]

Options:
      --blur <BLUR>                Manual override: Area for blur, as 'x,y,width,height'
      --crop-height <CROP_HEIGHT>  Manual override: crop height in pixel
      --crop-pos <CROP_POS>        Manual override: crop position [possible values: bottom, center, full]
  -g, --game <GAME>                Game that the screenshots are taken from [default: none] [possible values: none,
                                   wuwa]
  -o, --operation <OPERATION>      Operation to take on to the screenshots. If you specify anything other than 'Full' or
                                   'CreateDirectory', you must also set '-g|--game' to other than 'None' [default: full]
                                   [possible values: all, background, center, create-directory, cutscene, foreground0,
                                   foreground1, foreground2, foreground3, foreground4, foreground5, full]
      --width-from <WIDTH_FROM>    Manual override: Width of original image
      --width-to <WIDTH_TO>        Manual override: Width of converted image
  -h, --help                       Print help
  -V, --version                    Print version
```

## Default Config

Config file, `cs.toml` will be created at the same directory where `cs.exe` is located.

```toml
[general.folder_name]
background = "CS-Background"
center = "CS-Center"
cutscene = "CS-Cutscene"
foreground0 = "CS-Foreground-0"
foreground1 = "CS-Foreground-1"
foreground2 = "CS-Foreground-2"
foreground3 = "CS-Foreground-3"
foreground4 = "CS-Foreground-4"
foreground5 = "CS-Foreground-5"
full = "CS-Full"

[game.wuwa.background]
crop_height = 360
crop_position = "bottom"
blur = [[40, 1054, 330, 22], [1733, 1058, 140, 22]]

[game.wuwa.center]
crop_height = 200
crop_position = "center"
blur = []

[game.wuwa.cutscene]
crop_height = 810
crop_position = "center"
blur = [[1781, 929, 110, 16]]

[game.wuwa.foreground0]
crop_height = 310
crop_position = "bottom"
blur = [[1733, 1058, 140, 22]]

[game.wuwa.foreground1]
crop_height = 420
crop_position = "bottom"
blur = [[1733, 1058, 140, 22]]

[game.wuwa.foreground2]
crop_height = 505
crop_position = "bottom"
blur = [[1733, 1058, 140, 22]]

[game.wuwa.foreground3]
crop_height = 580
crop_position = "bottom"
blur = [[1733, 1058, 140, 22]]

[game.wuwa.foreground4]
crop_height = 655
crop_position = "bottom"
blur = [[1733, 1058, 140, 22]]

[game.wuwa.foreground5]
crop_height = 730
crop_position = "bottom"
blur = [[1733, 1058, 140, 22]]

[game.wuwa.full]
crop_height = 0
crop_position = "full"
blur = [[40, 1054, 330, 22], [1733, 1058, 140, 22]]
```
