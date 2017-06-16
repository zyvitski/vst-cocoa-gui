use libc::c_void;

#[derive(Default,Clone)]
pub struct VstGLContext {}

impl VstGLContext {}

impl Drop for VstGLContext {
    fn drop(&mut self) {}
}



#[derive(Clone)]
pub struct VstView {
    parent: *mut c_void,
    gl_context: VstGLContext,
}
impl VstView {
    pub fn new(handle: *mut c_void) -> Self {
        VstView {
            parent: handle,
            gl_context: VstGLContext::default(),
        }
    }
}
impl Drop for VstView {
    fn drop(&mut self) {}
}
