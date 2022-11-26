use super::*;
use crate::conv::*;
use crate::rt_types::*;
use crate::std::vector::{self, *};
use crate::std::string::*;
use alloc::{string::String, vec, vec::Vec};
use core::mem;

#[test]
fn test_string_check_utf8() {
    let rust_vec = vec![240, 159, 146, 150];
    let move_vec = rust_vec_to_move_byte_vec(rust_vec);

    let is_utf8 = unsafe { internal_check_utf8(&move_vec) };
    assert!(is_utf8);

    unsafe { move_byte_vec_to_rust_vec(move_vec) };

    let rust_vec = vec![0, 159, 146, 150];
    let move_vec = rust_vec_to_move_byte_vec(rust_vec);

    let is_utf8 = unsafe { internal_check_utf8(&move_vec) };
    assert!(!is_utf8);

    unsafe { move_byte_vec_to_rust_vec(move_vec) };
}

#[test]
fn test_string_is_char_boundary() {
    let rust_vec = String::from("Löwe").into_bytes();
    let move_vec = rust_vec_to_move_byte_vec(rust_vec);

    let is_char_0 = unsafe { internal_is_char_boundary(&move_vec, 0) };
    assert!(is_char_0);

    let is_char_1 = unsafe { internal_is_char_boundary(&move_vec, 2) };
    assert!(!is_char_1);

    unsafe { move_byte_vec_to_rust_vec(move_vec) };
}

#[test]
fn test_sub_string() {
    let rust_vec = b"sub string test".to_vec();
    let move_vec = rust_vec_to_move_byte_vec(rust_vec);

    let move_vec_sub_string = unsafe { internal_sub_string(&move_vec, 0, 10) };
    let rust_vec_sub_string = unsafe { move_byte_vec_to_rust_vec(move_vec_sub_string) };

    assert_eq!(rust_vec_sub_string, b"sub string");

    unsafe { move_byte_vec_to_rust_vec(move_vec) };
}

#[test]
fn test_string_index_of() {
    let rust_vec = b"bears love snow".to_vec();
    let move_vec = rust_vec_to_move_byte_vec(rust_vec);

    let rust_vec_sub = b"love".to_vec();
    let move_vec_sub = rust_vec_to_move_byte_vec(rust_vec_sub);

    let index = unsafe { internal_index_of(&move_vec, &move_vec_sub) };

    assert_eq!(index, 6);

    unsafe { move_byte_vec_to_rust_vec(move_vec) };
    unsafe { move_byte_vec_to_rust_vec(move_vec_sub) };
}

#[test]
fn test_vec_with_bool() {
    static ELEMENT_TYPE: MoveType = MoveType {
        type_desc: TypeDesc::Bool,
        type_info: TypeInfo { nothing: 0 },
    };

    let mut move_vec = vector::empty(&ELEMENT_TYPE);
    assert_eq!(move_vec.length, 0);
    assert_eq!(move_vec.capacity, 0);
    
    let move_vec_len = unsafe { vector::length(&ELEMENT_TYPE, &move_vec) };
    assert_eq!(move_vec_len, 0);

    let mut new_element: bool = true;
    let new_element_ptr = &mut new_element as *mut _ as *mut AnyValue;
    unsafe { vector::push_back(&ELEMENT_TYPE, &mut move_vec, new_element_ptr) }
    assert_eq!(move_vec.length, 1);
    
    let mut popped_element: bool = false;
    let popped_element_ptr = &mut popped_element as *mut _ as *mut AnyValue;

    unsafe { vector::pop_back(&ELEMENT_TYPE, &mut move_vec, popped_element_ptr) };
    assert_eq!(move_vec.length, 0);
    assert_eq!(popped_element, true);

    unsafe { vector::destroy_empty(&ELEMENT_TYPE, move_vec) }
}
