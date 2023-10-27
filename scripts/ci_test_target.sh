#!/bin/bash

# on freebsd to be able to work with message queues
#  kldload mqueuefs
#  mount -t mqueuefs null /mnt/mqueue

CRATES="
    elkodon_bb_lock_free
    elkodon_bb_threadsafe
    elkodon_bb_container
    elkodon_bb_elementary
    elkodon_bb_log
    elkodon_bb_memory
    elkodon_bb_posix
    elkodon_bb_system_types
    elkodon_bb_testing

    elkodon_cal
    elkodon

    elkodon_pal_posix
    elkodon_pal_settings
    elkodon_pal_concurrency_primitives

    example_publish_subscribe
    example_event
    example_discovery

    benchmark_publish_subscribe"

HAS_FAILED=0

function build() {
    cargo build -p $CRATE

    if [[ $? -ne  0 ]]
    then
        echo "FAILED to build: $CRATE"
        HAS_FAILED=1
        exit $HAS_FAILED
    fi
}

function test() {
    cargo test -p $CRATE -- --test-threads=1

    if [[ $? -ne  0 ]]
    then
        echo "FAILED test in: $CRATE"
        HAS_FAILED=1
        exit $HAS_FAILED
    fi
}

function doc_test() {
    cargo test --doc -p $CRATE

    if [[ $? -ne  0 ]]
    then
        echo "FAILED doc test in: $CRATE"
        HAS_FAILED=1
        exit $HAS_FAILED
    fi
}

cd $(git rev-parse --show-toplevel)

echo Install required packages
apt -y update
apt -y install libacl1-dev

echo Install some test users and test groups
useradd testuser1
useradd testuser2
groupadd testgroup1
groupadd testgroup2

for CRATE in $CRATES
do
    build
    test
    doc_test
done

exit $HAS_FAILED
