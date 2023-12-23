#[derive(Debug)]
pub enum ValueState {
    Number(u16),
    Register(u16),
    Invalid,
}

#[derive(Debug, Clone, Copy)]
pub struct Value(u16);

impl Value {
    pub fn get_value(self) -> ValueState {
        match self.0 {
            value if value <= 32767 => ValueState::Number(value),
            value if value <= 32775 => ValueState::Register(value - 32768),
            _ => ValueState::Invalid,
        }
    }
}
