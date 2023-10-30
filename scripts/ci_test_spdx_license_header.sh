#!/bin/bash

cd $(git rev-parse --show-toplevel)

RET_VAL=0
FILES=$(find . -iwholename "*.rs")
CHECK_LICENSE="// SPDX-License-Identifier: Apache-2.0"

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

exit $RET_VAL
