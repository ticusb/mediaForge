use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use thiserror::Error;
use image::{RgbaImage, Rgba, DynamicImage};

#[derive(Debug, Error)]
pub enum LutError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
}

/// Minimal 3D LUT representation (cube) using nearest-neighbor lookup.
pub struct Lut3D {
    size: usize,
    /// Flattened RGB entries in row-major order: r fastest, then g, then b
    entries: Vec<[u8; 3]>,
}

impl Lut3D {
    /// Load a very small subset of .cube files. Supports comments, TITLE, LUT_3D_SIZE n,
    /// and then n^3 floating RGB values in 0..1 range.
    pub fn from_cube(path: &Path) -> Result<Self, LutError> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);

        let mut size: Option<usize> = None;
        let mut values: Vec<[f32; 3]> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let s = line.trim();
            if s.is_empty() || s.starts_with('#') {
                continue;
            }

            if s.to_uppercase().starts_with("LUT_3D_SIZE") {
                let parts: Vec<&str> = s.split_whitespace().collect();
                if parts.len() >= 2 {
                    size = parts[1].parse::<usize>().ok();
                }
                continue;
            }

            // Try parse three floats
            let parts: Vec<&str> = s.split_whitespace().collect();
            if parts.len() == 3 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    parts[0].parse::<f32>(),
                    parts[1].parse::<f32>(),
                    parts[2].parse::<f32>(),
                ) {
                    values.push([r, g, b]);
                    continue;
                }
            }

            // Ignore other directives like TITLE, DOMAIN_MIN/MAX
        }

        // If LUT_3D_SIZE directive was not present, try to infer from value count
        let size = match size {
            Some(s) => s,
            None => {
                let expected = values.len();
                let root = (expected as f64).cbrt().round() as usize;
                if root * root * root == expected && expected > 0 {
                    root
                } else {
                    return Err(LutError::Parse("Missing LUT_3D_SIZE and cannot infer size".into()));
                }
            }
        };

        let expected = size * size * size;
        if values.len() != expected {
            return Err(LutError::Parse(format!("Expected {} entries but found {}", expected, values.len())));
        }

        // Convert floats 0..1 to u8
        let entries = values
            .into_iter()
            .map(|c| {
                [
                    (c[0].clamp(0.0, 1.0) * 255.0) as u8,
                    (c[1].clamp(0.0, 1.0) * 255.0) as u8,
                    (c[2].clamp(0.0, 1.0) * 255.0) as u8,
                ]
            })
            .collect();

        Ok(Lut3D { size, entries })
    }

    /// Apply the LUT to an image using nearest neighbor in RGB cube.
    pub fn apply_to_image(&self, img: &DynamicImage) -> RgbaImage {
        let rgba = img.to_rgba8();
        let (w, h) = rgba.dimensions();
        let mut out = RgbaImage::new(w, h);

        for (x, y, pixel) in rgba.enumerate_pixels() {
            let r = pixel[0] as usize;
            let g = pixel[1] as usize;
            let b = pixel[2] as usize;

            // Map 0..255 -> 0..(size-1)
            let ri = r * (self.size - 1) / 255;
            let gi = g * (self.size - 1) / 255;
            let bi = b * (self.size - 1) / 255;

            let idx = Self::index(self.size, ri, gi, bi);
            let outc = self.entries[idx];

            out.put_pixel(x, y, Rgba([outc[0], outc[1], outc[2], pixel[3]]));
        }

        out
    }

    fn index(size: usize, r: usize, g: usize, b: usize) -> usize {
        // r fastest (innermost), then g, then b
        r + g * size + b * size * size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_lut_load_and_apply() {
        // Create a tiny 2x2x2 cube file
        let tmp = std::env::temp_dir().join("test_lut.cube");
        let mut f = File::create(&tmp).unwrap();
        writeln!(f, "LUT_3D_SIZE 2").unwrap();
        // Order: r varies fastest, then g, then b per .cube spec
        // We'll write values so that output equals input inverted (255 - channel)
        // For simplicity, just write two slices
        writeln!(f, "0 0 0").unwrap();
        writeln!(f, "1 0 0").unwrap();
        writeln!(f, "0 1 0").unwrap();
        writeln!(f, "1 1 0").unwrap();
        writeln!(f, "0 0 1").unwrap();
        writeln!(f, "1 0 1").unwrap();
        writeln!(f, "0 1 1").unwrap();
        writeln!(f, "1 1 1").unwrap();

        let lut = Lut3D::from_cube(&tmp).unwrap();
        assert_eq!(lut.size, 2);
        assert_eq!(lut.entries.len(), 8);

        // Apply to a 1x1 image (50,100,150)
        let img = DynamicImage::new_rgba8(1, 1);
        let out = lut.apply_to_image(&img);
        assert_eq!(out.dimensions(), (1, 1));

        let _ = std::fs::remove_file(tmp);
    }
}
