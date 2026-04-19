use crate::palette::Palette;
use ext_palette::{IntoColor, Lab, rgb::channels::Argb, Srgb};
use rand::prelude::*;
use std::{ops::RangeInclusive, f32::consts::PI};
// use itertools::Itertools;
type Color = ext_palette::Lab;

use crate::{palette::PaletteOption};

#[derive(Debug)]
struct Config {
    is_dark: Option<bool>,
    hue_l: Option<RangeInclusive<f32>>,
    hue_c: Option<RangeInclusive<f32>>,
}

// struct Generator {
//     colors: [Option<Color>;9],
//     is_dark: Option<bool>,
//     hue_l: Option<RangeInclusive<f32>>,
//     hue_c: Option<RangeInclusive<f32>>,
// }

// impl Generator {
//     fn get_average_hue(&self) {

//     }
//     fn get_is_dark(&mut self) -> bool {
//         config.is_dark.get_or_insert(match (self.colors, average_hue) {
//             ([Some(c), ..], _) => is_dark(&c),
//             ([None, Some(c), ..], _) => !is_dark(&c),
//             ([None, None, ..], Some(c)) => !is_dark(&c),
//             ([None, None, ..], None) => rand::random::<bool>(),
//         })

//     }
// }

pub fn get_average_hue(palette_option: &PaletteOption) -> Option<Color> {
    let mut hue_average_lab_components = (0f32, 0f32, 0f32);
    let mut count = 0;
    for c in palette_option.colors {
        if c.is_none() { continue; }
        count += 1;
        let lab: ext_palette::Lab = c.unwrap().into_format().into_color();
        let comp = lab.into_components();
        hue_average_lab_components.0 += comp.0;
        hue_average_lab_components.1 += comp.1;
        hue_average_lab_components.2 += comp.2;
    }
    if count == 0 { return None; }
    let count = count as f32;
    hue_average_lab_components.0 /= count;
    hue_average_lab_components.1 /= count;
    hue_average_lab_components.2 /= count;
    Some(Color::from_components(hue_average_lab_components))
}


pub fn generate(palette_option: &PaletteOption) -> Palette {
    if palette_option.colors.iter().all(|x| x.is_some()) {
        return Palette { colors: palette_option.colors.map(|x| x.unwrap()) };
    }
    let mut colors = palette_option.colors.clone().map(|c| c.map(to_lab));

    // let mut rng = rand::rngs::SmallRng::seed_from_u64(0);
    let mut rng = rand::thread_rng();
    let mut config = Config {
        is_dark: None,
        // is_dark: Some(false),
        hue_l: None,
        hue_c: None,
        // hue_c: Some(50f32..=50f32),
    };

    let average_hue = get_average_hue(palette_option);
    config.is_dark.get_or_insert(match (colors, average_hue) {
        ([Some(c), ..], _) => is_dark(&c),
        ([None, Some(c), ..], _) => !is_dark(&c),
        ([None, None, ..], Some(c)) => !is_dark(&c),
        ([None, None, ..], None) => rand::random::<bool>(),
    });
    generate_bg(&mut rng, &mut colors, &config);
    generate_fg(&mut rng, &mut colors, &config);

    generate_hue(&mut rng, &mut colors, &mut config);
    // println!("{:?}", config);
    Palette { colors: colors.map(|x| from_lab(x.unwrap())) }
}

fn get_dark_color(rng: &mut impl rand::Rng) -> Color {
    let lab_0: Color = Srgb::from_u32::<Argb>(0xff_10009cu32).into_format().into_color();
    let lab_x: Color = Srgb::from_u32::<Argb>(0xff_003000u32).into_format().into_color();
    let lab_y: Color = Srgb::from_u32::<Argb>(0xff_5a0000u32).into_format().into_color();
    let mut rand_l: f32 = rng.gen();
    let mut rand_x: f32 = rng.gen();
    let mut rand_y: f32 = rng.gen();
    if rand_x + rand_y > 1. {
        rand_x = 1. - rand_x;
        rand_y = 1. - rand_y;
    }
    let mut rand_0 = 1. - rand_x - rand_y;
    rand_0 *= rand_l;
    rand_x *= rand_l;
    rand_y *= rand_l;

    let mut rtn = Lab::new(0.,0.,0.);
    rtn.l += lab_0.l * rand_0;
    rtn.l += lab_x.l * rand_x;
    rtn.l += lab_y.l * rand_y;

    rtn.a += lab_0.a * rand_0;
    rtn.a += lab_x.a * rand_x;
    rtn.a += lab_y.a * rand_y;

    rtn.b += lab_0.b * rand_0;
    rtn.b += lab_x.b * rand_x;
    rtn.b += lab_y.b * rand_y;

    rtn
}

fn get_light_color(rng: &mut impl rand::Rng) -> Color {
    let lab_w: Color = Srgb::from_u32::<Argb>(0xff_ffffffu32).into_format().into_color();
    let lab_0: Color = Srgb::from_u32::<Argb>(0xff_00e2ffu32).into_format().into_color();
    let lab_x: Color = Srgb::from_u32::<Argb>(0xff_00ef00u32).into_format().into_color();
    let lab_y: Color = Srgb::from_u32::<Argb>(0xff_ffadffu32).into_format().into_color();
    let mut rand_l: f32 = rng.gen();
    let mut rand_x: f32 = rng.gen();
    let mut rand_y: f32 = rng.gen();

    let mut rand_0 = 1. - rand_x - rand_y;
    let rand_w = 1. - rand_l;
    rand_0 *= rand_l;
    rand_x *= rand_l;
    rand_y *= rand_l;

    let mut rtn = Lab::new(0.,0.,0.);
    rtn.l += lab_w.l * rand_w;
    rtn.l += lab_0.l * rand_0;
    rtn.l += lab_x.l * rand_x;
    rtn.l += lab_y.l * rand_y;

    rtn.a += lab_0.a * rand_0;
    rtn.a += lab_x.a * rand_x;
    rtn.a += lab_y.a * rand_y;

    rtn.b += lab_0.b * rand_0;
    rtn.b += lab_x.b * rand_x;
    rtn.b += lab_y.b * rand_y;

    rtn
}

fn is_dark(c: &Lab) -> bool {
    c.l < 50.0
}

fn generate_bg(rng: &mut impl rand::Rng, colors: &mut [Option<Color>;9], config: &Config) {
    if colors[0].is_some() { return; }
    colors[0] = Some(if config.is_dark.unwrap() {
        get_dark_color(rng)
    } else {
        get_light_color(rng)
    });
}

fn generate_fg(rng: &mut impl rand::Rng, colors: &mut [Option<Color>;9], config: &Config) {
    if colors[1].is_some() { return; }
    colors[1] = Some(if config.is_dark.unwrap() {
        get_light_color(rng)
    } else {
        get_dark_color(rng)
    });
}

fn to_lab(c: crate::Color) -> Color {
    c.into_format().into_color()
}

fn from_lab(lab: Color) -> crate::Color {
    IntoColor::<Srgb>::into_color(lab).into_format()
}

pub fn length_and_angle(lab: &Color, base: &Color) -> (f32, f32) {
    let x = lab.a - base.a;
    let y = lab.b - base.b;
    let l = (x*x + y*y).sqrt();
    let angle = f32::atan2(y, x);

    (l, angle)
}

fn interpolate(bg: &Color, fg: &Color, l: f32) -> Color {
    let mut center = Color::new(l,0.,0.);

    if (bg.l < center.l) ^ (fg.l < center.l) {
        center.a = fg.a * (center.l - bg.l) / (fg.l - bg.l) + bg.a * (center.l - fg.l) / (bg.l - fg.l);
        center.b = fg.b * (center.l - bg.l) / (fg.l - bg.l) + bg.b * (center.l - fg.l) / (bg.l - fg.l);
    }

    center
}

fn generate_hue(rng: &mut impl rand::Rng, colors: &mut [Option<Color>;9], config: &mut Config) {
    let bg: Color = colors[0].unwrap();
    let fg: Color = colors[1].unwrap();

    let mut angles: Vec<f32> = Vec::new();
    let mut count = 0;
    {
        let mut max_l = Color::min_l();
        let mut min_l = Color::max_l();
        let mut max_c: f32 = 0.;
        let mut min_c: f32 = 300.;

        for c in &colors[2..] {
            if c.is_none() { continue; }
            let lab = c.unwrap();
            let center = interpolate(&bg, &fg, lab.l);
            let (d, th) = length_and_angle(&lab, &center);
            max_c = max_c.max(d);
            min_c = min_c.min(d);
            max_l = max_l.max(lab.l);
            min_l = min_l.min(lab.l);
            angles.push(th);
            count += 1;
        }
        if count == 0 {
            max_l = if config.is_dark.unwrap() {
                fg.l * 0.8
            } else {
                100. * (1. - 0.8) + fg.l*0.8
            };
            min_l = max_l;
            max_c = if config.is_dark.unwrap() {
                rng.gen_range(10f32..50f32)
            } else {
                rng.gen_range(20f32..50f32)
            };
            min_c = max_c;
        }
        config.hue_l.get_or_insert(min_l..=max_l);
        config.hue_c.get_or_insert(min_c..=max_c);
    }
    let mut new_angles = get_new_angles(rng, &angles);
    // println!("{:?}...{:?}", angles, new_angles);
    for c in colors {
        if c.is_some() { continue; }

        let angle = new_angles.pop().unwrap();
        let l = rng.gen_range(config.hue_l.clone().unwrap());
        let hue_distance = rng.gen_range(config.hue_c.clone().unwrap());
        // let l = 0.;
        // let hue_distance = 0.;
        let mut lab = interpolate(&bg, &fg, l);
        lab.a += angle.cos() * hue_distance;
        lab.b += angle.sin() * hue_distance;

        *c = Some(lab);
    }
}

fn angle_to_distance(a: f32) -> f32 {
    a
}

fn distance_to_angle(a: f32) -> f32 {
    a
}

fn get_new_angles(rng: &mut impl Rng, angles: &Vec<f32>) -> Vec<f32> {
    let remaining = 7 - angles.len();
    let mut rtn = Vec::<f32>::with_capacity(remaining);
    if remaining == 0 { return rtn; }
    let mut valid_distances: Vec<f32> = angles.iter().filter(|x| x.is_finite()).map(|x| angle_to_distance(*x)).collect();
    let module = PI * 2.;
    valid_distances.sort_by(|a,b| a.partial_cmp(b).unwrap());
    if valid_distances.len() == 0 {
        let start = rng.gen_range(0f32..module);
        rtn.push(distance_to_angle(start));
        for i in 1..7 {
            rtn.push(distance_to_angle((start + i as f32 * module / 7 as f32) % module));
        }
        rtn.shuffle(rng);
        return rtn;
    }
    if valid_distances.len() == 1 {
        let start = valid_distances[0];
        for i in 1..7 {
            rtn.push(distance_to_angle((start + i as f32 * module / 7 as f32) % module));
        }
        rtn.shuffle(rng);
        return rtn;
    }

    let mut gaps = Vec::<(f32, usize)>::with_capacity(valid_distances.len());
    for (i, d) in valid_distances.iter().enumerate() {
        let d2 = valid_distances[(i+1) % valid_distances.len()];
        gaps.push(((d2 + 2. * module - d) % module, 0));
    }
    
    for _i in 0..remaining {
        let slot = gaps.iter_mut().max_by(|(gap1, c1), (gap2, c2)|
            (gap1 / (c1 + 1) as f32).partial_cmp(&(gap2 / (c2 + 1) as f32)).unwrap()).unwrap();
        slot.1 += 1;
    }

    // println!("gaps: {:?}", &gaps);

    for (i, d) in valid_distances.iter().enumerate() {
        let (gap, count) = gaps[i];
        for j in 0..count {
            rtn.push(distance_to_angle((d + (j + 1) as f32 * gap / (count + 1) as f32) % module));
        }
    }

    rtn.shuffle(rng);
    rtn
}
