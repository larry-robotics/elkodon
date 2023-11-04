#!/bin/bash

cd $(git rev-parse --show-toplevel)

RET_VAL=0

check_license_header() {
    FILES=$(find . -type f -iwholename "${FILE_SUFFIX}" )
    CHECK_LICENSE="${COMMENT_SYMBOL} SPDX-License-Identifier: Apache-2.0"

    for FILE in $FILES
    do
        FIRST_LINE=$(head -n 1 $FILE)
        SECOND_LINE=$(head -n 2 $FILE | tail -n 1)

        if [[ "$FIRST_LINE" != "$CHECK_LICENSE" ]]
        then
            echo "$FILE :: missing license header \"$CHECK_LICENSE\""
            RET_VAL=1
        fi

        if [[ "$SECOND_LINE" != "" ]]
        then
            echo "$FILE :: missing new line after license header"
            RET_VAL=1
        fi
    done
}

check_rust() {
    FILE_SUFFIX="*.rs"
    COMMENT_SYMBOL="//"
    check_license_header
}

check_shell() {
    FILE_SUFFIX="*.sh"
    COMMENT_SYMBOL="#"
    check_license_header
}

check_toml() {
    FILE_SUFFIX="*.toml"
    COMMENT_SYMBOL="#"
    check_license_header
}

check_rust
check_shell
check_toml

exit $RET_VAL
