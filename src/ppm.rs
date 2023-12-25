use anyhow::Result;
use glam::DVec3;
use indicatif::ProgressIterator;
use itertools::Itertools;
use std::fs;

use crate::types::Color;

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

    pub fn write_fn(self, filename: &str, f: impl Fn(f64, f64) -> Color) -> Result<()> {
        let header = format!("P3\n{} {}\n{}\n", self.width, self.height, self.max_color);

        let content = (0..self.height)
            .cartesian_product(0..self.width)
            .progress_count(self.width as u64 * self.height as u64)
            .map(|(y, x)| {
                let res = f(
                    x as f64 / (self.width - 1) as f64,
                    y as f64 / (self.height - 1) as f64,
                );
                res.write()
            })
            .join("\n");

        fs::write(filename, format!("{header}\n{content}"))?;
        return Ok(());
    }
}
