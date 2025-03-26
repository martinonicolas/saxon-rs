#!/bin/bash
# Require Bash 4+
# Download SaxonC v12.5.0  
# and place it in the appropriate directory to build saxonrs-sys crate

edition=""
if [ x$1 = x ]; then
  echo "Usage: $0 <saxon-he|saxon-pe|saxon-ee>"
  exit 1
fi

case "$1" in
    saxon-he) edition=HE ;;
    saxon-pe) edition=PE ;;
    saxon-ee) edition=EE ;;
    *) 
        echo "Usage: $0 [saxon-he|saxon-pe|saxon-ee]"
        exit 1
    ;;
esac


if [ -d "saxonrs-sys/libsaxonc-${edition,,}" ]; then
    echo "Directory saxonrs-sys/libsaxonc-${edition,,} already exists"
    exit
fi


echo Downloading SaxonC-$edition v12.5.0 ...

curl -s -o tmp.zip "https://downloads.saxonica.com/SaxonC/$edition/12/libsaxon-${edition}C-linux-x86_64-v12.5.0.zip" && \
unzip -q tmp.zip && \
mv libsaxon-${edition}C-linux-amd64-v12.5.0 saxonrs-sys/libsaxonc-${edition,,} && \
rm -f tmp.zip

echo "Done. You can now run \`cargo build\`"