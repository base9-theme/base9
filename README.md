# base9
![discord](https://img.shields.io/discord/1001259743815409715?label=base9&logo=discord&style=flat-square)

A system for generating themes for various apps using color science based on 9 colors that user prefers.

Go to https://base9-theme.github.io for guided setup.

It is a spiritual successor of [base16](https://github.com/chriskempson/base16), but offers better color theme and is more user friendly. It is also inspired by [themer.dev](https://github.com/themerdev/themer).
For more info, see
[history and vision](./docs/history_and_vision.md)

## How to it works

Color theme are generated from the palette consist of 9 colors:

Background, foreground and 7 additional colors in order of importance.

[Base9 Bulder](https://github.com/base9-theme/base9-builder) will generate more colors based on the palette using color science.
It will blend colors as necessary to create different shades, pick the red color for error, etc.

Lastly, it will generate theme files for specific apps using [mustache](http://mustache.github.io/).

## Where code lives

This repo only hosts the general base9 documentations.

- [base9-builder](https://github.com/base9-theme/base9-builder) hosts the core logic.
- [base9-templates](https://github.com/base9-theme/base9-templates) hosts
the base9 templates (which is used to generate themes) for each supported apps.
Although some more complicated templates are only referenced there
and are hosted in separate repo.
- [base9-theme.github.io](https://github.com/base9-theme/base9-theme.github.io) hosts the official website.

## Contribute
See [CONTRIBUTE.md](./CONTRIBUTE.md)