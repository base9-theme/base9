mod wasm;
mod base9;
mod config;
mod color_science;
mod generator;
mod palette;

pub type Color = ext_palette::Srgb<u8>;
pub use palette::Palette;

pub fn to_data(palette: &Palette) -> serde_json::Value {
    let config = config::Config::from_palette(*palette);
    let variables = base9::get_variables(&config).unwrap();
    base9::format_variables(&config, &variables)
}

pub fn to_mustache_data(palette: &Palette) -> mustache::Data {
    let config = config::Config::from_palette(*palette);
    let variables = base9::get_variables(&config).unwrap();
    mustache::to_data(base9::format_variables(&config, &variables)).unwrap()
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


