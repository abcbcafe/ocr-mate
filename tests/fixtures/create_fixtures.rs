// Helper script to generate test fixtures
use image::{DynamicImage, RgbaImage};

fn main() {
    // Create a simple test image
    let mut img = RgbaImage::new(100, 100);
    
    // Fill with white
    for pixel in img.pixels_mut() {
        *pixel = image::Rgba([255, 255, 255, 255]);
    }
    
    // Add a black bar (simulating text)
    for y in 40..60 {
        for x in 20..80 {
            img.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
        }
    }
    
    let dynamic_img = DynamicImage::ImageRgba8(img);
    dynamic_img.save("tests/fixtures/test_image.png").unwrap();
    
    println!("Created test_image.png");
}
