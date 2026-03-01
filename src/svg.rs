use crate::{QrCode, QrCodeEcc};
use simple_svg::*;

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
