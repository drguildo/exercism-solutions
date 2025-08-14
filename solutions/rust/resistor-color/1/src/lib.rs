use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, IntoEnumIterator, PartialEq, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    if let Ok(color) = ResistorColor::from_int(value) {
        format!("{:?}", color)
    } else {
        String::from("value out of range")
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut resistors: Vec<ResistorColor> = ResistorColor::into_enum_iter().collect();
    resistors.sort_by_key(|r| r.int_value());
    resistors
}
