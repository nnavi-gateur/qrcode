extern crate qrcodegen;
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;
use qrcodegen::svg::create_svg;



fn do_basic_demo() {
	let text: &'static str = "Hello, world!";   // User-supplied Unicode text
	let errcorlvl: QrCodeEcc = QrCodeEcc::Low;  // Error correction level
	
	// Make and print the QR Code symbol
	let qr: QrCode = QrCode::encode_text(text, errcorlvl).unwrap();
	create_svg(&qr);
}

fn main() {
    do_basic_demo();
}