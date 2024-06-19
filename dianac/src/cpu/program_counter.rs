use arbitrary_int::{u6, Number};

#[derive(Default, Debug)]
pub struct ProgramCounter(pub u6, pub u6);

impl ProgramCounter {
    pub fn set(&mut self, value: (u6, u6)) {
        self.0 = value.0;
        self.1 = value.1;
    }

    pub fn increment(&mut self) {
        if self.1 < u6::MAX {
            self.1 += u6::new(1);
        } else {
            self.0 += u6::new(1);
            self.1 = u6::default();
        }
    }

    pub fn as_tuple(&self) -> (u6, u6) {
        (self.0, self.1)
    }
}
