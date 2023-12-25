use anyhow::Result;
use glam::DVec3;
use itertools::Itertools;
use std::{fs, io};

pub struct PPM {
    width: u32,
    height: u32,
    max_color: u8,
}

impl PPM {
    pub fn new(width: u32, height: u32, max_color: u8) -> Self {
        Self {
            width,
            height,
            max_color,
        }
    }

    // pub fn write(self, filename: &str, pixels: Vec<Vec<DVec3>>) -> Result<()> {
    //     let mut file = File::create(filename)?;
    //     let header = format!("P3\n{} {}\n{}\n", self.width, self.height, self.max_color);
    //     file.write_all(header.as_bytes())?;

    //     (0..self.height)
    //         .cartesian_product(0..self.width)
    //         .map(|(x, y)| {});
    //     return Ok(());
    // }

    pub fn write_fn(self, filename: &str, f: impl Fn(f64, f64) -> DVec3) -> Result<()> {
        let header = format!("P3\n{} {}\n{}\n", self.width, self.height, self.max_color);

        let content = (0..self.height)
            .cartesian_product(0..self.width)
            .map(|(y, x)| {
                let res = f(
                    x as f64 / (self.width - 1) as f64,
                    y as f64 / (self.height - 1) as f64,
                );
                let ir = (255.999 * res.x) as u32;
                let ig = (255.999 * res.y) as u32;
                let ib = (255.999 * res.z) as u32;
                format!("{} {} {}\n", ir, ig, ib)
            })
            .join("\n");

        fs::write(filename, format!("{header}\n{content}"))?;
        return Ok(());
    }
}
