pub enum Colors {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Reset,
    None
}
impl Colors {
    pub fn get_color_code(&self) -> &'static str {
        match self {
            Colors::Red => "$1",
            Colors::Green => "$2",
            Colors::Yellow => "$3",
            Colors::Blue => "$4",
            Colors::Magenta => "$5",
            Colors::Cyan => "$6" ,
            Colors::Reset => "$0",
            Colors::None => "",
        }
    }
}

pub fn get_escape_code(colors: Colors) -> &'static str {
    return match colors {
        Colors::Red => "\x1b[31m",
        Colors::Green => "\x1b[32m",
        Colors::Yellow => "\x1b[33m",
        Colors::Blue => "\x1b[34m",
        Colors::Magenta => "\x1b[35m",
        Colors::Cyan => "\x1b[36m" ,
        Colors::Reset => "\x1b[0m",
        _ => ""
    }
}
