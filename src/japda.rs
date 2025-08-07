pub fn from_hex(code: &str) -> iced::Color {
    let r = u8::from_str_radix(&code[1..3], 16);
    let g = u8::from_str_radix(&code[3..5], 16);
    let b = u8::from_str_radix(&code[5..7], 16);
    if let Ok(r) = r
        && let Ok(g) = g
        && let Ok(b) = b
    {
        iced::Color::from_rgb8(r, g, b)
    } else {
        iced::Color::from_rgb8(0, 0, 0)
    }
}
//아 메크로 써야하는데
