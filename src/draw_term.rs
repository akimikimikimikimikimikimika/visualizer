use crossterm::{
	terminal,style,cursor,event,
	QueueableCommand,
	Result
};
use std::io::{stdout,Write};

use crossterm::tty::IsTty;
use style::{Color,Stylize};
use event::{Event,KeyCode,MouseEventKind};
use std::env::var as getenv;
use std::time::SystemTime as Time;

use crate::lib::*;
use crate::draw_func::*;
use crate::draw_func_color::{color_mouse_down,color_mouse_drag};
use crate::draw_image::base64_image;

pub fn draw_term(s:Status) {
	try_catch!( draw_term_impl(s) );
}

#[inline(always)]
fn draw_term_impl(mut s:Status) -> Result<()> {

	if !stdout().is_tty() { error!("出力結果をリダイレクトできません。"); }

	if matches!(s.draw_mode,DM::Ansi) {
		if matches!(s.terminal,TM::Image) { error!("ANSIカラー表示の画像出力には対応していません"); }
		s.color = TC::Full;
		if matches!(s.pixels,TP::Double) { error!("ANSIカラー表示の double 出力には対応していません"); }
	}
	else {
		if matches!(s.color,TC::Auto) && matches!(s.terminal,TM::Texts) {
			s.color = match getenv("COLORTERM") {
				Ok(val) => if val.eq("truecolor") { TC::Full } else { TC::Less },
				_ => TC::Less
			}
		}
	}

	let mut draggable = matches!(s.terminal,TM::Texts);

	terminal::enable_raw_mode()?;
	stdout()
		.queue(terminal::Clear(terminal::ClearType::All))?
		.queue(event::EnableMouseCapture)?
		.queue(cursor::Hide)?
		.flush()?;

	s.size = terminal::size()?;
	s.size.1 *= 2;
	frame(&mut s)?;

	loop {
		match event::read()? {
			Event::Key(e) => {
				match e.code {
					KeyCode::Char('q')|KeyCode::Enter|KeyCode::Esc => { break },
					_ => {}
				}
			}
			Event::Mouse(e) => {
				if draggable {
					if let DM::Color(cs) = &mut s.draw_mode {
						match e.kind {
							MouseEventKind::Down(_) => {
								let sp:CF = ( 0.5/(s.size.0 as f64) , 1.0/(s.size.1 as f64) );
								let coord = unify_coord(e.column, e.row*2, &s.size, &[sp])[0];
								color_mouse_down(&coord,&s.size,cs);
							},
							MouseEventKind::Drag(_)|MouseEventKind::Up(_) => {
								let st = Time::now();
								let sp:CF = ( 0.5/(s.size.0 as f64) , 1.0/(s.size.1 as f64) );
								let coord = unify_coord(e.column, e.row*2, &s.size, &[sp])[0];
								color_mouse_drag(&coord,&s.size,cs);
								frame(&s)?;
								let en = Time::now();
								if let Ok(d)=en.duration_since(st) {
									if d.as_secs_f64()>=0.1 { draggable = false; }
								}
							},
							_ => {}
						}
					}
				}
			}
			Event::Resize(w,h) => {
				s.size = (w,h*2);
				frame(&mut s)?;
			}
		}
	}

	stdout()
		.queue(terminal::Clear(terminal::ClearType::All))?
		.queue(cursor::MoveTo(0,0))?
		.queue(cursor::Show)?
		.queue(event::DisableMouseCapture)?
		.queue(terminal::EnableLineWrap)?
		.flush()?;
	terminal::disable_raw_mode()?;

	Ok(())
}

fn frame(s:&Status) -> Result<()> {
	match s.terminal {
		TM::Texts => frame_texts(s),
		TM::Image => frame_image(s)
	}
}

const SPACE:&str = " ";
const UPPER:&str = "▀";
const LOWER:&str = "▄";

#[inline(always)]
fn frame_texts(s:&Status) -> Result<()> {

	stdout()
		.queue(terminal::DisableLineWrap)?;

	let subpixels = aa_subpixels(s.aa, &s.size, match s.pixels {
		TP::Single => true,
		TP::Double => false
	});

	let cells:Vec<_> = iproduct!(
			(0..s.size.1).step_by(2),
			0..s.size.0
		).par_bridge()
		.map(move |(y,x)| {

			let osc = match s.pixels {

				TP::Single => {

					let c = get_color(x,y,&s.size,s,subpixels.as_slice());

					match c {
						C::None    => None,
						C::Reverse => Some(SPACE.reverse()),
						_ => {
							let color = convert_color(c,s);
							Some(SPACE.on(color))
						}
					}

				},

				TP::Double => {

					let c1 = get_color(x,y  ,&s.size,s,subpixels.as_slice());
					let c2 = get_color(x,y+1,&s.size,s,subpixels.as_slice());

					match (c1,c2) {
						(C::None,C::None) => None,
						(c1,C::None) => {
							let color = convert_color(c1,s);
							Some(UPPER.with(color))
						},
						(C::None,c2) => {
							let color = convert_color(c2,s);
							Some(LOWER.with(color))
						},
						(c1,c2) => {
							let color1 = convert_color(c1,s);
							let color2 = convert_color(c2,s);
							Some(LOWER.on(color1).with(color2))
						}
					}

				}

			};

			(x,y,osc)

		})
		.collect();

	for (x,y,osc) in cells.into_iter() {
		if let Some(sc) = osc {
			stdout()
				.queue(cursor::MoveTo(x,y/2))?
				.queue(style::PrintStyledContent(sc))?;
		}
	}

	stdout()
		.queue(cursor::MoveTo(0,0))?
		.flush()?;

	Ok(())
}

fn convert_color(c:C,s:&Status) -> Color {
	match c {
		C::Float{r,g,b,a:_} => match s.color {
			TC::Full => Color::Rgb{
				r:(r*255.0).round() as u8,
				g:(g*255.0).round() as u8,
				b:(b*255.0).round() as u8
			},
			TC::Less => {
				let ri = ((r*5.0) as u8).min(4);
				let gi = ((g*5.0) as u8).min(4);
				let bi = ((b*5.0) as u8).min(4);
				Color::AnsiValue(ri*36+gi*6+bi+0x10)
			},
			_ => { panic!(); }
		},
		C::Int{r,g,b,a:_} => match s.color {
			TC::Full => Color::Rgb{r,g,b},
			TC::Less => {
				let ri = ((r as f64)/256.0*5.0) as u8;
				let gi = ((g as f64)/256.0*5.0) as u8;
				let bi = ((b as f64)/256.0*5.0) as u8;
				Color::AnsiValue(ri*36+gi*6+bi+0x10)
			},
			_ => { panic!(); }
		},
		C::GFloat{v,a:_} => match s.color {
			TC::Full => {
				let u = (v*255.0).round() as u8;
				Color::Rgb{r:u,g:u,b:u}
			},
			TC::Less => Color::AnsiValue(
				match (v*25.0).round() as u8 {
					0 => 0x10,
					v if v>0 && v<25 => v+0xe7,
					25 => 0xe7,
					_ => { panic!(); }
				}
			),
			_ => { panic!(); }
		},
		C::Ansi(v) => Color::AnsiValue(v),
		C::KL => Color::DarkGrey,
		C::KD => Color::Black,
		C::RL => Color::Red,
		C::RD => Color::DarkRed,
		C::GL => Color::Green,
		C::GD => Color::DarkGreen,
		C::YL => Color::Yellow,
		C::YD => Color::DarkYellow,
		C::BL => Color::Blue,
		C::BD => Color::DarkBlue,
		C::ML => Color::Magenta,
		C::MD => Color::DarkMagenta,
		C::CL => Color::Cyan,
		C::CD => Color::DarkCyan,
		C::WL => Color::White,
		C::WD => Color::Grey,
		_ => { panic!(); }
	}
}

#[inline(always)]
fn frame_image(s:&Status) -> Result<()> {

	let s = format!(
		"\u{001B}]1337;File={}:{}\u{0007}",
		"name=Visualizer Image;inline=1;width=100%;height=100%;preserveAspectRatio=1",
		base64_image(s)
	);

	stdout()
		.queue(cursor::MoveTo(0,0))?
		.queue(style::PrintStyledContent(style::style(s)))?
		.queue(cursor::MoveTo(0,0))?
		.flush()?;

	Ok(())

}