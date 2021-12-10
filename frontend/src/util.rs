use stylist::Style;

pub fn get_styles(styles: &[Style]) -> String {
    if cfg!(not(target_arch = "wasm32")) {
        styles
            .iter()
            .map(|s| s.get_style_str().to_owned())
            .collect::<Vec<String>>()
            .join("\n\n")
    } else {
        String::new()
    }
}
