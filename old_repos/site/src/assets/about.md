# Base9

A system for generating themes for various apps using color science based on 9
colors that user prefers.

Here is how to use base9:

First, pick 9 colors, either by selecting from exist presets, or randomly generate them, or hand pick them by following [Color Palette Guide](/guide/color_palette).

Copy the base9 palette code, which is the `-` separated hex value of the 9 colors.

Example: 

`282936-E9E9F4-FF5555-FFB86C-F1FA8C-50FA7B-8BE9FD-BD93F9-FF79C6`

Checkout [list of supported templates](https://github.com/base9-theme/base9-templates) to see how to apply it to each application.


## History and Vision

Base9 wants to democratize color theme. It aims to let end users with little to no
programming or color science knowledge to customize its favorite apps.

Base9 aims to achieve it by having the user designing/specify only a few things
(as little as possible, only 9 colors for now) and let Base9 figure out the rest
using color science and programming.

Base9 also aims to let user apply the designed theme to all programs easily.

Without sacrificing such simplicity, Base9 also tries to provide limited, optional finer
control for the theme generation for user who wants more control.

## Why base9 instead of base16 (the original one)

Base9 is heavily inspired by [base16](https://github.com/chriskempson/base16/tree/450914aeda69e6d9df90cb3bf0cc1c70f8d5b0f5)

But when I created base9, base16 was more or less stopped being maintained.

FYI, there is another group of people also creating a spiritual successor of base16
that has similarities with base9 and is in early active development
(Check out [here](https://github.com/base16-project/home)).

Base9 aims to address many problems that the original base16 has.



### Less but more choices
A base9 theme plugin for any program does not contain bloats for hundreds of
color combinations. Instead, it allows user to pick its own colors and apply
it to the programs.

### Less but more colors
Base16 has 16 colors but 8 of those are just different shades of black/white.
Base9 only requires 9 colors. But it uses color science to generate
different shades of colors, allowing more vivid UI.

### Relative vs Absolute
In UI, we sometimes need to the color red to display error, and green to display
happy case. I call it absolute color.
In other places we just want our favorite color to pop out and another favorite
color in other places. I call it relative color.
Base16 does not differentiate it, therefore if the button color is
the same as the error color, it will always be. If you don't like red being the
button color, you have made your error message non-red as well.

In base9, you only specify relative colors, and it will pick the most red color
as red and the most green color as green, and apply them in appropriate places.

### Better ANSI Color
Base16 does not differentiate bright vs normal ANSI color, while base9 supports
it via brighter shades of the colors.

Base16 also overrides the first 6 colors of ANSI 256, which could break other
programs that uses ANSI 256 color. When only ANSI 256 is supported, Base9 will
try its best to display the best color without overriding it. (This is not
implemented yet)

## Centralized Community
Every implementation of base16
is in it's own repository with its owner, it's very
hard to make changes across all of them.

Base9 has a centralized org. This allows Base9 to make
improvements across all apps more quickly, and ensure all
templates stays top quality.

## How to implement base9 color theme for your favorite program

See [Guide](https://github.com/base9-theme/base9-templates/blob/main/TEMPLATE_GUIDE.md)

