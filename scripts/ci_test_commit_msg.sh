#!/bin/bash

git shortlog | sed -n "s/      \(.*\)/\1/p" | grep -v "Merge pull request" | grep -v "Merge branch" | grep -v "Merge remote-tracking branch" | grep -Ev "\[#[0-9]*\]" | wc -l
