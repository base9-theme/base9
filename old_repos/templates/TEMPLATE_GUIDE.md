# Template Development Guide
This guide covers how to create a base9 template from scratch.

For most apps, you only need to write two files under `templates/<app-name>`:
- a config file with key meta infos to be used by the rest of base9 ecosystem.
- a [mustache](http://mustache.github.io/) file renders to the app's config file

Avoid adding too much extra files into this repo, since this repo could be used
by automated tools and will cost too much bandwidth otherwise.
This includes screenshots, executables, etc. Instead, upload them to external
hosting service, such as gist, separate repo, dedicated image hosting website.

## Writing config file for base9

It is required to have a config file named `config.yml` with key meta infos to
be used by the rest of base9 ecosystem.

It follows [json schema](https://json-schema.org/) at [`scripts/schema.yml`](./scripts/schema.yml).

You can also reference existing config files for other apps in `templates/<app-name>/config.yml`.

DO NOT write your own `README.md`. Instead, it will be generated based on the config.
It makes future base9 development been reflected on readme easier.
Raise issue if the template does not fit your need.

TODO(CONTRIB): Add github action to auto generate readme.

## How to write the mustache file

The file name must end with `.mustache`.
By convension name it `default[.ext].mustache`.

The guide will assume it's a dark theme.

Following this guide will ensure the theme is consistent with the rest of Base9 themes.
If following this guide makes the theme look worse in certain situations,
submit a PR to change this guide.
If this guide does not describe how you should write certain parts of the theme,
use your own judgement or reference existing themes.

Here is a list of well supported themes:
- vscode

### Data avaiable for mustache render

An example of complete set of data available for mustache render is
[here](https://jsoneditoronline.org/#left=url.https%3A%2F%2Fgist.githubusercontent.com%2Flijiaqigreat%2F1e0ddc268b35d06610a78156baffb14e%2Fraw%2Ffabafbd042bd77c35168009966c0edbda1096d77%2Fdata.json).

With a few exceptions, most colors can be accessed of the form `<semantic>.<shade>.<format>`.

An incomplete list exceptions worth mentioning:
- For [ANSI 4 bit](https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit), use `ansi.<name>.<format>`.
- The background color does not have shades, so use `background.<format>`.

### What color to pick

You should choose a semantic that closest to the purpose of where you use
it for.

Generally, most of interactable UI element should be `primary` and most other
interactable UI elements should be `secondary`.
Most non-interactable UI elements should be `foreground`.

Avoid using absolute and relative colors in the same place, since there might be
color collisions.

### What shade to pick

Other than the background, each color has 6 shades to choose from. They are
`p10`, `p25`, `p50`, `p75`, `p100`, `p125` from darkest to brightest. The names
roughly corresponds to the percentage of brightness compared to the original
color.

`p125` should be avoided since there might not be a color brighter than the
original color.

Here is how to pick them in different scenarios:

#### Foreground color of text

Use `p100` by default.

Use `p75` for a darker shade, such as:
- less important things
- comments
- disabled

Use `p50` for even less important things that are ok not to be readable.

To highlight a text, when methods like background highlight, border, bold
font are not possible, use `p125` as last resort.
#### Background highlight

Use `p25` for things requiring some user attention.

For elements requiring high attention, use inverted.

Use `p10` for to indicate a difference from regular background but does not need
user attention.

#### Border

Borders defined as UI elements that are much narrower than text height and
is only a few pixels wide.


Use `p50` by default.

Use `p75` for things requiring user attension. There should be only one UI
element with `p75` border in most cases.

Use `p25` for less important things, such as:
- character limit guide
- tree view vertical guide

#### Inverted

Inverted element is one with bright foreground and dark background.

Places where inverted should be used:
- buttons
- statusbar
- highlighting section of text where regular background highlight is
not strong enough or not ideal.

Use `p75` as background by default. `p100` as even brighter variant.

Use `background` as the text color.

#### Scroll bar

Scroll bar can be think of a special case for *inverted* element.

Use `p25` by default.

Use `p50` as brighter variant.

#### Selection

There are three types of recommende selection style for different scenarios:

Use `selection.p25` as background by default.

If the theme is for something that can select on top of some other
highlight, reverse background and foreground for selected section if the app supports it.
For example, terminal selection should use *inverted style*, since apps running inside
the terminal, such as vim, may already have some highlighted selection.

If reversing background and foreground is not possible, use *inverted style*:
`selection.p75` as background and `background` as foreground.

TODO(CONTRIB): reword this.

### Special notes about terminal apps

#### differentiate UI sections

Sometimes we need to differentiate UI sections,
but border is not possible/ideal. In that case, use either *inverted style*
or `p25` as background.

#### Limitted to ANSI 16

Use `ansi.*` only.

TODO: support `primary`, `secondary`, and aboslute color aliases.

#### Limitted to ANSI 256

base9 does not support ANSI 256 and will not support it in the near future.
