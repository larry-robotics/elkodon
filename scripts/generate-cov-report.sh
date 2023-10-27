#!/bin/bash

cd $(git rev-parse --show-toplevel)

CLEAN=0
GENERATE=0
REPORT=0
OVERVIEW=0
HTML=0
LCOV=0

cleanup() {
    find . -name "*profraw" -exec rm {} \;
    rm json5format.profdata
}

generate_profile() {
    RUSTFLAGS="-C instrument-coverage" cargo test --workspace -- --test-threads=1
}

merge_report() {
    local FILES=$(find . -name "*profraw")
    llvm-profdata merge -sparse $FILES -o json5format.profdata
}

generate() {
    cleanup
    generate_profile
    merge_report
}

show_overview() {
    local FILES=$(ls ./target/debug/deps/*tests* | grep -v ".d\$")
    CMD="llvm-cov report --use-color --ignore-filename-regex='/.cargo/registry' --instr-profile=json5format.profdata"

    for FILE in $FILES 
    do
        CMD="$CMD --object $FILE"
    done

    eval $CMD
}

show_report() {
    CMD="llvm-cov report --use-color --ignore-filename-regex='/.cargo/registry' --instr-profile=json5format.profdata"

    for FILE in $FILES 
    do
        CMD="$CMD --object $FILE"
    done
    CMD="$CMD --show-instantiations --show-line-counts-or-regions --Xdemangler=rustfilt | less -R"

    eval $CMD
}

generate_html_report() {
    mkdir -p ./target/coverage/html/
    grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/html
}

generate_lcov_report() {
    mkdir -p ./target/coverage/
    grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/tests.lcov
}



show_help() {
    echo "$0 [OPTIONS]"
    echo
    echo "-c|--clean                -   cleanup all reports"
    echo "-g|--generate             -   generate coverage report"
    echo "-o|--overview             -   show overview of coverage report"
    echo "-r|--report               -   show detailed report"
    echo "-l|--lcov                 -   creates lcov report"
    echo "-t|--html                 -   creates html report"
    echo "-f|--full                 -   generate coverage report and create html and lcov"
    echo
    exit 1
}

if [[ $# == 0 ]]; then
    show_help
fi

while [[ $# -gt 0 ]]; do
    case $1 in
        -c|--clean)
            CLEAN=1
            shift
            ;;
        -g|--generate)
            GENERATE=1
            shift
            ;;
        -o|--overview)
            OVERVIEW=1
            shift
            ;;
        -r|--report)
            REPORT=1
            shift
            ;;
        -f|--full)
            GENERATE=1
            LCOV=1
            HTML=1
            shift
            ;;
        -l|--lcov)
            LCOV=1
            shift
            ;;
        -t|--html)
            HTML=1
            shift
            ;;
        *)
            show_help
            ;;
    esac
done

if [[ $CLEAN == "1" ]]; then
    cleanup
fi

if [[ $GENERATE == "1" ]]; then
    generate
fi

if [[ $OVERVIEW == "1" ]]; then
    show_overview
fi

if [[ $REPORT == "1" ]]; then
    show_report
fi

if [[ $LCOV == "1" ]]; then
    generate_lcov_report
fi

if [[ $HTML == "1" ]]; then
    generate_html_report
fi
