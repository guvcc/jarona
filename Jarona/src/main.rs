use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        "reload" => {
            
        }

        "-s" => {
            
        }

        "--generate-conf" => {
            let home = std::env::var("HOME").unwrap();
            let config = format!("{}/.config/jarona",home)


            fs:create_dir_all(&config).unwrap();

            let packages_sf = format!("{}/packages")
            let packages_fi = format!("{}/packages/pkgs.jarona")
            let main_cfg = format!("{}/jarona.jarona")

            fs::create_dir_all(&packages_sf).unwrap();
            fs::write(packages_fi, r#"
                               // Package names for importing go here!    
                                   "#).unrwap();
            fs::write(main_cfg, r#"
            // Default config for Jarona
            jarona.pkgs = import$str(\"packages/pkgs.jarona\"); 
            "#, ) 
        }

        _ => println!("Unknown Command use --help to see list!")
    }
}