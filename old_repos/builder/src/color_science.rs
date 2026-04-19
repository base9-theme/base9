use std::str::FromStr;

use itertools::Itertools;
use ext_palette::rgb::channels::Argb;
use ext_palette::{Srgb, Xyz, Lab, convert::IntoColorUnclamped, IntoColor, Lch};

use crate::{Color, palette::Palette};

pub type Rgb = Srgb<u8>;

pub fn mix1d(a: f32, b: f32, w: f32) -> f32 {
    return a*(1.-w)+b*w;
}

pub fn mix(c1: &Rgb, c2: &Rgb, w: f32) -> Rgb {
    let c1_xyz: Xyz = c1.into_format().into_color_unclamped();
    let c2_xyz: Xyz = c2.into_format().into_color_unclamped();
    let c1_lab: Lab = c1_xyz.into_color_unclamped();
    let c2_lab: Lab = c2_xyz.into_color();
    let c1_contrast = (c1_xyz.y + 0.05).ln();
    let c2_contrast = (c2_xyz.y + 0.05).ln();
    let c3_contrast = mix1d(c1_contrast, c2_contrast, w);
    let c3y = c3_contrast.exp() - 0.05;
    let c3l_lab: Lab = Xyz::new(0., c3y, 0.).into_color_unclamped();
    let c3 = Lab::new(c3l_lab.l, mix1d(c1_lab.a, c2_lab.a, w), mix1d(c1_lab.b, c2_lab.b, w));

    let c3: Srgb = c3.into_color_unclamped();
    c3.into_format()
}

#[derive(Debug)]
pub struct ColorNameWeight {
    pub color: Rgb,
    pub name: &'static str,
    pub weight: f32,
}

impl ColorNameWeight {
    fn new(hex: &str, name: &'static str, weight: f32) -> ColorNameWeight {
        ColorNameWeight {
            color: Rgb::from_str(hex).unwrap(),
            name: name,
            weight: 9.
        }
    }
}

pub fn average_color2(colors: &[Rgb]) -> Lab {
    let mut mut_average = (0.,0.,0.);
    for c in colors {
        let c: Srgb<f32> = c.into_format();
        mut_average.0 += c.red as f32;
        mut_average.1 += c.green as f32;
        mut_average.2 += c.blue as f32;
    }
    let len = colors.len() as f32;
    mut_average.0 /= len;
    mut_average.1 /= len;
    mut_average.2 /= len;
    let rtn: Lab = Srgb::<f32>::from_components(mut_average).into_color();
    rtn
}

pub fn average_color(colors: &[Rgb]) -> Lab {
    let mut mut_average = Lab::new(0.,0.,0.);
    for c in colors {
        let lab: Lab = c.into_format().into_color();
        mut_average += lab;
    }
    let len = colors.len() as f32;
    mut_average /= len;
    mut_average
}

pub fn minus_colors(colors: &[Rgb], average: Lab) -> [Lab; 7] {
    colors.iter().map(|c| {
        let lab: Lab = c.into_format().into_color();
        lab - average
    }).collect::<Vec<Lab>>().try_into().unwrap()
} 

pub fn min_order_by<F>(colors: &[Rgb], target: &[ColorNameWeight;6], mut f: F) -> [ColorNameWeight;6]
where F: FnMut(&Rgb, &ColorNameWeight) -> f32 {
    let result = colors.into_iter().permutations(target.len()).min_by_key(|perm| {
            let mut sum: f32 = 0.;
            for (ca, cr) in target.iter().zip_eq(perm) {
                sum += f(cr, ca);
            };
            (sum * 1000.) as i64
    });
    result.unwrap().iter().zip_eq(target.iter()).map(|(c, cnw)| {
        ColorNameWeight::new(&format!("{:x}", **c), cnw.name, cnw.weight)
    }).collect::<Vec<ColorNameWeight>>().try_into().unwrap()
}

pub fn score2(cr: &Rgb, caa: &ColorNameWeight, average: Lab, average2: Lab) -> f32 {
    let ca: Srgb<f32> = caa.color.into_format();
    let cr: Srgb<f32> = cr.into_format();
    let average: Srgb<f32> = average.into_color();
    let average2: Srgb<f32> = average2.into_color();
    let ca = Srgb::new(ca.red - average2.red, ca.green - average2.green, ca.blue - average2.blue);
    let cr = Srgb::new(cr.red - average.red, cr.green - average.green, cr.blue - average.blue);
    // println!("cr: {:?}", cr);
    // println!("ca: {:?}", ca);
    let ds = ((ca.red - cr.red), (ca.green - cr.green), (ca.blue - cr.blue));
    let weight = (1., 1., 1.);
    let tmp_sum = (ds.0 * ds.0 * weight.0 + ds.1*ds.1 * weight.1 + ds.2*ds.2*weight.2).sqrt();
    tmp_sum * caa.weight
}

pub fn score(cr: &Rgb, caa: &ColorNameWeight, average: Lab) -> f32 {
        let ca: Lch = caa.color.into_format().into_color();
        let cr: Lab = cr.into_format().into_color();
        let cr: Lab = cr - average;
        println!("cr: {:?}", cr);
        let cr: Lch = cr.into_color_unclamped();
        println!("cr: {:?}", cr);
        println!("ca: {:?}", ca);
        let ds = ((ca.l - cr.l), (ca.chroma - cr.chroma), (ca.hue - cr.hue).to_degrees());
        println!("ds: {:?}", ds);
        let weight = (0., 1., 6.);
        let tmp_sum = (ds.0 * ds.0 * weight.0 + ds.1*ds.1 * weight.1 + ds.2*ds.2*weight.2).sqrt();
        tmp_sum * caa.weight
}

pub fn get_matching_absolute_color(colors: &[Rgb]) -> Vec<ColorNameWeight> {
    let absolute_colors: [ColorNameWeight;6] = [
        ColorNameWeight::new("ff0000", "red", 9.),
        ColorNameWeight::new("ffff00", "yellow", 9.),
        ColorNameWeight::new("00ff00", "green", 9.),
        ColorNameWeight::new("00ffff", "cyan", 1.),
        ColorNameWeight::new("0000ff", "blue", 1.),
        ColorNameWeight::new("ff00ff", "magenta", 1.),
    ];

    let average = average_color2(colors);
    let average2 = average_color2(&absolute_colors.iter().map(|cnw| cnw.color).collect::<Vec<Rgb>>());
    let result = min_order_by(colors, &absolute_colors, |cr, caa| score2(cr, caa, average, average2));
    result.into()
}

pub fn closest_order<'a, 'b>(colors: &'a[Rgb], target: &'b [(Rgb, f32)]) -> Vec<&'a Rgb> {
        let average = {
          let mut mut_average: (f32, f32, f32) = (0.,0.,0.);
            for c in colors {
                let lab: Lab = c.into_format().into_color();
                mut_average.0 += lab.l;
                mut_average.1 += lab.a;
                mut_average.2 += lab.b;
            }
            mut_average.0 /= colors.len() as f32;
            mut_average.1 /= colors.len() as f32;
            mut_average.2 /= colors.len() as f32;
            mut_average
        };

        let result = colors.into_iter().permutations(target.len()).min_by_key(|perm| {
            let mut sum: f32 = 0.;
            for (ca, cr) in target.iter().zip_eq(perm) {
                let weight2 = ca.1;
                let ca: Lab = ca.0.into_format().into_color();
                let mut cr: Lab = cr.into_format().into_color();
                cr.a-=average.1;
                cr.b-=average.2;
                let ca: Lch = ca.into_color();
                let cr: Lch = cr.into_color();
                let ds = ((ca.l - cr.l), (ca.chroma - cr.chroma), (ca.hue - cr.hue).to_degrees());
                let weight = (0., 1., 6.);
                let tmp_sum = (ds.0 * ds.0 * weight.0 + ds.1*ds.1 * weight.1 + ds.2*ds.2*weight.2).sqrt();
                sum += tmp_sum * weight2;
            }

            (sum * 1000.) as i64
        }).unwrap();
        
        result.iter().map(|x| *x).collect()
}

#[test]
#[ignore]
fn tmp() {
    let palette = Palette::from_str("1d2021-d5c4a1-fb4934-fabd2f-b8bb26-8ec07c-83a598-d3869b-fe8019");
    let colors = [
       Rgb::from_str("ff5555").unwrap(),
       Rgb::from_str("f1fa8c").unwrap(),
       Rgb::from_str("50fa7b").unwrap(),
       Rgb::from_str("bd93f9").unwrap(),
       Rgb::from_str("8be9fd").unwrap(),
       Rgb::from_str("ff79c6").unwrap(),
       Rgb::from_str("ffb86c").unwrap(),
    ];
    let absolute_colors: [ColorNameWeight;6] = [
        ColorNameWeight::new("ff0000", "red", 1.),
        ColorNameWeight::new("ffff00", "yellow", 1.),
        ColorNameWeight::new("00ff00", "green", 1.),
        ColorNameWeight::new("00ffff", "cyan", 1.),
        ColorNameWeight::new("0000ff", "blue", 1.),
        ColorNameWeight::new("ff00ff", "magenta", 1.),
    ];
    let average = average_color2(&colors);
    let average2 = average_color2(&absolute_colors.iter().map(|cnw| cnw.color).collect::<Vec<Rgb>>());
    // println!("rr: {:?}", score2(&colors[0], &absolute_colors[0], average, average2));
    // println!("or: {:?}", score2(&colors[6], &absolute_colors[0], average, average2));
    println!("yy: {:?}", score2(&colors[1], &absolute_colors[1], average, average2));
    println!("oy: {:?}", score2(&colors[6], &absolute_colors[1], average, average2));
}