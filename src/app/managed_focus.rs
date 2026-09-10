use eframe::egui::{Id, Ui};

// Custom implementation of managed default focus (it's not a library feature of egui)

/**
 * Contained datatype for managing default focus.
 */
pub type FocusManager = Option<(Id, bool)>;

/**
 * Trait providing api for management of focus.
 */
pub trait ManagedFocus {
    fn focus_state_mut(&mut self) -> &mut FocusManager;

    /**
     * Assign default focus to this element.
     */
    fn default_focus(&mut self, id : &Id) {
        let curr = self.focus_state_mut();
        // in the event that the new id matches the old one, make no changes.
        if let Some(v) = curr && v.0 == *id {
            return;
        }
        // otherwise, cache this id with the signal to focus this
        *curr = Some((id.clone(), true));
    }

    /**
     * Resets focus state completely.
     */
    fn reset_focus(&mut self) {
        let curr = self.focus_state_mut();
        if curr.is_some() {
            *curr = None;
        }
    }

    /**
     * Apply the current focus state on UI, triggering the current default to grab attention if it hasn't already.
     */
    fn apply_focus(&mut self, ui : &mut Ui) {
        if let Some(v) = self.focus_state_mut() && v.1 == true {
            v.1 = false;
            // apply focus here
            ui.memory_mut(|mem| {
                mem.request_focus(v.0);
            });
        }
    }
}

impl ManagedFocus for FocusManager {
    fn focus_state_mut(&mut self) -> &mut FocusManager {
        self
    }
}