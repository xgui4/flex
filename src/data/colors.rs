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

fn get_color_name(color_code: &str) -> Colors {
    return match color_code {
        "$1" => Colors::Red,
        "$2" => Colors::Green,
        "$3" => Colors::Yellow,
        "$4" => Colors::Blue,
        "$5" => Colors::Magenta,
        "$6" => Colors::Cyan,
        "$7" => Colors::Reset,
        _ => Colors::None
    }
}

fn get_color_color_code(colors: Colors) -> &'static str {
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

pub fn get_color_color_code_fom_code(colors_code: &str) -> &str {
    return match colors_code {
        "$1" => "\x1b[31m",
        "$2" => "\x1b[32m",
        "$3" => "\x1b[33m",
        "$4" => "\x1b[34m",
        "$5" => "\x1b[35m",
        "$6" => "\x1b[36m" ,
        "$7" => "\x1b[0m",
        _ => ""
    }
}

