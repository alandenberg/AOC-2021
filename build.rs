use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::io;

fn main()
{
    let target_dir = get_output_path();
    let src = Path::join(&env::current_dir().unwrap(), "data");
    let dest = Path::new(&target_dir);
    copy_dir_all(src, dest).expect("Could not copy files from data directory");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=cargo.lock");
    println!("cargo:rerun-if-changed=cargo.toml");
}

fn get_output_path() -> PathBuf
{
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string).join("target").join(build_type);
    return PathBuf::from(path);
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()>
{
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}