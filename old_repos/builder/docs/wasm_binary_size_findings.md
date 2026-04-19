# Findings while trying to reduce wasm binary size:

Guides followed:
https://github.com/johnthagen/min-sized-rust

Common command:
```bash
wasm-pack build --target web --release && ls -l pkg/base9_builder_bg.wasm
wasm-opt -Oz -o tmp.wasm pkg/base9_builder_bg.wasm && ls -l tmp.wasm
```

## History

### [commit on 2022-07-23](https://github.com/base9-theme/base9-builder/tree/63190a91e77c9f1290f1edb20c0fe9143bb7593e)

Findings:

- `regex` is very big, `const_regex` is much smaller.
- `json` is much smaller than `yaml`.
- `lto`, `opt-level=z`, `codegen-units=1`, `wasm-opt` are all somewhat useful.
- `opt-level=s`, `strip`, `panic=abort` are not as useful.
- `mustache`, `JsValue::from_serde` are big, but necessary.

Data:

```
+ serde_yaml + regex: 1041kb
+ serde_yaml + regex + lto: 1041kb
+ serde_yaml + regex + lto + opt=s: 950kb
+ serde_yaml + regex + lto + opt=z: 877kb
+ serde_yaml + const_regex + lto + opt=z: 335kb
+ serde_yaml + const_regex + lto + opt=s: 371kb
+ serde_yaml + const_regex + lto + opt=z + strip: 337kb
+ serde_yaml + const_regex + lto + opt=z + codegen=1: 328kb
+ serde_yaml + const_regex + lto + opt=z + codegen=1 + panic=abort: 328kb
+ serde_json + const_regex + lto + opt=z + codegen=1: 225kb
```

Then with
```
serde_json + const_regex + lto + opt=z + codegen=1: 225kb
```
as default, I did more experiments:

```
- mustache: 185kb
- JsValue.from_serde: 209kb
+ wasm-opt-Oz: 218kb
```

### [commit on 2022-08-05](https://github.com/base9-theme/base9-builder/tree/6d9b249f771fd06b906255d12dd410a0e1a36dd2)

Findings:
- Handlebars is about 170kb larger than mustache
- generator (probably because of rng) is 43kb
- small_rng for rand is larger.
- random range is not expensive.
- palette option check is 18kb.
- other regex check is cheap.



Data:
```
default: 220kb
+ handlebars: 378kb
+ generator: 263kb
+ generator + small_rng: 265kb
+ generator + small_rng + from_fixed_seed: 265kb
+ generator - inclusive_range: 263kb
+ generator - palette_regex_check: 245kb
+ generator - regex_check: 244kb
+ generator + smart_regex_check: 245kb
+ generator + smart_regex_check + wasm_opt: 243kb
+ generator + smart_regex_check + wasm_opt - inclusive_range: 243kb
```
Final:
```
+ generator + smart_regex_check + some_fix: 246kb
```
