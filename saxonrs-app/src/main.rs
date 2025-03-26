use saxonrs::{SaxonRS, XsltBuilder};

fn main() {
    println!("Hello, world!");


    println!("SaxonRS, uninitialized");
   
    {
        // let build = Saxonrs::xquery()
        //     .source("saxonrs-sys/libsaxonc/sample/data/cat.xml")
        //     .query_string("<out>{count(/out/person)}</out>")
        //     .build();

        

        
        let saxon = SaxonRS::new()
            .xslt()
            .source("saxonrs-sys/libsaxonc/samples/data/cat.xml")
            .stylesheet("saxonrs-sys/libsaxonc/samples/data/test.xsl")
            .run();
    }


    




    println!("Bye, world!");

}
