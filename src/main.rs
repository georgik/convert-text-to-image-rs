use std::fs::File;
use std::io::Read;
use textwrap::{wrap};
use image::{ImageBuffer, Rgb};

const MAX_WIDTH: u32 = 960;
const MAX_HEIGHT: u32 = 540;
const CALENDAR_X_OFFSET: i32 = 400;
const TEMPERATURE_X_OFFSET: i32 = 20;
const FONT_SIZE: f32 = 34.0;
const TEMPERATURE_FONT_SIZE: f32 = 100.0;

// Acquire temperature from OpenWeather
fn get_temperature() -> String {
    let api_key = std::env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY is not set");
    let city_id = std::env::var("OPENWEATHER_CITY_ID").expect("OPENWEATHER_CITY_ID is not set");

    let url = format!("https://api.openweathermap.org/data/2.5/weather?id={}&appid={}&units=metric", city_id, api_key);
    let resp = reqwest::blocking::get(&url).unwrap();
    let body = resp.text().unwrap();
    let data: serde_json::Value = serde_json::from_str(&body).unwrap();
    let temp = data["main"]["temp"].as_f64().unwrap();
    let temp = (temp * 10.0).round() / 10.0;
    temp.to_string()
}

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
    let line_height = 34;
    let padding = 10;
    let height = line_height * lines.len() as u32 + padding * 2;

    // Create image buffer
    let mut img = ImageBuffer::<Rgb<u8>, _>::new(MAX_WIDTH, MAX_HEIGHT);
    img.fill(255);

    // Draw text onto image buffer
    let font = rusttype::Font::try_from_bytes(include_bytes!("../DejaVuSans.ttf")).unwrap();
    let scale = rusttype::Scale::uniform(FONT_SIZE);
    let v_metrics = font.v_metrics(scale);
    let mut y = padding as f32 + v_metrics.ascent;

    for line in lines {
        println!("{}", line);
        if line.is_empty() {
            y += line_height as f32;
            continue;
        }
        let glyphs: Vec<_> = font.layout(&line, scale, point(0.0, y)).collect();
        let width = glyphs.iter().rev().next().unwrap().position().x as u32;
        let x = (MAX_WIDTH - width) / 2;
        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let x = x as i32 + bb.min.x + CALENDAR_X_OFFSET;
                    let y = y as i32 + bb.min.y;
                    let coverage = 255 - ((v * 255.0) as u8);
                    if x>=0 && y>=0 && x<MAX_WIDTH as i32 && y<MAX_HEIGHT as i32 {
                        img.put_pixel(x as u32, y as u32, Rgb([coverage, coverage, coverage]));
                    }
                })
            }
        }
        y += line_height as f32;
    }

    // Redner temperature
    let temperature = get_temperature() + "°C";
    let scale = rusttype::Scale::uniform(TEMPERATURE_FONT_SIZE);
    let y = 200.0;
    // let temperature = "20.0".to_string();
    let glyphs: Vec<_> = font.layout(&temperature, scale, point(0.0, y)).collect();
    let width = glyphs.iter().rev().next().unwrap().position().x as u32;
    let x = (MAX_WIDTH - width) / 2;
    for glyph in glyphs {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x as i32 + bb.min.x + TEMPERATURE_X_OFFSET;
                let y = y as i32 + bb.min.y;
                let coverage = 255 - ((v * 255.0) as u8);
                if x>=0 && y>=0 && x<MAX_WIDTH as i32 && y<MAX_HEIGHT as i32 {
                    img.put_pixel(x as u32, y as u32, Rgb([coverage, coverage, coverage]));
                }
            })
        }
    }

    // Save image to file
    img.save("output.bmp").unwrap();
}

fn point(x: f32, y: f32) -> rusttype::Point<f32> {
    rusttype::Point { x, y }
}
