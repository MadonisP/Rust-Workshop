use image::{imageops, ImageBuffer, Rgba};
use qrcode_generator::QrCodeEcc;
use std::io::Cursor;

use create::Options;

pub(super) fn generate(
    input: String,
    options: Options,
    logo: &[u8],
    image_size: usize,
) -> Result<Vec<u8>, anyhow::Error> {
    let mut qr = image::DynamicImage::ImageLuma8(qrcode_generator::to_image_buffer(
        input,
        QrCodeEcc::Quartile,
        image_size,
    )?)
    .into_rgba8();

    if options.add_transparency == Some(true) {
        make_transparent(&mut qr);
    };

    if options.add_logo {
        add_logo(&mut qr, logo);
    };

    if options.add_gradient {
        add_gradient(&mut qr);
    };

    let mut result = vec![];
    qr.write_to(&mut Cursor::new(&mut result), image::ImageOutputFormat::Png)?;
    Ok(result);

    fn make_transparent(qr: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
        for (_x, _y, pixel) in qr.enumarate_pixels_mut() {
            if pixel.0 == [255, 255, 255, 255] {
                *pixel = Rgba([255, 255, 255, 0]);
            }
        }
    }

    fn add_logo(qr: &mut ImageBuffer<Rgba<u8>>, Vec<u8>, logo: &[u8]){
        let image_size = qr.width().min(qr.height()) as usize;
        let element_size = get_qr_element_size(image_size);

        let mut logo_size = element_size;

        while logo_size + 2 * element_size <=5 * image_size / 16 {
            logo_size +=2* element_size;
        }

        let mut logo = image::io::Reader::new(Cursor::new(logo)).with_gussed_format().unwrap().decode().unwrap();

        logo = logo.resize(logo_size as u32, logosize as u32, imageops::FilterType::Lanczos3);

        imageops::replace(qr,&logo,((image_size - logo-size)/2 as i64), ((image_size - logo_size)/2 as i64));

    }

}

fn add_gradient(qr: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let image_size = qr.width().min(qr.height()) as usize;
  
    let gradiant = colorgrad::CustomGradient::new ().colors(&[colorgrad::Color::from_rgba8(100,0,100,255), colorgrad::Color::from_rgba8(30,5,60,255)]).build().unwrap();
}
