//! Rust bindings to the SaxonC library
//!

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
pub mod bindings{
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}


#[cfg(test)]
mod tests {
    use std::{alloc::{alloc, Layout}, ffi::{c_char, CStr, CString}, ptr};

    use crate::bindings::*;

    #[test]
    fn test_xpath() {

        unsafe {

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
            
            initSaxonc(&mut environi, &mut processor, &mut parameters, &mut properties, par_cap, prop_cap);
    
            /*
            * Initialize Graalvm run-time.
            * The handle of loaded component is used to retrieve Invocation API.
            */
                        
            create_graalvm_isolate(environi);
    
            c_createSaxonProcessor(environi, processor, 0);
    
            // Dynamically allocates memory for a sxnc_xpath structure using alloc(...) and 
            // assigns the allocated memory's pointer to xpath_proc
            let layout = Layout::new::<sxnc_xpath>();
            let xpath_proc: *mut sxnc_xpath = alloc(layout) as *mut sxnc_xpath;

            let check_proc = c_createXPathProcessor(environi, processor, xpath_proc);

            // Check XPath Processor
            if check_proc != 1 {
                let c_message: *const c_char = c_getErrorMessage(environi);
                let msg = CStr::from_ptr(c_message).to_string_lossy().into_owned();

                graal_tear_down((*environi).thread);
                freeSaxonc(&mut environi, &mut processor, &mut parameters, &mut properties);
                
                panic!("Failed to create SaxonC XPath Processor:\n{msg}")
            }

            setProperty(
                &mut properties, 
                &mut prop_len, 
                &mut prop_cap, 
                CString::new("s").unwrap().as_ptr(), 
                CString::new("libsaxonc/samples/data/cat.xml").unwrap().as_ptr());


            let result = evaluate(
                environi, 
                xpath_proc, 
                CString::new(cwd).unwrap().into_raw(), 
                CString::new("/out/person").unwrap().into_raw(), 
                ptr::null_mut(),
                parameters, 
                properties, 
                0, 
                prop_len);
            
            let cstr = CStr::from_ptr(getStringValue(environi, *result));
            let string_value = String::from_utf8_lossy(cstr.to_bytes()).to_string();

            assert_eq!(
                string_value,
                "<person>text1</person>\n\n<person>text2</person>\n\n<person>text3</person>",
                "Unexpected result for XPath: `/out/person`");


            let result_bool = effectiveBooleanValue(
                environi, 
                xpath_proc, 
                CString::new(cwd).unwrap().into_raw(), 
                CString::new("count(/out/person)>0").unwrap().into_raw(), 
                ptr::null_mut(),
                parameters, 
                properties, 
                par_len, 
                prop_len);
            
            if !result_bool {
                let c_message: *const c_char = c_getErrorMessage(environi);
                let msg = CStr::from_ptr(c_message).to_string_lossy().into_owned();

                graal_tear_down((*environi).thread);
                freeSaxonc(&mut environi, &mut processor, &mut parameters, &mut properties);
                
                panic!("Unexpected Boolean result: false.\n{msg}")
            }

            graal_tear_down((*environi).thread);
            freeSaxonc(&mut environi, &mut processor, &mut parameters, &mut properties);
        }
    }

    #[test]
    fn test_xquery() {

        unsafe {

            let cwd_pbuf = std::env::current_dir().unwrap();
            let cwd = cwd_pbuf.to_str().unwrap();

            let cap: i32 = 10;
    
            // Array of paramaters used for the transformation as (string, value) pairs
            let mut parameters: *mut sxnc_parameter = ptr::null_mut();
    
            let par_cap:i32 = cap;

            // Array of properties used for the transformation as (string, string) pairs
            let mut properties: *mut sxnc_property = ptr::null_mut();
    
            let mut prop_len:i32 = 0;
            let mut prop_cap = cap;
    
            let mut environi: *mut sxnc_environment = ptr::null_mut();
            let mut processor: *mut sxnc_processor = ptr::null_mut();
            
            initSaxonc(&mut environi, &mut processor, &mut parameters, &mut properties, par_cap, prop_cap);
    
            /*
            * Initialize Graalvm run-time.
            * The handle of loaded component is used to retrieve Invocation API.
            */
                        
            create_graalvm_isolate(environi);
    
            c_createSaxonProcessor(environi, processor, 0);
    
            // Dynamically allocates memory for a sxnc_xpath structure using alloc(...) and 
            // assigns the allocated memory's pointer to xpath_proc
            let layout = Layout::new::<sxnc_xpath>();
            let xpath_proc: *mut sxnc_xpath = alloc(layout) as *mut sxnc_xpath;

            let check_proc = c_createXPathProcessor(environi, processor, xpath_proc);

            // Check XPath Processor
            if check_proc != 1 {
                let c_message: *const c_char = c_getErrorMessage(environi);
                let msg = CStr::from_ptr(c_message).to_string_lossy().into_owned();

                graal_tear_down((*environi).thread);
                freeSaxonc(&mut environi, &mut processor, &mut parameters, &mut properties);
                
                panic!("Failed to create SaxonC XPath Processor:\n{msg}")
            }

            setProperty(
                &mut properties, 
                &mut prop_len, 
                &mut prop_cap, 
                CString::new("s").unwrap().into_raw(),
                CString::new("libsaxonc/samples/data/cat.xml").unwrap().into_raw());

            setProperty(
                &mut properties, 
                &mut prop_len, 
                &mut prop_cap, 
                CString::new("cwd").unwrap().into_raw(), 
                CString::new(cwd).unwrap().into_raw());

            setProperty(
                &mut properties, 
                &mut prop_len, 
                &mut prop_cap, 
                CString::new("qs").unwrap().into_raw(),
                CString::new("<out>{count(/out/person)}</out>").unwrap().into_raw());


            let result = executeQueryToString(
                environi, 
                processor, 
                CString::new(cwd).unwrap().into_raw(), 
                ptr::null_mut(), 
                properties, 
                0,
                prop_len);

            if result.is_null() {
                let c_message: *const c_char = c_getErrorMessage(environi);
                let msg = CStr::from_ptr(c_message).to_string_lossy().into_owned();

                graal_tear_down((*environi).thread);
                freeSaxonc(&mut environi, &mut processor, &mut parameters, &mut properties);
                
                panic!("Failed on executeQueryToString function.\n{msg}")
            } 
            
            let string_result = CStr::from_ptr(result).to_string_lossy().into_owned();

            graal_tear_down((*environi).thread);
            freeSaxonc(&mut environi, &mut processor, &mut parameters, &mut properties);

            let expected_result = r#"<?xml version="1.0" encoding="UTF-8"?><out>3</out>"#;
           
            assert_eq!(string_result,expected_result,"Unexpected result value from executeQueryToString function.")
            
        }
    }

    #[test]
    /// This function test XSLT Transformation. It use `libsaxonc/samples/data/cat.xml` as source file and
    /// `libsaxonc/samples/data/test.xsl` as the stylesheet to apply
    /// 
    /// SaxonC Handle 3 types of stylesheet outputs
    /// - The principal output.
    /// - Secondary outputs produced by use of `xsl:result-document`.
    /// - Messages produced using `xsl:message`.
    /// 
    /// For this test it's important to mention the presence of the `xsl:message` instruction in the 
    /// `libsaxonc/samples/data/test.xsl` stylesheet file.
    /// For that reason you will see `Testing message2` in the stderr output when you run this test.
    /// 
    /// Current SaxonC API do not have a way for handling `xsl:message` output so by default the values inside 
    /// ´xsl:message` are writen to stderr.
    /// 
    /// For more information about this, see: [SaxonC - Using XSLT - Handling output](https://www.saxonica.com/documentation12/index.html#!using-xsl/stylesheet-output)
    /// 
    fn test_xslt() {

        unsafe {

            let cwd_pbuf = std::env::current_dir().unwrap();
            let cwd = cwd_pbuf.to_str().unwrap();

            let cap: i32 = 10;
    
            // Array of paramaters used for the transformation as (string, value) pairs
            let mut parameters: *mut sxnc_parameter = ptr::null_mut();
    
            let par_cap:i32 = cap;
            let prop_cap = cap;

            // Array of properties used for the transformation as (string, string) pairs
            let mut properties: *mut sxnc_property = ptr::null_mut();
    
            let mut environi: *mut sxnc_environment = ptr::null_mut();
            let mut processor: *mut sxnc_processor = ptr::null_mut();
            
            initSaxonc(&mut environi, &mut processor, &mut parameters, &mut properties, par_cap, prop_cap);
    
            /*
            * Initialize Graalvm run-time.
            * The handle of loaded component is used to retrieve Invocation API.
            */
                        
            create_graalvm_isolate(environi);
    
            c_createSaxonProcessor(environi, processor, 0);

            let result = xsltApplyStylesheet(
                environi, 
                processor, 
                CString::new(cwd).unwrap().into_raw(), 
                CString::new("libsaxonc/samples/data/cat.xml").unwrap().into_raw(),
                CString::new("libsaxonc/samples/data/test.xsl").unwrap().into_raw(), 
                ptr::null_mut(), 
                ptr::null_mut(), 
                0, 0);

            if result.is_null() {
                let c_message: *const c_char = c_getErrorMessage(environi);
                let msg = CStr::from_ptr(c_message).to_string_lossy().into_owned();

                graal_tear_down((*environi).thread);
                freeSaxonc(&mut environi, &mut processor, &mut parameters, &mut properties);
                
                panic!("Failed on xsltApplyStylesheet function.\n{msg}")
            } 

            let string_result = CStr::from_ptr(result).to_string_lossy().into_owned();
            
            graal_tear_down((*environi).thread);
            freeSaxonc(&mut environi, &mut processor, &mut parameters, &mut properties);

            let expected_result = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<output>\n   <out>text1</out>\n   <out>text2</out>\n   <out>text3</out>\n   <out>4</out>\n</output>\n";

            assert_eq!(string_result,expected_result,"Unexpected result value from xsltApplyStylesheet funcion.");
            assert_eq!(1,1);
        }
    }
}
