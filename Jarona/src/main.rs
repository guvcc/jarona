use std::{env, fs};
#[path = "jarona.lang/reader.rs"]
mod reader;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("reload") => {
            let home = env::var("HOME").unwrap();
            let config = format!("{}/.config/jarona", home);
            let main_cfg = format!("{}/jarona.jarona", config);

            reader::run_file(&main_cfg).unwrap();
            
        }

        Some("-s") => {

        }

        Some("--generate-conf") => {
            let home = env::var("HOME").unwrap();

            let config = format!("{}/.config/jarona", home);

            fs::create_dir_all(&config).unwrap();

            let packages_sf = format!("{}/packages", config);
            let packages_fi = format!("{}/packages/pkgs.jarona", config);
            let main_cfg = format!("{}/jarona.jarona", config);

            fs::create_dir_all(&packages_sf).unwrap();

            fs::write(
                &packages_fi,
                r#"
// Package names for importing go here!
                "#,
            )
            .unwrap();

            fs::write(
                &main_cfg,
                r#"
// Default config for Jarona

jarona.pkgs = import$str("packages/pkgs.jarona");
                "#,
            )
            .unwrap();
        }

        _ => println!("Unknown command, use --help to see list!"),
    }
}