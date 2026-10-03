use see_library_site::documentation_route_paths;

fn main() {
    let paths: Vec<&str> = documentation_route_paths();
    println!("{}", serde_json::to_string(&paths).expect("serialize routes"));
}
