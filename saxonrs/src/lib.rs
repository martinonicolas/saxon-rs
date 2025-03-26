use std::{ffi::{c_char, CStr, CString}, ops::{Deref, DerefMut}, ptr};

use saxonrs_sys::bindings::*;

pub struct SaxonrsOLD {
    environi: *mut sxnc_environment,
    processor: *mut sxnc_processor,
    parameters: *mut sxnc_parameter,
    properties: *mut sxnc_property,
    parameters_capacity: i32,
    properties_capacity: i32,
}

impl SaxonrsOLD {
    pub fn xquery() -> SaxonrsXqueryBuilder {
        SaxonrsXqueryBuilder {
            properties: std::ptr::null_mut(),
            prop_len: 0,
            prop_cap: 10,
        }
    }



    // pub fn new() -> Self {
    //     // Array of paramaters used for the transformation as (string, value) pairs
    //     let mut parameters: *mut sxnc_parameter = ptr::null_mut();
        
    //     let cap: i32 = 10;
        
    //     let par_cap:i32 = cap;

    //     // Array of properties used for the transformation as (string, string) pairs
    //     let mut properties: *mut sxnc_property = ptr::null_mut();

    //     let prop_cap = cap;

    //     let mut environi: *mut sxnc_environment = ptr::null_mut();
    //     let mut processor: *mut sxnc_processor = ptr::null_mut();
        
    //     unsafe { 
    //         initSaxonc(&mut environi, &mut processor, &mut parameters, &mut properties, par_cap, prop_cap);
    //         create_graalvm_isolate(environi);
    //         c_createSaxonProcessor(environi, processor, 0);
    //     }

    //     SaxonRS { 
    //         environi, 
    //         processor, 
    //         parameters, 
    //         properties, 
    //         parameters_capacity: par_cap, 
    //         properties_capacity: prop_cap,  
    //     }
    // }  
}

impl Drop for SaxonrsOLD {
    fn drop(&mut self) {
        
        unsafe {
            graal_tear_down((*self.environi).thread);
            freeSaxonc(&mut self.environi, &mut self.processor, &mut self.parameters, &mut self.properties);
        }
       println!("SaxonRS Dropped!");
    }
}



pub struct SaxonrsXqueryBuilder {
    /// Array of properties used for the transformation as (string, string) pairs
    properties: *mut sxnc_property,
    prop_len: i32,  // Property lenght (Number of defined properties)
    prop_cap: i32,  // property capacity (capacidad total de propedades a definir)
}

impl SaxonrsXqueryBuilder {


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
                &mut self.properties, 
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


    pub fn build(mut self) -> SaxonrsOLD {
        
        let mut saxon = SaxonrsOLD { 
            environi: std::ptr::null_mut(), 
            processor: std::ptr::null_mut(),
            parameters: std::ptr::null_mut(), 
            properties: self.properties, 
            parameters_capacity: 10, 
            properties_capacity: self.prop_cap,  
        };
        
        
        
        unsafe { 
            initSaxonc(&mut saxon.environi, &mut saxon.processor, &mut saxon.parameters, &mut self.properties, 0, self.prop_cap);
            create_graalvm_isolate(saxon.environi);
            c_createSaxonProcessor(saxon.environi, saxon.processor, 0);
        }

        saxon
        
    }

}


#[derive(Debug)]
pub struct SaxonRS {
    environi: *mut sxnc_environment,
    processor: *mut sxnc_processor,
    parameters: *mut sxnc_parameter,
    properties: *mut sxnc_property,
    prop_hdl: Vec<(CString,Option<CString>)>,
    par_cap: i32,
    
    prop_cap: i32,
    prop_len: i32,
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
impl SaxonRS {
    pub fn new<'a>() -> Self {
        let cwd_pbuf = std::env::current_dir().unwrap();
        let cwd = cwd_pbuf.to_str().unwrap();

        let cap: i32 = 10;

        // Array of paramaters used for the transformation as (string, value) pairs
        let mut parameters: *mut sxnc_parameter = ptr::null_mut();

        let par_len:i32 = 0;
        let par_cap:i32 = cap;

        // Array of properties used for the transformation as (string, string) pairs
        let mut properties: *mut sxnc_property = ptr::null_mut();

        let mut prop_len:i32 = 0;
        let mut prop_cap = cap;

        let mut environi: *mut sxnc_environment = ptr::null_mut();
        let mut processor: *mut sxnc_processor = ptr::null_mut();
        
        unsafe {
            initSaxonc(&mut environi, &mut processor, &mut parameters, &mut properties, par_cap, prop_cap);
            
            // Initialize Graalvm run-time.
            // The handle of loaded component is used to retrieve Invocation API.
            create_graalvm_isolate(environi);
            c_createSaxonProcessor(environi, processor, 0);
        }


        Self { 
            environi, 
            processor, 
            parameters, 
            properties, 
            par_cap, 
            prop_cap,
            prop_hdl: Vec::new(),
            prop_len,
        }

    }
    pub fn xslt<'a>(&'a mut self) -> XsltBuilder<'a> {
        XsltBuilder {
            saxonrs: self,
            prop_refs: Vec::new(),
            stylesheet: None,
            source: None,
        }
    }


    fn set_property(&mut self, key: CString, value: Option<CString>) {
        self.prop_hdl.push((key,value));
    }
}

#[derive(Debug)]
pub struct XsltBuilder<'a> {
    saxonrs: &'a mut SaxonRS,
    prop_refs: Vec<(&'a str, Option<&'a str>)>,
    stylesheet: Option<&'a str>,
    source: Option<&'a str>,
}
// impl<'a> Deref for XsltBuilder<'a> {
//     type Target = SaxonBuilder;

//     fn deref(&self) -> &Self::Target {
//         self.0
//     }
// }
// impl<'a> DerefMut for XsltBuilder<'a> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0 // Accedemos a la referencia mutable contenida en XsltBuilder
//     }
// }
impl<'a> XsltBuilder<'a> {

    
    // fn set_property(&mut self, key: &str, value: Option<&'a str>) -> &mut Self {

    //     //let value_cstring; 
    //     let value = match value {
    //         Some(val) => {
    //             CStr::from_ptr(val.as_ptr())
    //             // value_cstring = CString::new(val).unwrap();
    //             // value_cstring.as_ptr()
    //         }
    //         None => ptr::null(),
    //     };

    //     unsafe {
    //         setProperty(
    //             &mut self.properties, 
    //             &mut self.prop_len, 
    //             &mut self.prop_cap, 
    //             CString::new(key).unwrap().as_ptr(), 
    //             value);
    //     }
    //     self
    // }

    pub fn source(mut self, filename: &'a str) -> Self {
        // self.prop_refs.push(("s", Some(filename)));
        self.source = Some(filename);
        self
    }
    
    pub fn stylesheet(mut self, filename: &'a str) -> Self {
        self.stylesheet = Some(filename);
        self
    }



    pub fn run(self) -> String {

        for prop in self.prop_refs {
            let key = CString::new(prop.0).unwrap();

            let value = match prop.1 {
                Some(v) => Some(CString::new(v).unwrap()),
                None => None
            };
            self.saxonrs.set_property(key, value);
        }

        // FOR TEST
        for c_prop in &self.saxonrs.prop_hdl {
            println!("{:?}",c_prop);
        }



        // Set properties
         for (key, value) in &self.saxonrs.prop_hdl {
            unsafe {
                setProperty(
                    &mut self.saxonrs.properties, 
                    &mut self.saxonrs.prop_len, 
                    &mut self.saxonrs.prop_cap, 
                    key.as_ptr(), 
                    value.as_ref().map_or_else(|| std::ptr::null(), |v| v.as_ptr()));
            }
        }


        let cwd_pbuf = std::env::current_dir().unwrap();
        let cwd = cwd_pbuf.to_str().unwrap();


        let stylesheet = match self.stylesheet {
            Some(v) => CString::new(v).unwrap(),
            None => panic!("NO SE ESPECIFICO STYLESHEET")
        };

        let source = match self.source {
            Some(v) => CString::new(v).unwrap(),
            None => panic!("NO SE ESPECIFICO STYLESHEET")
        };

        unsafe {
            let result = xsltApplyStylesheet(
                self.saxonrs.environi, 
                self.saxonrs.processor, 
                CString::new(cwd).unwrap().into_raw(), 
                source.into_raw(),
                stylesheet.into_raw(), 
                ptr::null_mut(), 
                ptr::null_mut(), 
                0, 0);
            

            if result.is_null() {
                let c_message: *const c_char = c_getErrorMessage(self.saxonrs.environi);
                let msg = CStr::from_ptr(c_message).to_string_lossy().into_owned();

                graal_tear_down((*self.saxonrs.environi).thread);
                freeSaxonc(&mut self.saxonrs.environi, &mut self.saxonrs.processor, &mut self.saxonrs.parameters, &mut self.saxonrs.properties);
                
                panic!("Failed on xsltApplyStylesheet function.\n{msg}")
            } 

            let string_result = CStr::from_ptr(result).to_string_lossy().into_owned();
          
            println!("{:?}",string_result);
        }   


        "done".to_string()
        
    }

}

// pub struct XpathBuilder;
// impl XpathBuilder {
//     pub fn build(self ) -> SaxonBuilder {
//         SaxonBuilder
//     }
// }
