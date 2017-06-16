#[macro_use]
extern crate vst2;
extern crate libc;
extern crate objc;
extern crate objc_foundation;
extern crate cocoa;

mod editor;
mod view;

use vst2::plugin::{Info, Plugin};
use vst2::editor::Editor;

use editor::BasicEditor;

#[derive(Default)]
struct BasicPlugin {
    editor: BasicEditor,
}

impl Plugin for BasicPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "Basic Plugin".to_string(),
            unique_id: 1357, // Used by hosts to differentiate between plugins.

            ..Default::default()
        }
    }
    fn get_editor(&mut self) -> Option<&mut Editor> {
        Some(&mut self.editor)
    }
}

plugin_main!(BasicPlugin); // Important!
