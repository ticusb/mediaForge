// backend/src/services/processing.rs
// Self-hosted background removal and image processing

use image::{DynamicImage, Rgba, RgbaImage, GenericImageView};
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum ProcessingError {
    #[error("Model load failed: {0}")]
    ModelLoadFailed(String),
    #[error("Image load failed: {0}")]
    ImageLoadFailed(#[from] image::ImageError),
    #[error("Inference failed: {0}")]
    InferenceFailed(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub struct ImageProcessor {
    model_path: String,
}

impl ImageProcessor {
    pub fn new(model_path: String) -> Result<Self, ProcessingError> {
        // Verify model exists
        if !Path::new(&model_path).exists() {
            tracing::warn!("ML model not found at {}, using fallback processing", model_path);
        }

        Ok(Self { model_path })
    }

    /// Remove background from an image (simplified version for MVP)
    pub fn remove_background(
        &self,
        input_path: &Path,
        output_path: &Path,
    ) -> Result<(), ProcessingError> {
        let img = image::open(input_path)?;

        // For MVP: Use simple threshold-based background removal
        // In production, replace with actual ONNX model inference
        let result = self.simple_bg_removal(&img)?;

        result.save(output_path)?;
        tracing::info!("Background removed: {} -> {}", input_path.display(), output_path.display());

        Ok(())
    }

    /// Simple background removal using color threshold (MVP fallback)
    fn simple_bg_removal(&self, img: &DynamicImage) -> Result<RgbaImage, ProcessingError> {
        let (width, height) = img.dimensions();
        let rgba = img.to_rgba8();
        let mut result = RgbaImage::new(width, height);

        // Sample corners to determine background color
        let bg_color = self.estimate_background_color(&rgba);

        for (x, y, pixel) in rgba.enumerate_pixels() {
            let diff = self.color_distance(pixel, &bg_color);
            
            // If pixel is similar to background, make it transparent
            let alpha = if diff < 50.0 {
                0
            } else {
                255
            };

            result.put_pixel(x, y, Rgba([pixel[0], pixel[1], pixel[2], alpha]));
        }

        Ok(result)
    }

    fn estimate_background_color(&self, img: &RgbaImage) -> Rgba<u8> {
        let (width, height) = img.dimensions();
        
        // Sample corners
        let corners = vec![
            img.get_pixel(0, 0),
            img.get_pixel(width - 1, 0),
            img.get_pixel(0, height - 1),
            img.get_pixel(width - 1, height - 1),
        ];

        // Average the corner colors
        let avg_r = corners.iter().map(|p| p[0] as u32).sum::<u32>() / 4;
        let avg_g = corners.iter().map(|p| p[1] as u32).sum::<u32>() / 4;
        let avg_b = corners.iter().map(|p| p[2] as u32).sum::<u32>() / 4;

        Rgba([avg_r as u8, avg_g as u8, avg_b as u8, 255])
    }

    fn color_distance(&self, a: &Rgba<u8>, b: &Rgba<u8>) -> f32 {
        let r_diff = (a[0] as f32 - b[0] as f32).powi(2);
        let g_diff = (a[1] as f32 - b[1] as f32).powi(2);
        let b_diff = (a[2] as f32 - b[2] as f32).powi(2);
        (r_diff + g_diff + b_diff).sqrt()
    }

    /// Replace background with solid color
    pub fn replace_background(
        &self,
        input_path: &Path,
        output_path: &Path,
        bg_color: [u8; 3],
    ) -> Result<(), ProcessingError> {
        // First remove background
        let temp_path = std::env::temp_dir().join("temp_removed.png");
        self.remove_background(input_path, &temp_path)?;

        // Load transparent image
        let transparent = image::open(&temp_path)?.to_rgba8();

        // Create colored background
        let (width, height) = transparent.dimensions();
        let mut result = RgbaImage::new(width, height);

        // Fill with background color
        for pixel in result.pixels_mut() {
            *pixel = Rgba([bg_color[0], bg_color[1], bg_color[2], 255]);
        }

        // Composite foreground over background
        for (x, y, pixel) in transparent.enumerate_pixels() {
            let alpha = pixel[3] as f32 / 255.0;
            let bg_pixel = result.get_pixel_mut(x, y);

            bg_pixel[0] = ((pixel[0] as f32 * alpha) + (bg_pixel[0] as f32 * (1.0 - alpha))) as u8;
            bg_pixel[1] = ((pixel[1] as f32 * alpha) + (bg_pixel[1] as f32 * (1.0 - alpha))) as u8;
            bg_pixel[2] = ((pixel[2] as f32 * alpha) + (bg_pixel[2] as f32 * (1.0 - alpha))) as u8;
        }

        result.save(output_path)?;

        // Cleanup
        std::fs::remove_file(&temp_path).ok();

        Ok(())
    }

    /// Convert image format
    pub fn convert_format(
        &self,
        input_path: &Path,
        output_path: &Path,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<(), ProcessingError> {
        let mut img = image::open(input_path)?;

        // Resize if dimensions provided
        if let (Some(w), Some(h)) = (width, height) {
            img = img.resize_exact(w, h, image::imageops::FilterType::Lanczos3);
        }

        img.save(output_path)?;
        tracing::info!("Image converted: {} -> {}", input_path.display(), output_path.display());

        Ok(())
    }

    /// Apply color grading
    pub fn color_grade(
        &self,
        input_path: &Path,
        output_path: &Path,
        hue: Option<i32>,
        saturation: Option<i32>,
        brightness: Option<i32>,
        contrast: Option<i32>,
    ) -> Result<(), ProcessingError> {
        let img = image::open(input_path)?;
        let mut rgba = img.to_rgba8();

        // Apply adjustments
        if let Some(b) = brightness {
            self.adjust_brightness(&mut rgba, b);
        }
        if let Some(c) = contrast {
            self.adjust_contrast(&mut rgba, c);
        }
        if let Some(s) = saturation {
            self.adjust_saturation(&mut rgba, s);
        }
        if let Some(h) = hue {
            self.adjust_hue(&mut rgba, h);
        }

        rgba.save(output_path)?;
        tracing::info!("Color grading applied: {} -> {}", input_path.display(), output_path.display());

        Ok(())
    }

    fn adjust_brightness(&self, img: &mut RgbaImage, amount: i32) {
        for pixel in img.pixels_mut() {
            pixel[0] = (pixel[0] as i32 + amount).clamp(0, 255) as u8;
            pixel[1] = (pixel[1] as i32 + amount).clamp(0, 255) as u8;
            pixel[2] = (pixel[2] as i32 + amount).clamp(0, 255) as u8;
        }
    }

    fn adjust_contrast(&self, img: &mut RgbaImage, amount: i32) {
        let factor = (259.0 * (amount as f32 + 255.0)) / (255.0 * (259.0 - amount as f32));
        
        for pixel in img.pixels_mut() {
            pixel[0] = (factor * (pixel[0] as f32 - 128.0) + 128.0).clamp(0.0, 255.0) as u8;
            pixel[1] = (factor * (pixel[1] as f32 - 128.0) + 128.0).clamp(0.0, 255.0) as u8;
            pixel[2] = (factor * (pixel[2] as f32 - 128.0) + 128.0).clamp(0.0, 255.0) as u8;
        }
    }

    fn adjust_saturation(&self, img: &mut RgbaImage, amount: i32) {
        let factor = (amount as f32 + 100.0) / 100.0;
        
        for pixel in img.pixels_mut() {
            let gray = (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32) as u8;
            
            pixel[0] = (gray as f32 + factor * (pixel[0] as f32 - gray as f32)).clamp(0.0, 255.0) as u8;
            pixel[1] = (gray as f32 + factor * (pixel[1] as f32 - gray as f32)).clamp(0.0, 255.0) as u8;
            pixel[2] = (gray as f32 + factor * (pixel[2] as f32 - gray as f32)).clamp(0.0, 255.0) as u8;
        }
    }

    fn adjust_hue(&self, img: &mut RgbaImage, amount: i32) {
        let hue_shift = amount as f32 / 360.0;
        
        for pixel in img.pixels_mut() {
            let (h, s, v) = Self::rgb_to_hsv(pixel[0], pixel[1], pixel[2]);
            let new_h = (h + hue_shift) % 1.0;
            let (r, g, b) = Self::hsv_to_rgb(new_h, s, v);
            
            pixel[0] = r;
            pixel[1] = g;
            pixel[2] = b;
        }
    }

    fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            ((g - b) / delta) % 6.0
        } else if max == g {
            (b - r) / delta + 2.0
        } else {
            (r - g) / delta + 4.0
        } / 6.0;

        let s = if max == 0.0 { 0.0 } else { delta / max };
        let v = max;

        (h, s, v)
    }

    fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
        let c = v * s;
        let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = match (h * 6.0) as i32 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        (
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }

    /// Apply preset color grade
    pub fn apply_preset(&self, input_path: &Path, output_path: &Path, preset: &str) -> Result<(), ProcessingError> {
        match preset {
            "vintage" => self.color_grade(input_path, output_path, Some(15), Some(-20), Some(-10), Some(10)),
            "cinematic" => self.color_grade(input_path, output_path, Some(-5), Some(10), Some(-15), Some(20)),
            "bright" => self.color_grade(input_path, output_path, Some(0), Some(15), Some(30), Some(5)),
            _ => Err(ProcessingError::InferenceFailed(format!("Unknown preset: {}", preset))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_creation() {
        let processor = ImageProcessor::new("./models/u2net.onnx".to_string());
        assert!(processor.is_ok());
    }

    #[test]
    fn test_color_distance() {
        let processor = ImageProcessor::new("./models/u2net.onnx".to_string()).unwrap();
        let black = Rgba([0, 0, 0, 255]);
        let white = Rgba([255, 255, 255, 255]);
        let distance = processor.color_distance(&black, &white);
        assert!(distance > 400.0);
    }
}