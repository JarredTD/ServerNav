use crate::buffer::storage::static_vec::BoundedVec;
use std::time::SystemTime;

const MAX_VEC_CAPACITY: usize = 50;

#[derive(Debug)]
pub struct TextBuffer {
    pub content: Option<String>,
    pub last_modified: SystemTime,
    pub is_saved: bool,
    undo_stack: BoundedVec<String>,
    redo_stack: BoundedVec<String>,
}

impl Default for TextBuffer {
    fn default() -> Self {
        TextBuffer {
            content: None,
            last_modified: SystemTime::now(),
            is_saved: true,
            undo_stack: BoundedVec::new(MAX_VEC_CAPACITY),
            redo_stack: BoundedVec::new(MAX_VEC_CAPACITY),
        }
    }
}

impl TextBuffer {
    pub fn new(initial_content: String) -> Self {
        TextBuffer {
            content: Some(initial_content),
            last_modified: SystemTime::now(),
            is_saved: true,
            undo_stack: BoundedVec::new(MAX_VEC_CAPACITY),
            redo_stack: BoundedVec::new(MAX_VEC_CAPACITY),
        }
    }

    pub fn update_content(&mut self, new_content: String) {
        if let Some(current_content) = self.content.take() {
            self.undo_stack.push(current_content);
        }
        self.content = Some(new_content);
        self.last_modified = SystemTime::now();
        self.is_saved = false;
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) {
        if let Some(previous_content) = self.undo_stack.pop() {
            if let Some(current_content) = self.content.take() {
                self.redo_stack.push(current_content)
            }
            self.content = Some(previous_content);
            self.last_modified = SystemTime::now();
        }
    }

    pub fn redo(&mut self) {
        if let Some(next_content) = self.redo_stack.pop() {
            if let Some(current_content) = self.content.take() {
                self.undo_stack.push(current_content)
            }
            self.content = Some(next_content);
            self.last_modified = SystemTime::now();
        }
    }

    pub fn is_modified(&self) -> bool {
        !self.is_saved
    }
}
