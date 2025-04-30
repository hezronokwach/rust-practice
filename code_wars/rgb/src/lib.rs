fn rgb(r: i32, g: i32, b: i32) -> String {
    let mut red = r;
    let mut blue = b;
    let mut green = g;
    if red > 255 {
        red = 255;
    }
    if green > 255 {
        green = 255;
    }
    if blue > 255 {
        blue = 255;
    }
    if red < 0 {
        red = 0;
    }
    if green < 0 {
        green = 0;
    }
    if blue < 0 {
        blue = 0;
    }
    format!("{}{}{}", hex(red), hex(green), hex(blue))
}

fn hex(n: i32) -> String {
    if n == 0 {
        return "00".to_string();
    }
    let mut num = n;
    let mut result = String::new();
    let hex: Vec<char> = vec![
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    let div = num / 16;
    result.push(hex[div as usize]);
    let rem = num % 16;
    result.push(hex[rem as usize]);
    result
}

/*
fn rgb(r: i32, g: i32, b: i32) -> String {
    let r = r.min(255).max(0);
    let g = g.min(255).max(0);
    let b = b.min(255).max(0);
    format!("{:02X}{:02X}{:02X}", r, g, b)
} */
