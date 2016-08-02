pub type Color = [f32; 4];

const RED: Color = [1.0, 0.0, 0.0, 1.0];
const GREEN: Color = [0.0, 1.0, 0.0, 0.3];
const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];
const ORANGE: Color = [1.0, 0.5, 0.0, 1.0];
const DEFBACKGROUND: Color = [0.55, 0.55, 0.55, 1.0];
const MACBACKGROUND: Color = [83.0 / 255.0, 127.0 / 255.0, 183.0 / 255.0, 1.0];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AppColors {
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Background,
    MacBackground,
}

/// Simple Color Chooser
impl Into<Color> for AppColors {
    fn into(self) -> Color {
        match self {
            AppColors::Red => RED,
            AppColors::Green => GREEN,
            AppColors::Blue => BLUE,
            AppColors::Yellow => YELLOW,
            AppColors::Orange => ORANGE,
            AppColors::Background => DEFBACKGROUND,
            AppColors::MacBackground => MACBACKGROUND,
        }
    }
}
