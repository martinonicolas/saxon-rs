use std::path::PathBuf;
use std::env;

fn main(){

    // Tells Cargo to re-run the build script if any files in the directory `./libsaxonc` has changed
    println!("cargo::rerun-if-changed=libsaxonc");

    // -- LINK -------------------------------------------------------------------------------------
    // ---------------------------------------------------------------------------------------------
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    
    println!("cargo:rustc-link-search={}",project_dir);
    println!("cargo:rustc-link-lib=saxonc");

    println!("cargo:rustc-link-search={}/libsaxonc/libs/nix", project_dir);
    println!("cargo:rustc-link-lib=saxon-hec-12.5.0");


    // -- BUILD -------------------------------------------------------------------------------------
    // ---------------------------------------------------------------------------------------------

    // 1. Compile Saxon into object files
    if !std::process::Command::new("gcc") 
        .arg("-Wall")
        .arg("-std=c99")
        .arg("-Ilibsaxonc/Saxon.C.API/graalvm")
        .arg("-c")
        .arg("libsaxonc/Saxon.C.API/SaxonCGlue.c")
        .arg("libsaxonc/Saxon.C.API/SaxonCProcessor.c")
        .arg("libsaxonc/Saxon.C.API/SaxonCXPath.c")
        .arg("-Wl,-rpath,libsaxonc/libs/nix")
        .arg("-ldl")
        .arg("-lsaxon-hec-12.5.0")
        .arg("-Llibsaxonc/libs/nix")
        .output()
        .expect("could not spawn `clang`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not compile object file");
    }

    // 2. Create static the library
    if !std::process::Command::new("ar")
        .arg("-cvq")
        .arg("libsaxonc.a")
        .arg("SaxonCGlue.o")
        .arg("SaxonCProcessor.o")
        .arg("SaxonCXPath.o")
        .output()
        .expect("could not spawn `ar`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not emit library file");
    }

    // 2. Remove object files, no longer needed
    if !std::process::Command::new("rm")
        .arg("-f")
        .arg("SaxonCGlue.o")
        .arg("SaxonCProcessor.o")
        .arg("SaxonCXPath.o")
        .output()
        .expect("could not spawn `ar`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not emit library file");
    }
    
    
    // -- BINDS -------------------------------------------------------------------------------------
    // ---------------------------------------------------------------------------------------------

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-Ilibsaxonc/Saxon.C.API")
        .clang_arg("-Ilibsaxonc/Saxon.C.API/graalvm")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

}