use std::mem::{
    align_of,
    size_of,
};

#[inline]
pub fn cast<A: Copy, B: Copy>(a: &[A]) -> &[B] {
    match unsafe { try_cast_slice(a) } {
        Ok(b) => b,
        Err(e) => panic!("{:?}", e),
    }
}

#[inline]
fn is_aligned_to(ptr: *const (), align: usize) -> bool {
    ((ptr as usize) % align) == 0
}

unsafe fn try_cast_slice<A: Copy, B: Copy>(a: &[A]) -> Result<&[B], &str> {
    if align_of::<B>() > align_of::<A>() && !is_aligned_to(a.as_ptr() as *const (), align_of::<B>()) {
        Err("Target Alignment Greater And Input Not Aligned")
    } else if size_of::<B>() == size_of::<A>() {
        Ok(unsafe { core::slice::from_raw_parts(a.as_ptr() as *const B, a.len()) })
    } else if size_of::<A>() == 0 || size_of::<B>() == 0 {
        Err("Size Mismatch")
    } else if core::mem::size_of_val(a) % size_of::<B>() == 0 {
        let new_len = core::mem::size_of_val(a) / size_of::<B>();
        Ok(unsafe { core::slice::from_raw_parts(a.as_ptr() as *const B, new_len) })
    } else {
        Err("Output Slice Would Have Slop")
    }
}
