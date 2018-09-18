extern crate ptx_builder;

use ptx_builder::prelude::*;
use std::process::exit;

fn main() {
    if let Err(error) = build() {
        eprintln!("{}", BuildReporter::report(error));
        exit(1);
    }
}

fn build() -> Result<()> {
    let status = Builder::new(".")?.build()?;

    match status {
        BuildStatus::Success(output) => {
            println!(
                "cargo:rustc-env=KERNEL_PTX_PATH={}",
                output.get_assembly_path().to_str().unwrap()
            );

            for path in output.source_files()? {
                println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
            }
        }

        BuildStatus::NotNeeded => {
            println!("cargo:rustc-env=KERNEL_PTX_PATH=/dev/null");
        }
    };

    Ok(())
}
