#![allow(dead_code)]

extern crate gobject_2_0_sys as gobject;
extern crate gtypes as gtypes;

use gtypes::gpointer;
use std::mem;

mod ptr;

#[repr(C)]
struct TestFoo {
    parent_instance: gobject::GObject,
    private: *mut TestFooPrivate,
}

#[repr(C)]
struct TestFooPrivate {
}

#[repr(C)]
struct TestFooClass {
    parent_class: gobject::GObjectClass
}

extern "C" fn test_foo_class_init(klass: *mut TestFooClass) {
    unsafe {
        gobject::g_type_class_add_private(klass as gpointer, mem::size_of::<TestFooPrivate>());
        let g_object_class = klass as *mut gobject::GObjectClass;
        (*g_object_class).finalize = Some(test_foo_finalize);
    }
}

extern "C" fn test_foo_finalize(_arg: *mut gobject::GObject) {
    // XXX
}

fn main() {
}

