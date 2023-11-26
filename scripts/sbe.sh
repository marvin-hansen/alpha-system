# bin/bash
set -o errexit
set -o nounset
set -o pipefail

# Define the path to the target file that we're going to check
TARGET_FILE="sbe_hashes.md5"
TARGET_FOLDER="sbe/rust"

# Check if Java is working
# Java is assumed to be installed with SDMMAN
# https://www.andrewhoog.com/post/3-ways-to-install-java-on-macos-2023/#install-java-with-sdkman-1
command java --version >/dev/null 2>&1 || {
#  If not, source it.
    source "$HOME/.sdkman/bin/sdkman-init.sh"
}

# Check if if the SBE schema has changed and if so, regenerate the Rust bindings
function check_hashes() {

        for file in sbe/schema/*; do md5 -q $file; done > sbe_hashes_current.txt

        md5 -q sbe_hashes_current.txt > sbe_hashes_curent.md5

        if cmp -s sbe_hashes.md5 sbe_hashes_curent.md5;
        then
          echo "SBE schema unchanged."

        else
          echo "SBE Schema change detected. Regenerating Rusting bindings"
          regenerate_bindings
        fi
}

# Generate Rusting bindings and generate a md5 hash of the underlying schema.
function regenerate_bindings() {

#         https://github.com/real-logic/simple-binary-encoding?tab=readme-ov-file
        command java -Dsbe.generate.ir=true -Dsbe.target.language=Rust -Dsbe.target.namespace=sbe -Dsbe.output.dir=sbe -Dsbe.errorLog=yes -jar tools/sbe-all-1.30.0-SNAPSHOT.jar sbe/schema/schema.xml

        for file in sbe/schema/*; do md5 -q $file; done > sbe_hashes.txt

        md5 -q sbe_hashes.txt > sbe_hashes.md5

        rm sbe_hashes.txt
}

if test -f "$TARGET_FOLDER";
then
#  Folder exists, nothing to do
#  echo "TARGET_FOLDER already exists"
  echo ""
else
#  Folder does not exist, create it by initial generating the bindings
#  echo "TARGET_FOLDER does not exist, generating Rust bindings"
#  echo ""
  regenerate_bindings
fi

# Check if the hash file exists to determine if the SBE Rust bindings need regeneration.
if test -f "$TARGET_FILE";
then
#   echo "$TARGET_FILE exists."
    check_hashes
else
    echo "$TARGET_FILE does not exist."
    exit 0
fi

# Clean up temporary files
rm sbe_hashes_current.txt
rm sbe_hashes_curent.md5

exit 0
