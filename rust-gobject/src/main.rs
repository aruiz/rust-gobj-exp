#![allow(dead_code)]

extern crate gobject_2_0_sys as gobject;
extern crate gtypes as gtypes;
#[macro_use]
extern crate lazy_static;

use gtypes::gpointer;
use std::cell::Cell;
use std::mem;
use std::ptr;

mod pointer;
use self::pointer::Ptr;

#[repr(C)]
struct TestFoo {
    parent_instance: gobject::GObject,
    private: Box<TestFooPrivate>
}

impl TestFoo {
    fn private(&self) -> &TestFooPrivate {
        &self.private
    }
}

#[repr(C)]
struct TestFooPrivate {
    some: Cell<usize>,
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

extern "C" fn test_foo_finalize(object: *mut gobject::GObject) {
    unsafe {
        let foo = object as *mut TestFoo;
        mem::drop(&mut (*foo).private);

        // FIXME -- g_class field of `GTypeInstance` ought to be `pub`
        let xxx = object as *mut *mut gobject::GTypeClass;
        let object_class = *xxx;
        let parent_class = gobject::g_type_class_peek_parent(object_class as gpointer);
        let g_object_class = parent_class as *mut gobject::GObjectClass;
        ((*g_object_class).finalize.unwrap())(object);
    }
}

extern "C" fn test_foo_instance_init(obj: *mut TestFoo) {
    let private = Box::new(TestFooPrivate {
        some: Cell::new(5)
    });
    unsafe {
        ptr::write(&mut (*obj).private, private);
    }
}

fn test_foo_construct(object_type: gtypes::GType) -> Ptr<TestFoo> {
    unsafe {
        let this: *mut TestFoo = gobject::g_object_new(object_type, ptr::null_mut()) as *mut TestFoo;
        Ptr::new(this)
    }
}

fn test_foo_new() -> Ptr<TestFoo> {
    test_foo_construct(*TEST_TYPE_FOO)
}

lazy_static! {
    pub static ref TEST_TYPE_FOO: gtypes::GType = {
        unsafe {
            gobject::g_type_register_static_simple(
                gobject::g_object_get_type(),
                b"TestFoo\0" as *const u8 as *const i8,
                mem::size_of::<TestFooClass>() as u32,
                mem::transmute(test_foo_class_init as extern "C" fn(*mut TestFooClass)),
                mem::size_of::<TestFoo>() as u32,
                mem::transmute(test_foo_instance_init as extern "C" fn(*mut TestFoo)),
                mem::transmute(0)) // FIXME GTypeFlags should not be an enum
        }
    };
}

fn main() {
    let bar = test_foo_new();
}

