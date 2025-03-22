use charming::{ImageFormat, ImageRenderer, theme::Theme};
use std::{fs, path::PathBuf, str::FromStr};
mod fetch;
mod send;
mod user;

fn main() {
    let theme = Theme::Dark;
    let f_chart = fetch::chart();
    let u_chart = user::chart();
    let s_chart = send::chart();

    let mut renderer = ImageRenderer::new(1200, 800).theme(theme);
    let mut path = fs::canonicalize(PathBuf::from_str("../").unwrap()).unwrap();

    let mut user_path = path.clone();
    user_path.push("user.png");

    let mut fetch_path = path.clone();
    fetch_path.push("fetch.png");

    let mut send_path = path.clone();
    send_path.push("send.png");

    renderer
        .save_format(ImageFormat::Png, &u_chart, &user_path)
        .unwrap();

    renderer
        .save_format(ImageFormat::Png, &f_chart, &fetch_path)
        .unwrap();

    renderer
        .save_format(ImageFormat::Png, &s_chart, &send_path)
        .unwrap();
}
