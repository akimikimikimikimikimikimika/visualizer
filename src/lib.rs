// unsigned int coordinate
pub type CU = (u16,u16);
// float coordinate
pub type CF = (f64,f64);

pub struct Status {
	pub size:      CU,
	pub draw_mode: DM,
	pub terminal:  TM,
	pub color:     TC,
	pub pixels:    TP,
	pub output:    Option<String>
}

pub enum InternalDrawMode {
	Ansi,
	Color(ColorStatus),
	Colorbar(ColorbarStatus),
	Mandelbrot
}
pub type DM = InternalDrawMode;

pub enum InternalTerminalMode {
	Texts,
	Image
}
pub type TM = InternalTerminalMode;

pub enum InternalTerminalColor {
	Auto,
	Less,
	Full
}
pub type TC = InternalTerminalColor;

pub enum InternalTerminalPixels {
	Single,
	Double
}
pub type TP = InternalTerminalPixels;

pub struct ColorStatus {
	pub angle: f64,
	pub mode:  CSMode,
	pub mouse_position: CF
}

pub enum CSMode {
	ConicW,
	Conic,
	BVW,
	BV
}

pub struct ColorbarStatus {
	pub mode:   CBSMode,
	pub repeat: bool
}
pub enum CBSMode {
	SMPTE,
	ARIB
}

pub enum C {
	Float{r:f64,g:f64,b:f64,a:f64},
	Int{r:u8,g:u8,b:u8,a:u8},
	GFloat{v:f64,a:f64},
	Reverse,
	None,
	Ansi(u8),KL,KD,RL,RD,GL,GD,YL,YD,BL,BD,ML,MD,CL,CD,WL,WD
}

#[macro_export]
macro_rules! error {
	($msg:expr) => {{
		eprintln!($msg);
		std::process::exit(1);
	}};
}

#[macro_export]
macro_rules! try_catch {
	($trial:expr) => {
		if let Err(e) = $trial {
			eprintln!("エラーが発生しました:\n{:?}",e);
			std::process::exit(1);
		}
	};
}