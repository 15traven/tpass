use std::collections::HashSet;
use rand::seq::SliceRandom;
use clipboard::{ClipboardContext, ClipboardProvider};

pub fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();

        (rgba, width, height)
    };

    tray_icon::Icon::from_rgba(
        icon_rgba,
        icon_width,
        icon_height
    ).expect("Failed to open icon")
}

pub fn generate_password(
    length: usize,
    use_lowercase: bool,
    use_uppercase: bool,
    use_numbers: bool,
    use_symbols: bool
) {
    let mut character_pool = HashSet::new();

    if use_lowercase {
        character_pool.extend('a'..='z');
    }
    if use_uppercase {
        character_pool.extend('A'..='Z');
    }
    if use_numbers {
        character_pool.extend('0'..='9');
    }
    if use_symbols {
        character_pool.extend("!@#$%^&*()-_=+[]{}|;:'\",.<>?/`~".chars());
    }
    
    let character_pool: Vec<char> = character_pool.into_iter().collect();
    let password: String = (0..length)
        .map(|_| {
            *character_pool.choose(&mut rand::thread_rng()).expect("Character pool cannot be empty")
        })
        .collect();

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let _ = ctx.set_contents(password);
}