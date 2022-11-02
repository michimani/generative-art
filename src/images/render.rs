use image::ImageBuffer;

pub fn render(filename: &str, pixels: Vec<[u8; 3]>, bounds: (usize, usize)) -> Result<(), &str> {
    if pixels.len() != bounds.0 * bounds.1 {
        return Err("length of pixels is not equal to bounds");
    }

    let mut imgbuf = ImageBuffer::new(bounds.0 as u32, bounds.1 as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb(pixels[(y * bounds.0 as u32 + x) as usize]);
    }

    match imgbuf.save(filename) {
        Ok(_) => Ok(()),
        _ => Err("failed to render image"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        let bounds: (usize, usize) = (100, 100);
        let mut pixels: Vec<[u8; 3]> = vec![[0, 0, 0]; bounds.0 * bounds.1];
        for row in 0..bounds.1 {
            for col in 0..bounds.0 {
                let idx = row * bounds.0 + col;
                pixels[idx] = [(idx % 255) as u8, (idx % 255) as u8, (idx % 255) as u8];
            }
        }

        let res = render("out/render_test.png", pixels, bounds);
        assert_eq!(false, res.is_err());
    }
}
