use std::ops::Deref;

use gobject;
use gtypes;

/// Represents an owned gobject `T`
pub struct Ptr<T> {
    data: *mut T
}

impl<T> Ptr<T> {
    pub unsafe fn new(data: *mut T) -> Ptr<T> {
        Ptr { data: data }
    }
}

impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        unsafe {
            gobject::g_object_ref(self.data as gtypes::gpointer);
            Ptr { data: self.data }
        }
    }
}

impl<T> Deref for Ptr<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {
            &(*self.data)
        }
    }
}

impl<T> Drop for Ptr<T> {
    fn drop(&mut self) {
        unsafe {
            gobject::g_object_unref(self.data as gtypes::gpointer);
        }
    }
}
