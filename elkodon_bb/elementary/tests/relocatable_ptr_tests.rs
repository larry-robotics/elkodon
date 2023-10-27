use elkodon_bb_elementary::{pointer_trait::PointerTrait, relocatable_ptr::RelocatablePointer};
use elkodon_bb_testing::assert_that;

#[test]
fn relocatable_pointer_works() {
    let mut sut = RelocatablePointer::<i32>::new(0);
    let _o2: i32 = 0;
    let mut _o3: i32 = 0;
    let value = 9128391;

    let distance = std::ptr::addr_of!(_o3) as isize - std::ptr::addr_of!(sut) as isize;

    sut = RelocatablePointer::<i32>::new(distance);
    _o3 = value;
    assert_that!(unsafe { *sut.as_ptr() }, eq value);
}
