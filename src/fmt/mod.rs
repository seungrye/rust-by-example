use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

struct City {
    name: &'static str,
    lat: f32, // 위도
    lon: f32, // 경도
}
impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        return write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        );
    }
}
pub fn main() {
    for city in [
        City {
            name: "Doublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", city);
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        println!("{}", color);
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut hex: u32 = (self.red as u32) << (8 * 2);
        hex |= (self.green as u32) << (8);
        hex |= self.blue as u32;
        return write!(
            f,
            "RGB ({}, {}, {}) 0x{:06X}",
            self.red, self.green, self.blue, hex
        );
    }
}
