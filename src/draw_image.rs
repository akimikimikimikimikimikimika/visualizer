use image::{RgbaImage,Rgba,ImageEncoder};
use image::codecs::png::PngEncoder;
use base64::encode as encode_base64;

use crate::lib::*;
use crate::draw_func::*;

pub fn draw_image(s:Status) {

	let output = s.output.as_ref().unwrap();

	try_catch!( frame(&s,&s.size).save(output) );

}

fn frame(s:&Status,size:&CU) -> RgbaImage {

	let mut ib = RgbaImage::new(size.0 as u32,size.1 as u32);

	match s.draw_mode {
		DM::Ansi => error!("ANSIカラー表示の出力には対応していません"),
		_ => {}
	}

	let subpixels = aa_subpixels(s.aa, &s.size, false);

	izip!(
			iproduct!(0..size.1,0..size.0),
			ib.pixels_mut()
		).par_bridge()
		.for_each(move |((y,x),p)| {

			let c = get_color(x,y,&size,s,subpixels.as_slice());

			*p = match c {
				C::None|C::Reverse => Rgba([0,0,0,0]),
				C::Float{r,g,b,a} => {
					let (r,g,b,a) = (
						(r*255.0).round() as u8,
						(g*255.0).round() as u8,
						(b*255.0).round() as u8,
						(a*255.0).round() as u8
					);
					Rgba([r,g,b,a])
				},
				C::Int{r,g,b,a} => Rgba([r,g,b,a]),
				C::GFloat{v,a} => {
					let (v,a) = (
						(v*255.0).round() as u8,
						(a*255.0).round() as u8
					);
					Rgba([v,v,v,a])
				},
				_ => { panic!(); }
			};

		});

	ib

}

pub fn base64_image(s:&Status) -> String {

	let size = (s.size.0*4,s.size.1*4);

	let mut png = Vec::new();
	let encoder = PngEncoder::new(&mut png);

	if let Err(e)=encoder.write_image(&frame(s,&size).into_raw(),size.0 as u32,size.1 as u32,image::ColorType::Rgba8) {
		eprintln!("エラーが発生しました: {:?}",e);
		std::process::exit(1);
	}

	encode_base64(png)

}