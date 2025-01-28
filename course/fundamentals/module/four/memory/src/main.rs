#![allow(dead_code)]


/// # Safety
/// 
/// This function is unsafe because ...
unsafe fn my_unsafe_fn() {

}

fn allocate_mem_with_libc() {
    unsafe {
        let num: *mut i32 = libc::malloc(std::mem::size_of::<i32>() as libc::size_t) as *mut i32;
        if num.is_null() {
            panic!("failed to allocate memory")
        }

        *num = 123;
        assert_eq!(123, *num);

        libc::free(num as *mut libc::c_void);
    }
}

fn allocate_using_rust() {
    use std::alloc::{alloc, dealloc, Layout};

    unsafe {
        let layout = Layout::new::<i32>();
        let ptr = alloc(layout);

        *ptr = 123;
        assert_eq!(123, *ptr);

        dealloc(ptr, layout);
    }
}


fn main() {
    // let v = vec![1,2,3,4,5];
    // we can also do the get_unchecked() which is faster
    // there could be a situation when we make the whole block unsafe
    // if let _ = v.get(2) {
    // }

    // v[20];

    allocate_mem_with_libc();
    allocate_using_rust();
}
