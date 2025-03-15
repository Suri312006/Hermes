use std::{any::Any, fs, path::PathBuf, str::FromStr};

use charming::{ImageFormat, ImageRenderer, theme::Theme};

mod fetch;

fn main() {
    let theme = Theme::Westeros;
    let chart = fetch::chart();

    let mut renderer = ImageRenderer::new(1000, 800).theme(theme);

    let mut path = fs::canonicalize(PathBuf::from_str("../").unwrap()).unwrap();
    // .unwrap()
    path.push("lets_go.png");

    renderer
        .save_format(ImageFormat::Png, &chart, path)
        .unwrap();
}
