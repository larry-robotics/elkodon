use elkodon_bb_elementary::unique_id::*;
use elkodon_bb_testing::assert_that;

#[test]
fn unique_id_is_unique() {
    let a = UniqueId::new();
    let b = UniqueId::new();
    let c = UniqueId::new();

    assert_that!(a, ne b);
    assert_that!(a, ne c);
    assert_that!(b, ne c);
}

#[test]
fn typed_unique_id_is_unique() {
    let a = TypedUniqueId::<u64>::new();
    let b = TypedUniqueId::<u64>::new();
    let c = TypedUniqueId::<u64>::new();

    assert_that!(a, ne b);
    assert_that!(a, ne c);
    assert_that!(b, ne c);

    let d = TypedUniqueId::<u32>::new();
    assert_that!(a.value(), ne d.value());
}
