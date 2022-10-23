pub fn convert_to_ascii(img: image::DynamicImage) -> String {
    use image::GenericImageView;
    let (width, height) = img.dimensions();
    let scale = min(width / 150, height / 100);
    let (width, height) = ((width / scale) - 1, (height / scale / 2) - 1);

    let mut ascii = "".to_string();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x * scale, y * scale * 2);
            let intent = intent(pixel);
            let character = get_ascii_char(intent);
            ascii.push(character);
        }
        ascii.push('\n');
    }

    ascii
}

fn min(a: u32, b: u32) -> u32 {
    if a == 0 || b == 0 {
        1
    } else {
        match a < b {
            true => a,
            false => b,
        }
    }
}

fn intent(rgb: image::Rgba<u8>) -> u8 {
    match rgb.0[3] == 0 {
        true => 0,
        false => 255 - (rgb.0[0] / 3 + rgb.0[1] / 3 + rgb.0[2] / 3),
    }
}

fn get_ascii_char(intent: u8) -> char {
    let index = intent / 32;
    let ascii = [' ', '.', ',', '-', '~', '+', '=', '@'];
    ascii[index as usize]
}
