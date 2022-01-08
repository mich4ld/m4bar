pub fn rgb(red: i32, green: i32, blue: i32) -> [f64; 3] {
    return [
        red as f64 / 255.0,
        green as f64 / 255.0,
        blue as f64 / 255.0,
    ];
}