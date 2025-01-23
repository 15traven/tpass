extern crate winres;
extern crate winapi;

fn main( ) {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_language(winapi::um::winnt::MAKELANGID(
            winapi::um::winnt::LANG_ENGLISH,
            winapi::um::winnt::SUBLANG_ENGLISH_US
        ));
        res.set_icon("./assets/icon.ico");
        res.compile().unwrap();
    }
}