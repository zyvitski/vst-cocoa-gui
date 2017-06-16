use libc::c_void;
use cocoa::base::{id, nil};
use cocoa::appkit::{NSWindow, NSView};

#[derive(Clone)]
pub struct VstView {
    parent: *mut c_void,
    view: id,
}
impl VstView {
    pub fn new(handle: *mut c_void) -> Self {
        VstView {
            parent: handle,
            view: nil,
        }
    }
    pub fn open(&mut self) {
        //treat as NSWindow
        let window = self.parent as id;
        unsafe {
            self.view = NSView::alloc(self.view);
            self.view.init(); //maybe with rect
            window.addSubview_(self.view);
        }
    }
    pub fn close(&mut self) {
        unsafe {
            self.view.removeFromSuperview();
        }
    }
}
impl Drop for VstView {
    fn drop(&mut self) {
        unsafe {
            self.view.removeFromSuperview();
        }
    }
}
