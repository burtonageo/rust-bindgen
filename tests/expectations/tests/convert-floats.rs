/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct foo {
    pub bar: ::std::os::raw::c_float,
    pub baz: ::std::os::raw::c_float,
    pub bazz: ::std::os::raw::c_double,
    pub bazzz: *mut ::std::os::raw::c_double,
    pub complexFloat: __BindgenComplex<::std::os::raw::c_float>,
    pub complexDouble: __BindgenComplex<::std::os::raw::c_double>,
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(::std::mem::size_of::<foo>() , 48usize);
    assert_eq!(::std::mem::align_of::<foo>() , 8usize);
}
impl Clone for foo {
    fn clone(&self) -> Self { *self }
}
