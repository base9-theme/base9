## Template File
A mustache file used by builder to generate scheme for specific program.

Variables provided by builder:
- `scheme-name` - obtained from the scheme file
- `scheme-author` - obtained from the scheme file
- `scheme-slug` - obtained from the scheme filename, as described above
- `base00-hex-r` to `base0F-hex-r` - lower case hex value of red component.
- `base00-hex-g` to `base0F-hex-g` - lower case hex value of green component.
- `base00-hex-b` to `base0F-hex-b` - lower case hex value of blue component.
- `base00-rgb-r` to `base0F-rgb-r` - integer value of the red component.
- `base00-rgb-g` to `base0F-rgb-g` - integer value of the green component.
- `base00-rgb-b` to `base0F-rgb-b` - integer value of the blue component.
- `base00-dec-r` to `base0F-dec-r` - fraction value of the red component.
- `base00-dec-g` to `base0F-dec-g` - fraction value of the green component.
- `base00-dec-b` to `base0F-dec-b` - fraction value of the blue component.
- `base00-hex` to `base0F-hex` - hex value of rgb without the `#`.
- `base00-hex-bgr` to `base0F-hex-bgr` - hex value of blue-green-red without `#`

## Template Config File
A yaml file specifying how to generate templates.


default: 
    extension: .vim
    output: colors


## Considerations
Mustache was chosen as the templating language due to its simplicity and widespread adoption across languages. YAML was chosen to describe scheme and configuration files for the same reasons.
