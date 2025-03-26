

fn main() -> miette::Result<()> {


    #[cfg(feature = "saxon-he")]
    let libsaxonc_path = format!("libsaxonc-he");
    
    #[cfg(feature = "saxon-pe")]
    let libsaxonc_path = format!("libsaxonc-pe");

    #[cfg(feature = "saxon-ee")]
    let libsaxonc_path = format!("libsaxonc-ee");


    let include_dir = format!("{libsaxonc_path}/Saxon.C.API/graalvm");

    let c_objects = cc::Build::new().flag("-g")
        .flag("-fPIC")
        .include(&include_dir)
        .flag("-ldl")
        .flag("-lc")
        .flag("-lsaxon-hec-12.5.0")
        .files([
            format!("{libsaxonc_path}/Saxon.C.API/SaxonCGlue.c"),
            format!("{libsaxonc_path}/Saxon.C.API/SaxonCXPath.c"),
        ])
        .compile_intermediates();


    let mut build = autocxx_build::Builder::new("src/lib.rs", &[&format!("{libsaxonc_path}/Saxon.C.API"), &include_dir])
        .build().unwrap();

    build
        .files([
            format!("{libsaxonc_path}/Saxon.C.API/SaxonProcessor.cpp"),
            format!("{libsaxonc_path}/Saxon.C.API/SaxonApiException.cpp"),
            format!("{libsaxonc_path}/Saxon.C.API/XdmValue.cpp"),
            format!("{libsaxonc_path}/Saxon.C.API/XdmItem.cpp"),
            format!("{libsaxonc_path}/Saxon.C.API/XdmNode.cpp"),
            format!("{libsaxonc_path}/Saxon.C.API/XdmMap.cpp"),
            format!("{libsaxonc_path}/Saxon.C.API/XdmArray.cpp"),
            format!("{libsaxonc_path}/Saxon.C.API/XdmFunctionItem.cpp"),
            format!("{libsaxonc_path}/Saxon.C.API/XdmAtomicValue.cpp"),
            format!("{libsaxonc_path}/Saxon.C.API/DocumentBuilder.cpp"),
            format!("{libsaxonc_path}/Saxon.C.API/Xslt30Processor.cpp"),
            format!("{libsaxonc_path}/Saxon.C.API/XsltExecutable.cpp"),
            format!("{libsaxonc_path}/Saxon.C.API/XQueryProcessor.cpp"),
            format!("{libsaxonc_path}/Saxon.C.API/XPathProcessor.cpp"),
            format!("{libsaxonc_path}/Saxon.C.API/SchemaValidator.cpp"),
        ])
        .std("c++14")
        .flag("-g")
        .flag("-fPIC")
        .objects(c_objects)
        .cpp(true)
        .flag("-ldl")
        .flag("-lc")
        .flag("-lsaxon-hec-12.5.0")

        .compile("saxon-rs");


    // Add #[allow(unsafe_op_in_unsafe_fn)] #[cfg(not(doctest))] to the generated Rust bindings
    let cargo_out_dir = std::env::var("OUT_DIR").unwrap();
    let file_path = format!("{cargo_out_dir}/autocxx-build-dir/rs/autocxx-ffi-default-gen.rs");
    let contents = std::fs::read_to_string(&file_path).expect("No se pudo leer el archivo");
    let new_contents = format!("#[allow(unsafe_op_in_unsafe_fn)] #[cfg(not(doctest))]{}", contents);
    std::fs::write(file_path, new_contents).expect("No se pudo escribir el archivo modificado");

    
    let project_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}/libsaxonc/libs/nix", project_dir);
    println!("cargo:rustc-link-lib=saxon-hec-12.5.0");

    Ok(())
}