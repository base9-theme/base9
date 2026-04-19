# Official Base9 Builder

For more info about base9 in general, go to [here](https://github.com/base9-theme/base9)

## CLI

Help page:
```
$ base9-builder help
base9 builder CLI

USAGE:
    base9-builder <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    help       Print this message or the help of the given subcommand(s)
    preview    prints a table of all generated colors to preview
    render     renders theme template
```

Example commands:
```bash
PALETTE="282936-E9E9F4-FF5555-FFB86C-F1FA8C-50FA7B-8BE9FD-BD93F9-FF79C6"

# preivew default palette in stdout
base9-builder preview -

# render mustache in stdout
base9-builder render $PALETTE template.mustache

# render mustache to file
base9-builder render $PALETTE template.mustache out.txt
```

## NPM/WASM/Typescript

```ts
// Only required for browser version
import init from 'base9-builder';
await init();


import * as base9 from 'base9-builder';
const palette = "282936-E9E9F4-FF5555-FFB86C-F1FA8C-50FA7B-8BE9FD-BD93F9-FF79C6"

const colors = base9.getColors(palette);
console.log(colors.red.p100); // #ff5555

const data = base9.getData(palette);
console.log(data.foreground.p100.hex) // e9e9f4

const template = "foreground: {{foreground.p100.hex}}";
const rendered = base9.renderString(palette, template);
console.log(rendered); // foreground: e9e9f4

```

## Rust crate

```rust
use base9_builder::{Palette, to_mustache_data};
use mustache::compile_str;

fn main() {
    let palette_str = "282936-E9E9F4-FF5555-FFB86C-F1FA8C-50FA7B-8BE9FD-BD93F9-FF79C6";
    let template_str = "primary: {{primary.p100.hex}}";
    let palette = Palette::from_str(palette_str).unwrap();
    let data = to_mustache_data(palette);
    let template = compile_str(&template_str).unwrap();

    template.render_data(&mut io::stdout(), &data).unwrap();
    // prints: "primary: ff5555"
}
```

## Unstable features:

Future updates may break these features. Do not rely on them.

### Randomly Generate Palette
Instead of specifying all 9 colors, you can only specify a subset of them and
let the builder randomly generate the rest colors.

Use `_` for a single unspecified color and `?` for many unspecified color.
`?` can only be used at the end.

Examples:
- `?`: generate all 9 colors.
- `_-_-_-_-_-_-_-_-_`: generate all 9 colors.
- `_-_-?`: generate all 9 colors.
- `_-FFFFFF-?`: foreground is `#FFFFFF` and generate the rest.
- `000000-_-00FF00-?`: background is `#000000`, primary color is `#00FF00`, generate the rest.

### Get all Mustache Variables in JSON

For CLI:
```bash
PALETTE="282936-E9E9F4-FF5555-FFB86C-F1FA8C-50FA7B-8BE9FD-BD93F9-FF79C6"
base9-builder list-variables $PALETTE # prints all variables in json 
```

For rust crate:
```rust
use base9_builder::{Palette, to_data};
fn main() {
    let palette_str = "282936-E9E9F4-FF5555-FFB86C-F1FA8C-50FA7B-8BE9FD-BD93F9-FF79C6";
    let palette = Palette::from_str(palette_str).unwrap();
    let data = to_data(palette); // returns serde_json::Value
}
```
