use gobject;
use gtypes;

/// Represents an owned gobject `T`
pub struct Ptr<T> {
    data: *mut T
}

impl<T> Drop for Ptr<T> {
    fn drop(&mut self) {
        unsafe {
            gobject::g_object_unref(self.data as gtypes::gpointer);
        }
    }
}
