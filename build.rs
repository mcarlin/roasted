use cornucopia::{CodegenSettings, Error};
use std::process::Command;

fn main() -> Result<(), Error> {
    let queries_path = "db/queries";
    let schema_file = "db/schema.sql";
    let destination = "src/cornucopia.rs";
    let settings = CodegenSettings {
        is_async: true,
        derive_ser: true,
    };

    println!("cargo:rerun-if-changed={queries_path}");
    println!("cargo:rerun-if-changed={schema_file}");
    cornucopia::generate_managed(
        queries_path,
        vec![schema_file.to_string()],
        Some(destination),
        false,
        settings,
    )?;

    Command::new("rustfmt")
        .args([destination])
        .output()
        .expect("failed to execute process");

    Ok(())
}
