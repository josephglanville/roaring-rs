extern crate roaring;

use roaring::RoaringBitmap;


fn as_ptr(bitmap: RoaringBitmap<u32>) -> *mut RoaringBitmap<u32> {
  Box::into_raw(Box::new(bitmap))
}

#[no_mangle]
pub extern fn roaring_rs_new() -> *mut RoaringBitmap<u32> {
  as_ptr(RoaringBitmap::new())
}

#[no_mangle]
pub extern fn roaring_rs_collect(data_p: *const u32, len: u32) -> *mut RoaringBitmap<u32> {
  let data = unsafe { std::slice::from_raw_parts(data_p, len as usize) };
  as_ptr(data.iter().collect())
}

#[no_mangle]
pub extern fn roaring_rs_delete(this_p: *mut RoaringBitmap<u32>) {
  unsafe { Box::from_raw(this_p) };
}

#[no_mangle]
pub extern fn roaring_rs_contains(this_p: *const RoaringBitmap<u32>, i: u32) -> bool {
  let this = unsafe { &*this_p };
  this.contains(i)
}

#[no_mangle]
pub extern fn roaring_rs_insert(this_p: *mut RoaringBitmap<u32>, i: u32) -> bool {
  let this = unsafe { &mut *this_p };
  this.insert(i)
}

#[no_mangle]
pub extern fn roaring_rs_len(this_p: *const RoaringBitmap<u32>) -> u32 {
  let this = unsafe { &*this_p };
  this.len()
}

#[no_mangle]
pub extern fn roaring_rs_and(this_p: *const RoaringBitmap<u32>, other_p: *const RoaringBitmap<u32>) -> *mut RoaringBitmap<u32> {
  let this = unsafe { &*this_p };
  let other = unsafe { &*other_p };
  as_ptr(this & other)
}

#[no_mangle]
pub extern fn roaring_rs_or(this_p: *const RoaringBitmap<u32>, other_p: *const RoaringBitmap<u32>) -> *mut RoaringBitmap<u32> {
  let this = unsafe { &*this_p };
  let other = unsafe { &*other_p };
  as_ptr(this | other)
}

#[no_mangle]
pub extern fn roaring_rs_xor(this_p: *const RoaringBitmap<u32>, other_p: *const RoaringBitmap<u32>) -> *mut RoaringBitmap<u32> {
  let this = unsafe { &*this_p };
  let other = unsafe { &*other_p };
  as_ptr(this ^ other)
}

#[no_mangle]
pub extern fn roaring_rs_sub(this_p: *const RoaringBitmap<u32>, other_p: *const RoaringBitmap<u32>) -> *mut RoaringBitmap<u32> {
  let this = unsafe { &*this_p };
  let other = unsafe { &*other_p };
  as_ptr(this - other)
}