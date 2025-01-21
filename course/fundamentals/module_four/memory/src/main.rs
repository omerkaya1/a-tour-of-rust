/// # Safety
/// 
/// This function is unsafe because ...
unsafe fn my_unsafe_fn() {

}


fn main() {
    let v = vec![1,2,3,4,5];
    // we can also do the get_unchecked() which is faster
    // there could be a situation when we make the whole block unsafe
    if let _ = v.get(2) {
    }

    v[20];
}
