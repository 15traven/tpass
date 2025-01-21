#![windows_subsystem = "windows"]

use helpers::load_icon;
use tao::{
    event::Event,
    event_loop::{
        ControlFlow, 
        EventLoopBuilder,
    }
};
use tray_icon::{
    menu::{
        AboutMetadata, CheckMenuItem, Menu, 
        MenuEvent, MenuItem, PredefinedMenuItem, 
        Submenu
    },
    Icon,
    TrayIcon,
    TrayIconBuilder,
};

mod helpers;

enum UserEvent {
    MenuEvent(MenuEvent),
}

fn main() {
    let dark_icon_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/dark_icon.png");
    let dark_icon = load_icon(std::path::Path::new(dark_icon_path));

    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(UserEvent::MenuEvent(event));
    }));

    let configure_sub_menu = Submenu::new("Configure", true);
    let use_lower_item = CheckMenuItem::new("Use lowercase", true, true, None);
    let use_upper_item = CheckMenuItem::new("Use uppercase", true, true, None);
    let use_numbers_item = CheckMenuItem::new("Use numbers", true, true, None);
    let use_special_item = CheckMenuItem::new("Use symbols", true, true, None);
    let _ = configure_sub_menu.append_items(&[
        &use_lower_item,
        &use_upper_item,
        &use_numbers_item,
        &use_special_item,
        &PredefinedMenuItem::separator(),
        &PredefinedMenuItem::about(
            None,
            Some(AboutMetadata {
                name: Some("TPass".to_string()),
                version: Some("0.1.0".to_string()),
                ..Default::default()
            })
        ),
    ]);

    let tray_menu = Menu::new();
    let generate_item = MenuItem::new("Generate", true, None);
    let quit_item = MenuItem::new("Quit", true, None);
    let _ = tray_menu.append_items(&[
        &generate_item,
        &configure_sub_menu,
        &PredefinedMenuItem::separator(),
        &quit_item
    ]);
    let mut tray_icon: Option<TrayIcon> = None;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(tao::event::StartCause::Init) => {
                tray_icon = Some(
                    TrayIconBuilder::new()
                        .with_menu(Box::new(tray_menu.clone()))
                        .with_icon(dark_icon.clone())
                        .build()
                        .unwrap()
                );
            }
            Event::UserEvent(UserEvent::MenuEvent(event)) => {
                if event.id == quit_item.id() {
                    tray_icon.take();
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
    });
}
