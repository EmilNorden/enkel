use std::collections::HashMap;
use std::rc::Rc;
use winit::event::VirtualKeyCode;

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
        Ok(axis.current_value)
    }

    pub fn create_axis(&mut self, definition: AxisDefinition) {
        self.axes.insert(definition.name.to_string(), Axis {
            current_value: 0.0,
            bindings: definition.bindings
        });
    }

    pub(crate) fn on_keyboard_input(&mut self, keycode: VirtualKeyCode) {
        for (_, axis) in &mut self.axes {
            for binding_def in &axis.bindings {
                if binding_def.binding == AxisBinding::Keyboard(keycode) {
                    axis.current_value += binding_def.scale;
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
    Keyboard(VirtualKeyCode),
}

pub struct Axis {
    bindings: Vec<AxisBindingDefinition>,
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
