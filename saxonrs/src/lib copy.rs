use std::{ffi::{c_char, CString}, ptr};

use saxonrs_sys::bindings::*;

pub struct SaxonRS {
    environi: *mut sxnc_environment,
    processor: *mut sxnc_processor,
    parameters: *mut sxnc_parameter,
    properties: *mut sxnc_property,
    parameters_capacity: i32,
    properties_capacity: i32,
}

impl SaxonRS {
    pub fn xquery(self) -> SaxonRSxqueryBuilder {
        SaxonRSxqueryBuilder {
            saxonrs: self,
            prop_len: 0,
            prop_cap: 10,
        }
    }



    pub fn new() -> Self {
        // Array of paramaters used for the transformation as (string, value) pairs
        let mut parameters: *mut sxnc_parameter = ptr::null_mut();
        
        let cap: i32 = 10;
        
        let par_cap:i32 = cap;

        // Array of properties used for the transformation as (string, string) pairs
        let mut properties: *mut sxnc_property = ptr::null_mut();

        let prop_cap = cap;

        let mut environi: *mut sxnc_environment = ptr::null_mut();
        let mut processor: *mut sxnc_processor = ptr::null_mut();
        
        unsafe { 
            initSaxonc(&mut environi, &mut processor, &mut parameters, &mut properties, par_cap, prop_cap);
            create_graalvm_isolate(environi);
            c_createSaxonProcessor(environi, processor, 0);
        }

        SaxonRS { 
            environi, 
            processor, 
            parameters, 
            properties, 
            parameters_capacity: par_cap, 
            properties_capacity: prop_cap,  
        }
    }  
}

impl Drop for SaxonRS {
    fn drop(&mut self) {
        
        unsafe {
            graal_tear_down((*self.environi).thread);
            freeSaxonc(&mut self.environi, &mut self.processor, &mut self.parameters, &mut self.properties);
        }
       println!("SaxonRS Dropped!");
    }
}



pub struct SaxonRSxqueryBuilder {
    saxonrs: SaxonRS,
    prop_len: i32,  // Property lenght (Number of defined properties)
    prop_cap: i32,  // property capacity (capacidad total de propedades a definir)
}

impl SaxonRSxqueryBuilder {


    fn set_property(&mut self, key: &str, value: Option<&str>) -> &mut Self {

        let value_cstring; 
        let value = match value {
            Some(val) => {
                value_cstring = CString::new(val).unwrap();
                value_cstring.as_ptr()
            }
            None => ptr::null(),
        };

        unsafe {
            setProperty(
                &mut self.saxonrs.properties, 
                &mut self.prop_len, 
                &mut self.prop_cap, 
                CString::new(key).unwrap().as_ptr(), 
                value);
        }
        self
    }


    pub fn query_string(&mut self, query: &str) -> &mut Self {
        self.set_property("qs", Some(query))
    }

    pub fn build(self) -> SaxonRS {
        self.saxonrs
    }




    /// Use the **xml-stylesheet processing instruction** in the source document to identify the 
    /// stylesheet to be used. The stylesheet argument must not be present on the builder. 
    /// 
    /// For more details see [Using the xml_stylesheet_instruction](https://www.saxonica.com/documentation12/index.html#!using-xsl/commandline/aoption)
    ///
    /// *Same as `-a[:(on|off)]` option in the command line*
    pub fn use_xml_stylesheet_instruction(&mut self ) -> &mut Self {
        self.set_property("a", Some("on"))
    }

    /// `filenames` is either a file name or a list of file names separated by semicolons; 
    /// the files are OASIS XML catalogs used to define how public identifiers and system 
    /// identifiers (URIs) used in a source document, stylesheet, or schema are to be redirected,
    /// typically to resources available locally. 
    /// For more details see [Using XML catalogs](https://www.saxonica.com/documentation12/index.html#!sourcedocs/resources-and-uris/xml-catalogs) 
    ///
    /// *Same as `-catalog:filenames` option in the command line*
    pub fn catalog(&mut self, filenames: &str) -> &mut Self {
        self.set_property("catalog", Some(filenames))
    }

    /// Indicates that configuration information should be taken from the 
    /// supplied `filename` configuration file. Any options supplied on the 
    /// builder override options specified in the configuration file.
    /// 
    /// *Same as `-config:filename` option in the command line*
    pub fn config(&mut self, filename: &str) -> &mut Self {
        self.set_property("config", Some(filename))
    }


    /// Identifies the source file or directory. Mandatory unless the **`initial_named_template()`** method is used. 
    /// The source file is parsed to create a tree, and the document node of this tree acts as the initial context 
    /// item for the transformation.
    ///
    /// If the name identifies a directory, all the files in the directory will be processed individually. 
    /// In this case the **`output()`** method is mandatory, and must also identify a directory, to contain the 
    /// corresponding output files. A directory must be specified as a filename, not as a URL.
    /// 
    /// *Same as `-s:filename` option in the command line*
    pub fn source(&mut self, filename: &str) -> &mut Self {
        self.set_property("s", Some(filename))
    }


    /// Selects the initial named template to be executed. If this is namespaced, it can be written 
    /// as {uri}localname. If the template name is omitted, the default is xsl:initial-template. 
    /// When this option is used, you do not need to supply a source file, but if you do, you must 
    /// supply it using the **`source()`** method.
    /// 
    /// *Same as `-it[:template-name]` option in the command line*
    pub fn initial_named_template(&mut self, templatename: Option<&str>) -> &mut Self {
        self.set_property("it", templatename)
    }



}