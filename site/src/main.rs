use see_library_site::App;

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    dioxus::launch(App);
}

#[cfg(all(not(target_arch = "wasm32"), feature = "server"))]
fn main() {
    dioxus::LaunchBuilder::server().launch(App);
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "server")))]
fn main() {
    eprintln!("see_library_site: enable the server feature to run the native fullstack binary");
    std::process::exit(1);
}
