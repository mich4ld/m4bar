pub fn rgb(red: i32, green: i32, blue: i32) -> [f64; 3] {
    [
        red as f64 / 255.0,
        green as f64 / 255.0,
        blue as f64 / 255.0,
    ]
}

pub fn hexify(string: &str) -> i32 {
    i32::from_str_radix(string, 16).unwrap()
}

pub fn hex_to_rgb(hex_str: String) -> [f64; 3] {
    let hex = hex_str.replace("#", "");

    let red = hexify(&hex[0..2]);
    let green = hexify(&hex[2..4]);
    let blue = hexify(&hex[4..6]);

    rgb(red, green, blue)
}
