#!/bin/bash

cd $(git rev-parse --show-toplevel)

LICENSE_PREFIX="SPDX-License-Identifier:"
LICENSE=$1

set_license_header() {
    FILES=$(find . -type f -iwholename "${FILE_SUFFIX}" )

    for FILE in $FILES
    do
        HAS_LICENSE_IDENTIFIER=$(cat $FILE | head -n 1 | grep -E "^${COMMENT_SYMBOL} ${LICENSE_PREFIX}" | wc -l)
        if [[ "$HAS_LICENSE_IDENTIFIER" == "1" ]]
        then
            sed -i "1c ${COMMENT_SYMBOL} ${LICENSE_PREFIX} ${LICENSE}" $FILE
        else
            sed -i "1i ${COMMENT_SYMBOL} ${LICENSE_PREFIX} ${LICENSE}" $FILE
        fi

        HAS_NEW_LINE=$(cat $FILE | head -n 2 | tail -n 1 | grep -E "^[ ]*\$" | wc -l)
        if [[ "$HAS_NEW_LINE" == "0" ]]
        then
            sed -i '2i\
' $FILE
        fi
    done
}

set_rust() {
    FILE_SUFFIX="*.rs"
    COMMENT_SYMBOL="//"
    set_license_header
}

set_shell() {
    FILE_SUFFIX="*.sh"
    COMMENT_SYMBOL="#"
    set_license_header
}

set_toml() {
    FILE_SUFFIX="*.toml"
    COMMENT_SYMBOL="#"
    set_license_header
}

if [[ -z $1 ]]
then
    echo
    echo Usage: $0 LICENSE_SPDX_IDENTIFIER
    echo
    exit 1
fi

echo Setting license to: $LICENSE

set_rust
set_shell
set_toml

