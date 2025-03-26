# SaxonRS

Rust bindings for SaxonC API.

This is a work in progress to build Rust bindings for the [SaxonC](https://www.saxonica.com/saxon-c/index.xml) library.



# Introduction

This repository includes two crates:  
- `saxonrs-sys`, a low-level crate that provides access to the SaxonC API methods via bindings, and thus employs unsafe code.  
- `saxonrs`, which leverages `saxonrs-sys` to expose a higher-level API, providing a safe and Rust-friendly way to use the API methods.



# How to build
1. Download SaxonC
2. Build saxonrs-sys
3. Build saxonrs

Before compiling **SaxonRS**, you must first download SaxonC and place it in the appropriate directory in order to build the `saxonrs-sys` crate.  
You can use the `download_saxon.sh` script, which automates all the necessary steps.  
This script automatically downloads the corresponding version of SaxonC and places it with the appropriate name in the correct directory.
```bash
cd <path_to_repository>
./download_saxonc.sh
```

---


# Saxon Licence
SaxonC (on the Native platform) is distributed in three editions: SaxonC-HE, SaxonC-PE, and SaxonC-EE. SaxonC-HE is available under an open-source license (specifically, the Mozilla Public License 2.0), and all its features are available to all users. SaxonC-PE and SaxonC-EE are commercial products and require activation by a license key.

Saxon-HE

Home Edition: open-source entry-level product (available from GitHub, Maven, and Nuget). Provides implementations of XSLT 3.0, XQuery 3.1, and XPath 3.1 at the basic level of conformance defined by W3C.
View full description for: SaxonJ-HE 12, SaxonC-HE 12

Saxon-PE

Professional Edition: adds a number of features to Saxon-HE, including optional features in XSLT 3.0, support for Saxon extensions, extensibility mechanisms, and optional features in XQuery 3.1; integration with the ICU library to provide localization for different languages; and support for external object models (such as JDOM2, XOM, DOM4J, AXIOM) on Java.
View full description for: SaxonJ-PE 12, SaxonC-PE 12


Saxon-EE

Enterprise Edition: the fully-featured product. Fully conformant XSD 1.0 and XSD 1.1 schema processor, with support for schema-aware XSLT and XQuery processing. Many other features including streaming in XSLT and XQuery, XSLT packages, support for XQuery updates, an advanced query optimizer, compilation of XQuery and XSLT code to Java bytecode, and much more.
View full description for: SaxonJ-EE 12, SaxonCS-EE 12, SaxonC-EE 12




https://www.saxonica.com/html/documentation12/about/license/licensekey.html 

We use `LICENSE_FILE_LOCATION` configuration property.