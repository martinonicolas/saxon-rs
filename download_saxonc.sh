#!/bin/bash
# Download SaxonC Home Edition 12.5.0  
# Place it in the appropriate directory to build saxonrs-sys crate

curl -o tmp.zip "https://downloads.saxonica.com/SaxonC/HE/12/libsaxon-HEC-linux-x86_64-v12.5.0.zip" && \
unzip tmp.zip && \
mv libsaxon-HEC-linux-amd64-v12.5.0 saxonrs-sys/libsaxonc && \
rm -f tmp.zip
