use vst2::editor::Editor;
use libc::c_void;
use view::VstView;


#[derive(Default)]
pub struct BasicEditor {
    view: Option<VstView>,
}

impl Editor for BasicEditor {
    fn size(&self) -> (i32, i32) {
        (0, 0)
    }
    fn position(&self) -> (i32, i32) {
        (0, 0)
    }
    fn open(&mut self, handle: *mut c_void) {
        if self.view.is_none() {
            self.view = Some(VstView::new(handle));
            //init nsview?
        } else {
            unimplemented!()
        }
    }
    fn close(&mut self) {
        if let Some(view) = self.view.clone() {
            drop(view)
            //clean up
        } else {
            unimplemented!()
        }
    }
    fn is_open(&mut self) -> bool {
        self.view.is_some()
    }
}
