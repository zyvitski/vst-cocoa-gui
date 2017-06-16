use vst2::editor::{Editor, KeyCode, KnobMode};
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
        }
        if let Some(ref mut view) = self.view {
            view.open();
        }
    }
    fn close(&mut self) {
        if let Some(ref mut view) = self.view {
            view.close();
            drop(view);
        }
        self.view = None;
    }
    fn is_open(&mut self) -> bool {
        self.view.is_some()
    }
    fn idle(&mut self) {}
    fn set_knob_mode(&mut self, mode: KnobMode) -> bool {
        false
    }
    fn key_up(&mut self, keycode: KeyCode) -> bool {
        false
    }
    fn key_down(&mut self, keycode: KeyCode) -> bool {
        false
    }
}
