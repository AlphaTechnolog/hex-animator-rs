use std::io::{self, Write, BufRead};

#[derive(Clone)]
struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

struct Hex {
    value: String
}

impl RGBA {
    pub fn from_hex(hex: &str) -> Self {
        let color = hex.trim_start_matches("#");
        let r = u8::from_str_radix(&color[0..2], 16).unwrap();
        let g = u8::from_str_radix(&color[2..4], 16).unwrap();
        let b = u8::from_str_radix(&color[4..6], 16).unwrap();
        let a = if color.len() == 8 {
            u8::from_str_radix(&color[6..8], 16).unwrap()
        } else {
            255
        };
        Self { r, g, b, a }
    }

    #[allow(dead_code)]
    pub fn display(&self) {
        println!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a);
    }

    pub fn as_hex(self) -> Hex {
        Hex::from_rgba(self.clone())
    }
}

impl Hex {
    pub fn from_rgba(rgba: RGBA) -> Self {
        let r = Hex::clip(rgba.r, 0, 255);
        let g = Hex::clip(rgba.g, 0, 255);
        let b = Hex::clip(rgba.b, 0, 255);
        let a = Hex::clip(rgba.a, 0, 255);

        Self {
            value: format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a),
        }
    }

    pub fn clip(num: u8, min_num: u8, max_num: u8) -> u8 {
        num.min(max_num).max(min_num)
    }

    pub fn display(&self) {
        println!("{}", self.value);
    }
}

fn animate(base_color: String, dest_color: String) {
    let mut rgba_base = RGBA::from_hex(&base_color);
    let mut rgba_dest = RGBA::from_hex(&dest_color);

    while
        rgba_base.r != rgba_dest.r ||
        rgba_base.g != rgba_dest.g ||
        rgba_base.b != rgba_dest.b ||
        rgba_base.a != rgba_dest.a
    {
        rgba_base.clone()
            .as_hex()
            .display();

        if rgba_base.r < rgba_dest.r {
            rgba_base.r += 1;
        } else if rgba_base.r > rgba_dest.r {
            rgba_base.r -= 1;
        }
        if rgba_base.g < rgba_dest.g {
            rgba_base.g += 1;
        } else if rgba_base.g > rgba_dest.g {
            rgba_base.g -= 1;
        }
        if rgba_base.b < rgba_dest.b {
            rgba_base.b += 1;
        } else if rgba_base.b > rgba_dest.b {
            rgba_base.b -= 1;
        }
        if rgba_base.a < rgba_dest.a {
            rgba_base.a += 1;
        } else if rgba_dest.a > rgba_dest.a {
            rgba_dest.a -= 1;
        }
    }

    rgba_base.clone()
        .as_hex()
        .display();
}

fn main() {
    let stdin = io::stdin();
    let mut base_color = String::new();
    let mut dest_color = String::new();

    print!("Write the base color (in hex): ");

    io::stdout()
        .flush()
        .unwrap();

    stdin.lock()
        .read_line(&mut base_color)
        .unwrap();

    base_color = base_color.trim().to_string();

    print!("Write the destination color (in hex): ");

    io::stdout()
        .flush()
        .unwrap();

    stdin.lock()
        .read_line(&mut dest_color)
        .unwrap();

    dest_color = dest_color.trim().to_string();

    animate(base_color, dest_color);
}
