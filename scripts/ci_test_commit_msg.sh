#!/bin/bash

RET_VAL=0
IFS=$'\n'
for LINE in $(git shortlog | sed -n "s/      \(.*\)/\1/p" | grep -v "Merge pull request" | grep -v "Merge branch" | grep -v "Merge remote-tracking branch" )
do
    if [[ $(echo $LINE | grep -Ev "\[#[0-9]*\]" | wc -l) != "0" ]]
    then
        echo "Every commit message must start with [#???] where ??? corresponds to the issue number."
        echo "\"$LINE\" violates the commit message format"
        RET_VAL=1
    fi

    if [[ $(echo $LINE | sed -n "s/\[#[0-9]*\]\ \(.*\)/\1/p") == "" ]]
    then
        echo "Empty commit messages are not allowed, this commit message has no content after the issue number prefix."
        echo "\"$LINE\" violates the commit message format"
        RET_VAL=1
    fi
done

exit $RET_VAL
