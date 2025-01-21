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
        AboutMetadata,
        Menu,
        MenuEvent,
        MenuItem,
        PredefinedMenuItem,
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

    let tray_menu = Menu::new();
    let generate_item = MenuItem::new("Generate", true, None);
    let quit_item = MenuItem::new("Quit", true, None);
    let _ = tray_menu.append_items(&[
        &generate_item,
        &PredefinedMenuItem::about(
            None,
            Some(AboutMetadata {
                name: Some("TPass".to_string()),
                version: Some("0.1.0".to_string()),
                ..Default::default()
            })
        ),
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
