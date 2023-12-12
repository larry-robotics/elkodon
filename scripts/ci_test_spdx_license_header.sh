# Copyright (c) 2023 Contributors to the Eclipse Foundation
#
# See the NOTICE file(s) distributed with this work for additional
# information regarding copyright ownership.
#
# This program and the accompanying materials are made available under the
# terms of the Apache Software License 2.0 which is available at
# https://www.apache.org/licenses/LICENSE-2.0, or the MIT license
# which is available at https://opensource.org/licenses/MIT.
#
# SPDX-License-Identifier: Apache-2.0 OR MIT

#!/bin/bash

cd $(git rev-parse --show-toplevel)

REQUIRED_LICENSE_HEADER="SPDX-License-Identifier: Apache-2.0"
RET_VAL=0

check_license_header() {
    FILES=$(find . -type f -iwholename "${FILE_SUFFIX}" )
    CHECK_LICENSE="${COMMENT_SYMBOL} ${REQUIRED_LICENSE_HEADER}"

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

# no toml check for now
# it is usually only some configuration files which can be used without copyright notice
# check_toml

exit $RET_VAL
