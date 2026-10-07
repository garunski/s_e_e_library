use std::{env, fs, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let product = manifest_dir.join("../../s_e_e");
    let assets = manifest_dir.join("assets");
    let input = manifest_dir.join("tailwind.css");
    let output = assets.join("tailwind.css");

    println!("cargo:rerun-if-changed={}", input.display());
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("src").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        product.join("gui/tailwind.css").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        product.join("gui_kit/src").display()
    );

    s_e_e_icons::write_gui_branding(&product).expect("write product branding assets");
    s_e_e_icons::write_gui_fonts(&product).expect("write product font assets");

    fs::create_dir_all(assets.join("branding")).expect("assets/branding");
    fs::create_dir_all(assets.join("fonts")).expect("assets/fonts");
    copy_tree(
        &product.join("gui/assets/branding"),
        &assets.join("branding"),
    )
    .expect("copy branding into site assets");
    copy_tree(&product.join("gui/assets/fonts"), &assets.join("fonts"))
        .expect("copy fonts into site assets");

    let dx_theme =
        manifest_dir.join("../../s_e_e_project/design-system/assets/dx-components-theme.css");
    if dx_theme.is_file() {
        println!("cargo:rerun-if-changed={}", dx_theme.display());
        fs::copy(&dx_theme, assets.join("dx-components-theme.css"))
            .expect("copy dx-components-theme.css");
    }

    s_e_e_icons::write_tailwind_files(&input, &output).expect("write library site tailwind css");
}

fn copy_tree(src: &std::path::Path, dest: &std::path::Path) -> std::io::Result<()> {
    if !src.is_dir() {
        return Ok(());
    }
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let from = entry.path();
        let to = dest.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}
