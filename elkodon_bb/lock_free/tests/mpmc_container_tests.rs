use elkodon_bb_elementary::relocatable_container::RelocatableContainer;
use elkodon_bb_lock_free::mpmc::container::*;
use elkodon_bb_lock_free::mpmc::unique_index_set::*;
use elkodon_bb_memory::bump_allocator::BumpAllocator;
use elkodon_bb_memory::memory::Memory;
use elkodon_bb_posix::system_configuration::SystemInfo;
use elkodon_bb_testing::assert_that;
use pin_init::init_stack;
use std::collections::HashMap;
use std::collections::HashSet;
use std::mem::MaybeUninit;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::{Barrier, Mutex};
use std::thread;

const CAPACITY: usize = 129;

#[test]
fn mpmc_container_add_elements_until_full_works() {
    let sut = FixedSizeContainer::<usize, CAPACITY>::new();
    let mut stored_indices = vec![];
    assert_that!(sut.capacity(), eq CAPACITY);
    for i in 0..CAPACITY {
        let index = sut.add(i * 5 + 2);
        assert_that!(index, is_some);
        stored_indices.push(index.unwrap());
    }
    let index = sut.add(0);
    assert_that!(index, is_none);

    let state = sut.get_state();
    let mut contained_values = vec![];
    state.for_each(|index: u32, value: &usize| contained_values.push((index, *value)));

    for i in 0..CAPACITY {
        assert_that!(contained_values[i].0, eq i as u32);
        assert_that!(contained_values[i].1, eq i * 5 + 2);
    }
}

#[test]
fn mpmc_container_add_and_remove_elements_works() {
    let sut = FixedSizeContainer::<usize, CAPACITY>::new();
    let mut stored_indices = vec![];
    for i in 0..CAPACITY - 1 {
        let index = sut.add(i * 3 + 1);
        assert_that!(index, is_some);
        stored_indices.push(index.unwrap());

        let index = sut.add(i * 7 + 5);
        assert_that!(index, is_some);
        stored_indices.push(index.unwrap());

        stored_indices.remove(stored_indices.len() - 2);
    }

    let state = sut.get_state();
    let mut contained_values = vec![];
    state.for_each(|_: u32, value: &usize| contained_values.push(*value));

    for i in 0..CAPACITY - 1 {
        assert_that!(contained_values[i], eq i * 7 + 5);
    }
}

#[test]
fn mpmc_container_add_and_remove_elements_works_with_uninitialized_memory() {
    init_stack!(
        memory =
            Memory::<{ Container::<usize>::const_memory_size(129_usize) }, BumpAllocator>::new_filled(
                0xff,
            )
    );
    let memory = memory.unwrap();
    let sut = unsafe { Container::new_uninit(CAPACITY) };
    unsafe { assert_that!(sut.init(memory.allocator()), is_ok) };

    let mut stored_indices = vec![];
    for i in 0..CAPACITY - 1 {
        let index = unsafe { sut.add(i * 3 + 1) };
        assert_that!(index, is_some);
        stored_indices.push(index.unwrap());

        let index = unsafe { sut.add(i * 7 + 5) };
        assert_that!(index, is_some);
        stored_indices.push(index.unwrap());

        stored_indices.remove(stored_indices.len() - 2);
    }

    let state = unsafe { sut.get_state() };
    let mut contained_values = vec![];
    state.for_each(|_: u32, value: &usize| contained_values.push(*value));

    for i in 0..CAPACITY - 1 {
        assert_that!(contained_values[i], eq i * 7 + 5);
    }
}

#[test]
fn mpmc_container_add_and_unsafe_remove_elements_works() {
    let sut = FixedSizeContainer::<usize, CAPACITY>::new();
    let mut stored_indices: Vec<MaybeUninit<UniqueIndex>> = vec![];

    for i in 0..CAPACITY - 1 {
        let index = sut.add(i * 3 + 1);
        assert_that!(index, is_some);
        stored_indices.push(MaybeUninit::new(index.unwrap()));

        let index = sut.add(i * 7 + 5);
        assert_that!(index, is_some);
        stored_indices.push(MaybeUninit::new(index.unwrap()));

        unsafe {
            sut.remove_raw_index(
                stored_indices[stored_indices.len() - 2]
                    .assume_init_ref()
                    .value(),
            )
        };
    }

    let state = sut.get_state();
    let mut contained_values = vec![];
    state.for_each(|_: u32, value: &usize| contained_values.push(*value));

    for i in 0..CAPACITY - 1 {
        assert_that!(contained_values[i], eq i * 7 + 5);
    }
}

#[test]
fn mpmc_container_state_not_updated_when_contents_do_not_change() {
    let sut = FixedSizeContainer::<usize, CAPACITY>::new();
    let mut stored_indices: Vec<UniqueIndex> = vec![];

    for i in 0..CAPACITY - 1 {
        let index = sut.add(i * 3 + 1);
        assert_that!(index, is_some);
        stored_indices.push(index.unwrap());
    }

    let mut state = sut.get_state();
    let mut contained_values1 = vec![];
    state.for_each(|_: u32, value: &usize| contained_values1.push(*value));

    assert_that!(state.update(), eq false);
    let mut contained_values2 = vec![];
    state.for_each(|_: u32, value: &usize| contained_values2.push(*value));

    for i in 0..CAPACITY - 1 {
        assert_that!(contained_values1[i], eq i * 3 + 1);
        assert_that!(contained_values2[i], eq i * 3 + 1);
    }
}

#[test]
fn mpmc_container_state_updated_when_contents_are_removed() {
    let sut = FixedSizeContainer::<usize, CAPACITY>::new();
    let mut stored_indices: Vec<UniqueIndex> = vec![];

    for i in 0..CAPACITY - 1 {
        let index = sut.add(i * 3 + 1);
        assert_that!(index, is_some);
        stored_indices.push(index.unwrap());
    }

    let mut state = sut.get_state();
    stored_indices.clear();

    assert_that!(state.update(), eq true);
    let mut contained_values = vec![];
    state.for_each(|_: u32, value: &usize| contained_values.push(*value));

    assert_that!(contained_values, is_empty);
}

#[test]
fn mpmc_container_state_updated_when_contents_are_changed() {
    let sut = FixedSizeContainer::<usize, CAPACITY>::new();
    let mut stored_indices: Vec<UniqueIndex> = vec![];

    for i in 0..CAPACITY - 1 {
        let index = sut.add(i * 3 + 1);
        assert_that!(index, is_some);
        stored_indices.push(index.unwrap());
    }

    let mut state = sut.get_state();
    stored_indices.clear();

    let mut results = HashMap::<u32, usize>::new();
    for i in 0..CAPACITY - 1 {
        let index = sut.add(i * 81 + 56);
        assert_that!(index, is_some);
        results.insert(index.as_ref().unwrap().value(), i * 81 + 56);
        stored_indices.push(index.unwrap());
    }

    assert_that!(state.update(), eq true);
    let mut contained_values = vec![];
    state.for_each(|_: u32, value: &usize| contained_values.push(*value));

    for i in 0..CAPACITY - 1 {
        assert_that!(contained_values[i], eq * results.get(&(i as u32)).unwrap());
    }
}

#[test]
fn mpmc_container_concurrent_add_release_for_each() {
    const REPETITIONS: i64 = 1000;
    let number_of_threads_per_op = (SystemInfo::NumberOfCpuCores.value() / 2).clamp(2, usize::MAX);

    let sut = FixedSizeContainer::<usize, CAPACITY>::new();
    let barrier = Barrier::new(number_of_threads_per_op * 2);
    let mut added_content: Vec<Mutex<Vec<(u32, usize)>>> = vec![];
    let mut extracted_content: Vec<Mutex<Vec<(u32, usize)>>> = vec![];

    for _ in 0..number_of_threads_per_op {
        added_content.push(Mutex::new(vec![]));
        extracted_content.push(Mutex::new(vec![]));
    }

    let finished_threads_counter = AtomicU64::new(0);
    thread::scope(|s| {
        for thread_number in 0..number_of_threads_per_op {
            let barrier = &barrier;
            let sut = &sut;
            let added_content = &added_content;
            let finished_threads_counter = &finished_threads_counter;
            s.spawn(move || {
                let mut repetition = 0;
                let mut ids = vec![];
                let mut counter = 0;

                barrier.wait();
                while repetition < REPETITIONS {
                    counter += 1;
                    let value = counter * number_of_threads_per_op + thread_number;

                    match sut.add(value) {
                        Some(index) => {
                            let index_value = index.value();
                            ids.push(index);
                            added_content[thread_number]
                                .lock()
                                .unwrap()
                                .push((index_value, value));
                        }
                        None => {
                            repetition += 1;
                            ids.clear();
                        }
                    }
                }

                finished_threads_counter.fetch_add(1, Ordering::Relaxed);
            });
        }

        for thread_number in 0..number_of_threads_per_op {
            let sut = &sut;
            let barrier = &barrier;
            let finished_threads_counter = &finished_threads_counter;
            let extracted_content = &extracted_content;
            s.spawn(move || {
                barrier.wait();

                let mut state = sut.get_state();
                while finished_threads_counter.load(Ordering::Relaxed)
                    != number_of_threads_per_op as u64
                {
                    if state.update() {
                        state.for_each(|index: u32, value: &usize| {
                            extracted_content[thread_number]
                                .lock()
                                .unwrap()
                                .push((index, *value));
                        })
                    }
                }
            });
        }
    });

    let mut added_contents_set = HashSet::<(u32, usize)>::new();

    for thread_number in 0..number_of_threads_per_op {
        for entry in &*added_content[thread_number].lock().unwrap() {
            added_contents_set.insert(*entry);
        }
    }

    for thread_number in 0..number_of_threads_per_op {
        for entry in &*extracted_content[thread_number].lock().unwrap() {
            assert_that!(added_contents_set.get(entry), is_some);
        }
    }

    // check if it is still in a consistent state
    mpmc_container_add_and_remove_elements_works();
}
