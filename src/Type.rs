use super::Indention::*;
use super::Writer::qwriter;

pub trait Type: TypeClone {
    fn emit(&mut self, result: &mut qwriter, indention: &mut Indention);
}

trait TypeClone {
    fn clone_box(&self) -> Box<dyn Type>;
}

impl<T> TypeClone for T
where
    T: 'static + Type + Clone,
{
    fn clone_box(&self) -> Box<dyn Type> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Type> {
    fn clone(&self) -> Box<dyn Type> {
        self.clone_box()
    }
}
