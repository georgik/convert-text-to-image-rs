use std::fs::File;
use std::io::Read;
use textwrap::{wrap};
use image::{ImageBuffer, Rgb};

const MAX_WIDTH: u32 = 900;

fn main() {
    // Load text from file
    let mut file = File::open("input.txt").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    // Split text into lines and wrap to fit within MAX_WIDTH
    let lines: Vec<_> = wrap(&text, MAX_WIDTH as usize)
        .iter()
        .map(|s| s.to_string())
        .collect();

    // Calculate height of the image
    let line_height = 20;
    let padding = 10;
    let height = line_height * lines.len() as u32 + padding * 2;

    // Create image buffer
    let mut img = ImageBuffer::<Rgb<u8>, _>::new(MAX_WIDTH, height);
    img.fill(255);

    // Draw text onto image buffer
    let font = rusttype::Font::try_from_bytes(include_bytes!("../DejaVuSans.ttf")).unwrap();
    let scale = rusttype::Scale::uniform(18.0);
    let v_metrics = font.v_metrics(scale);
    let mut y = padding as f32 + v_metrics.ascent;

    for line in lines {
        let glyphs: Vec<_> = font.layout(&line, scale, point(0.0, y)).collect();
        let width = glyphs.iter().rev().next().unwrap().position().x as u32;
        let x = (MAX_WIDTH - width) / 2;
        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let x = x as i32 + bb.min.x;
                    let y = y as i32 + bb.min.y;
                    let coverage = 255 - ((v * 255.0) as u8);
                    img.put_pixel(x as u32, y as u32, Rgb([coverage, coverage, coverage]));
                })
            }
        }
        y += line_height as f32;
    }

    // Save image to file
    img.save("output.bmp").unwrap();
}

fn point(x: f32, y: f32) -> rusttype::Point<f32> {
    rusttype::Point { x, y }
}
