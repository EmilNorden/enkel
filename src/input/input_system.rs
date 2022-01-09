use std::collections::HashMap;
use std::rc::Rc;
use winit::event::VirtualKeyCode;
use crate::input::keyboard::KeyCode;

pub struct InputSystem {
    axes: HashMap<String, Axis>,
}

#[derive(Debug)]
pub enum InputError {
    AxisDoesNotExist
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            axes: HashMap::new()
        }
    }

    pub fn query_axis(&self, axis: &str) -> Result<f32, InputError> {
        let axis = self.axes.get(axis).ok_or(InputError::AxisDoesNotExist)?;
        let value = axis.bindings
            .iter()
            .filter(|(_, active)| *active)
            .fold(0.0f32, |val, (binding, _)| val + binding.scale);

        Ok(value)
    }

    pub fn create_axis(&mut self, definition: AxisDefinition) {
        self.axes.insert(definition.name.to_string(), Axis {
            current_value: 0.0,
            bindings: definition.bindings.into_iter().zip(std::iter::repeat(false)).collect()
        });
    }

    pub(crate) fn on_key_down(&mut self, keycode: KeyCode) {
        for (_, axis) in &mut self.axes {
            for (binding_def, active) in &mut axis.bindings {
                if binding_def.binding == AxisBinding::Keyboard(keycode) {
                    *active = true;
                }
            }
        }
    }

    pub(crate) fn on_key_up(&mut self, keycode: KeyCode) {
        for (_, axis) in &mut self.axes {
            for (binding_def, active) in &mut axis.bindings {
                if binding_def.binding == AxisBinding::Keyboard(keycode) {
                    *active = false;
                }
            }
        }
    }

    pub(crate) fn on_new_frame(&mut self) {
        for (_, axis) in &mut self.axes {
            axis.current_value = 0.0;
        }
    }

}

#[derive(PartialEq)]
pub enum AxisBinding {
    Keyboard(KeyCode),
}

pub struct Axis {
    bindings: Vec<(AxisBindingDefinition, bool)>,
    current_value: f32,
}

pub struct AxisBindingDefinition {
    pub binding: AxisBinding,
    pub scale: f32,
}

pub struct AxisDefinition<'a> {
    pub name: &'a str,
    pub bindings: Vec<AxisBindingDefinition>,
}
