#!/bin/bash
# run in repo directory

DTS_FILE="pkg/base9_builder.d.ts"

function fix_type() {
    echo "Changing \"$1\" to return \"$2\""
    sed -i "s/\(function $1(.*\)any;$/\1$2;/g" $DTS_FILE
}

echo "Monkey patching $DTS_FILE with custom return types"

fix_type "getData" "Data"
fix_type "getColors" "Colors<string>"

cargo run render - templates/wasm_typescript_types.mustache >> $DTS_FILE