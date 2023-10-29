use std::process::Command;

const ASM_FILE: &str = "asm/context.S";
const OUT_FILE: &str = "asm/context.o";
const LIB_FILE: &str = "asm/libcontext.a";

fn main() -> Result<(), std::io::Error> {
    Command::new("cc")
        .args([ASM_FILE, "-c", "-fPIC", "-o", OUT_FILE])
        .status()?;
    Command::new("ar")
        .args(["crus", LIB_FILE, OUT_FILE])
        .status()?;

    // Link
    println!("cargo:rustc-link-search=native={}", "asm");
    println!("cargo:rustc-link-lib=static=context");
    println!("cargo:rerun-if-changed=asm/context.S");
    Ok(())
}
