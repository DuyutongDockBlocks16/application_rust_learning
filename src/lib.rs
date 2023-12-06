#[derive(Debug)]
pub struct PositiveF64{
    value : f64
}

impl PositiveF64 {

    pub fn new (value: f64) -> Option<Self>{
        if value > 0.0{
            return Some(Self { value })
        }
        None
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn replace_value(&mut self, new_value: f64) -> Option<f64> {
        if new_value > 0.0{
            let old_value = self.value();
            self.value = new_value;
            return Some(old_value)
        }
        None
    }

}
