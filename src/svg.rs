use crate::{QrCode, QrCodeEcc};
use simple_svg::*;
use image::{RgbImage, Rgb};
use image::codecs::jpeg::JpegEncoder;
use base64::{Engine, engine::general_purpose};

pub fn create_qr_code(data: &str, level: i32) -> QrCode {
    if level == 1 {
        return QrCode::encode_text(data, QrCodeEcc::Low).unwrap();
    } else if level == 2 {
        return QrCode::encode_text(data, QrCodeEcc::Medium).unwrap();
    } else if level == 3 {
        return QrCode::encode_text(data, QrCodeEcc::Quartile).unwrap();
    } else if level == 4 {
        return QrCode::encode_text(data, QrCodeEcc::High).unwrap();
    } else {
        panic!("Invalid error correction level: {}", level);
    }
}
pub fn create_svg(qr: &QrCode) -> String {
    let size = qr.size() as f64;
    let module_size: f64 = 10.0; // 10 pixels per module
    let pixel_size = size * module_size;
    let mut svg = Svg::new(pixel_size, pixel_size);
    let square_id = svg.add_shape(Shape::Rect(Rect::new(module_size, module_size)));

    let mut circle_sstyle = Sstyle::new();
    circle_sstyle.stroke_width = Some(1.0);
    let mut black_sstyle = circle_sstyle.clone();
    black_sstyle.fill = Some("rgb(0, 0, 0)".to_string());
    let mut white_sstyle = circle_sstyle.clone();
    white_sstyle.fill = Some("rgba(255, 255, 255, 1)".to_string());

    let mut group = Group::new();
    let border: i32 = 4;
    for y in -border..qr.size() + border {
        for x in -border..qr.size() + border {
            let pos_x = (x) as f64 * module_size;
            let pos_y = (y) as f64 * module_size;
            if qr.get_module(x, y) {
                group.place_widget(Widget {
                    shape_id: square_id.clone(),
                    style: Some(black_sstyle.clone()),
                    at: Some((pos_x, pos_y)),
                    ..Default::default()
                })
            } else {
                group.place_widget(Widget {
                    shape_id: square_id.clone(),
                    style: Some(white_sstyle.clone()),
                    at: Some((pos_x, pos_y)),
                    ..Default::default()
                })
            };
        }
    }

    svg.add_default_group(group);
    let svg_str = svg_out(svg);
    println!("{}", svg_str);
    // std::fs::write("output.svg", svg_str).unwrap();
    return svg_str;
}
pub fn create_jpg(qr: &QrCode) -> String {
    let module_size: u32 = 10; // 10 pixels per module
    let border: i32 = 4;
    let qr_size = qr.size() as i32;
    let total_size = (qr_size + 2 * border) as u32;
    let image_size = total_size * module_size;
    
    // Create a new RGB image with white background
    let mut img = RgbImage::new(image_size, image_size);
    img.fill(255); // Fill with white
    
    let black = Rgb([0u8, 0u8, 0u8]);
    
    // Draw the QR code modules
    for y in -border..qr_size + border {
        for x in -border..qr_size + border {
            if qr.get_module(x, y) {
                let pixel_x_start = ((x + border) as u32) * module_size;
                let pixel_y_start = ((y + border) as u32) * module_size;
                
                // Fill the module (10x10 pixels)
                for py in pixel_y_start..pixel_y_start + module_size {
                    for px in pixel_x_start..pixel_x_start + module_size {
                        img.put_pixel(px, py, black);
                    }
                }
            }
        }
    }
    
    // Encode as JPG bytes
    let mut jpg_bytes = Vec::new();
    let encoder = JpegEncoder::new_with_quality(&mut jpg_bytes, 95);
    img.write_with_encoder(encoder).unwrap();
    
    // Convert to base64 string
    let jpg_str = general_purpose::STANDARD.encode(&jpg_bytes);
    println!("{}", jpg_str);
    return jpg_str;
}
