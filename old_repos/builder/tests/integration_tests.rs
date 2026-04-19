use std::{fs, str::FromStr, path::Path};
use ext_palette::Srgb;
use itertools::Itertools;

use base9_builder::{self, to_mustache_data, Palette, to_data};
use jsonschema::{JSONSchema, output::BasicOutput};

pub type Rgb = Srgb<u8>;

#[test]
fn correct_absolute_color() {
    let contents = fs::read_to_string("tests/palette_with_correct_absolute_order").unwrap();
    for palette_str in contents.split('\n') {
        if palette_str.len() == 0 || palette_str.chars().next().unwrap() == '#' {
            continue;
        }
        let palette = base9_builder::Palette::from_str(palette_str).unwrap();
        let data = base9_builder::to_mustache_data(&palette);
        // {{red.p100.hex}}-{{yellow.p100.hex}}-{{green.p100.hex}}-{{cyan.p100.hex}}-{{blue.p100.hex}}-{{magenta.p100.hex}}
        let template = mustache::compile_str(include_str!("../templates/absolute.mustache")).unwrap();
        let actual = template.render_data_to_string(&data).unwrap();
        let expected = &palette_str[((6+1)*2)..((6+1)*(6+2)-1)];
        assert_eq!(actual, expected, "wrong order: \nexpected: https://coolors.co/{}\nactual: https://coolors.co/{} ", palette_str, actual);
    }
}

#[test]
fn match_schema() {
    // Not working, not sure if we should use json schema.
    // assert_eq!(format!("{:0>3x}", 17), "011");
    let schema_path = "./tests/schema.yml";
    let yml = std::fs::File::open(Path::new(schema_path)).unwrap();
    let schema: serde_json::Value = serde_yaml::from_reader(yml).unwrap();
    let json_data = to_data(&Palette::from_str("?").unwrap());
    println!("{}", &serde_json::to_string_pretty(&schema).unwrap());
    println!("{}", &serde_json::to_string_pretty(&json_data).unwrap()[0..400]);
    let compiled_schema = JSONSchema::compile(&schema).expect("A valid schema");
    let output = compiled_schema.apply(&json_data).basic();
    match output {
    BasicOutput::Valid(_) => {
        return;
    },
    BasicOutput::Invalid(errors) => {
        for error in errors {
            eprintln!(
                "Error: {} at path {}",
                error.error_description(),
                error.instance_location()
            )
        }
        assert!(false);
    }
}
    
}