use crate::state::DrawObject;

/// A reversible command that can be undone and redone.
#[derive(Debug, Clone)]
pub enum Command {
    /// An object was added at the end of the objects list.
    Add(DrawObject),
    /// An object was removed from the given index.
    Remove(usize, DrawObject),
    /// An object at the given index was modified (old, new).
    Modify(usize, DrawObject, DrawObject),
}

/// Tracks a linear history of commands with a cursor for undo/redo navigation.
pub struct History {
    commands: Vec<Command>,
    /// Points to the next command slot (i.e. `commands[0..cursor]` are the "done" commands).
    cursor: usize,
}

impl History {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            cursor: 0,
        }
    }

    /// Record a new command, discarding any redo entries beyond the current cursor.
    pub fn push(&mut self, cmd: Command) {
        self.commands.truncate(self.cursor);
        self.commands.push(cmd);
        self.cursor += 1;
    }

    /// Reverse the last command, updating the objects list in place.
    pub fn undo(&mut self, objects: &mut Vec<DrawObject>) {
        if !self.can_undo() {
            return;
        }
        self.cursor -= 1;
        match &self.commands[self.cursor] {
            Command::Add(_) => {
                objects.pop();
            }
            Command::Remove(index, obj) => {
                let idx = (*index).min(objects.len());
                objects.insert(idx, obj.clone());
            }
            Command::Modify(index, old, _new) => {
                if let Some(slot) = objects.get_mut(*index) {
                    *slot = old.clone();
                }
            }
        }
    }

    /// Replay the next command, updating the objects list in place.
    pub fn redo(&mut self, objects: &mut Vec<DrawObject>) {
        if !self.can_redo() {
            return;
        }
        match &self.commands[self.cursor] {
            Command::Add(obj) => {
                objects.push(obj.clone());
            }
            Command::Remove(index, _obj) => {
                let idx = (*index).min(objects.len().saturating_sub(1));
                if idx < objects.len() {
                    objects.remove(idx);
                }
            }
            Command::Modify(index, _old, new) => {
                if let Some(slot) = objects.get_mut(*index) {
                    *slot = new.clone();
                }
            }
        }
        self.cursor += 1;
    }

    pub fn can_undo(&self) -> bool {
        self.cursor > 0
    }

    pub fn can_redo(&self) -> bool {
        self.cursor < self.commands.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Color32, Pos2};

    fn make_freehand(id: u8) -> DrawObject {
        DrawObject::Freehand {
            points: vec![Pos2::new(id as f32 * 0.1, 0.5)],
            colour: Color32::BLACK,
            width: 2.0,
        }
    }

    #[test]
    fn push_and_undo_restores_state() {
        let mut history = History::new();
        let mut objects: Vec<DrawObject> = Vec::new();

        let obj = make_freehand(1);
        objects.push(obj.clone());
        history.push(Command::Add(obj));

        assert_eq!(objects.len(), 1);
        assert!(history.can_undo());

        history.undo(&mut objects);
        assert_eq!(objects.len(), 0);
        assert!(!history.can_undo());
    }

    #[test]
    fn redo_replays_command() {
        let mut history = History::new();
        let mut objects: Vec<DrawObject> = Vec::new();

        let obj = make_freehand(1);
        objects.push(obj.clone());
        history.push(Command::Add(obj));

        history.undo(&mut objects);
        assert_eq!(objects.len(), 0);
        assert!(history.can_redo());

        history.redo(&mut objects);
        assert_eq!(objects.len(), 1);
        assert!(!history.can_redo());
    }

    #[test]
    fn push_after_undo_truncates_redo_entries() {
        let mut history = History::new();
        let mut objects: Vec<DrawObject> = Vec::new();

        let obj1 = make_freehand(1);
        objects.push(obj1.clone());
        history.push(Command::Add(obj1));

        let obj2 = make_freehand(2);
        objects.push(obj2.clone());
        history.push(Command::Add(obj2));

        // Undo obj2
        history.undo(&mut objects);
        assert_eq!(objects.len(), 1);

        // Push a new obj3 — should discard the redo of obj2
        let obj3 = make_freehand(3);
        objects.push(obj3.clone());
        history.push(Command::Add(obj3));

        assert!(!history.can_redo());
        assert!(history.can_undo());
    }

    #[test]
    fn undo_remove_reinserts_object() {
        let mut history = History::new();
        let mut objects: Vec<DrawObject> = Vec::new();

        let obj = make_freehand(1);
        objects.push(obj.clone());
        history.push(Command::Add(obj.clone()));

        // Simulate eraser removing the object at index 0
        objects.remove(0);
        history.push(Command::Remove(0, obj));

        assert_eq!(objects.len(), 0);

        history.undo(&mut objects);
        assert_eq!(objects.len(), 1);
    }

    #[test]
    fn redo_remove_deletes_object_again() {
        let mut history = History::new();
        let mut objects: Vec<DrawObject> = Vec::new();

        let obj = make_freehand(1);
        objects.push(obj.clone());
        history.push(Command::Add(obj.clone()));

        objects.remove(0);
        history.push(Command::Remove(0, obj));

        history.undo(&mut objects);
        assert_eq!(objects.len(), 1);

        history.redo(&mut objects);
        assert_eq!(objects.len(), 0);
    }

    #[test]
    fn multiple_undo_redo_round_trip() {
        let mut history = History::new();
        let mut objects: Vec<DrawObject> = Vec::new();

        for i in 0..5 {
            let obj = make_freehand(i);
            objects.push(obj.clone());
            history.push(Command::Add(obj));
        }
        assert_eq!(objects.len(), 5);

        // Undo all
        for _ in 0..5 {
            history.undo(&mut objects);
        }
        assert_eq!(objects.len(), 0);
        assert!(!history.can_undo());

        // Redo all
        for _ in 0..5 {
            history.redo(&mut objects);
        }
        assert_eq!(objects.len(), 5);
        assert!(!history.can_redo());
    }

    #[test]
    fn undo_on_empty_history_is_no_op() {
        let mut history = History::new();
        let mut objects: Vec<DrawObject> = Vec::new();
        history.undo(&mut objects);
        assert_eq!(objects.len(), 0);
    }

    #[test]
    fn redo_on_empty_history_is_no_op() {
        let mut history = History::new();
        let mut objects: Vec<DrawObject> = Vec::new();
        history.redo(&mut objects);
        assert_eq!(objects.len(), 0);
    }

    #[test]
    fn modify_command_undo_redo() {
        let mut history = History::new();

        let old = make_freehand(1);
        let new = make_freehand(2);
        let mut objects = vec![old.clone()];

        objects[0] = new.clone();
        history.push(Command::Modify(0, old.clone(), new));

        history.undo(&mut objects);
        // After undo, the object should match the old value
        if let DrawObject::Freehand { points, .. } = &objects[0] {
            assert!((points[0].x - 0.1).abs() < f32::EPSILON);
        } else {
            panic!("expected Freehand");
        }

        history.redo(&mut objects);
        if let DrawObject::Freehand { points, .. } = &objects[0] {
            assert!((points[0].x - 0.2).abs() < f32::EPSILON);
        } else {
            panic!("expected Freehand");
        }
    }
}
