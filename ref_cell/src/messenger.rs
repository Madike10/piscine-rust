pub use std::{cell::RefCell, rc::Rc};
// pub mod messenger {
    pub trait Logger {
        fn warning(&self, msg: &str);
        fn info(&self, msg: &str);
        fn error(&self, msg: &str);
    }

    pub struct Tracker<'a, T:Logger>{
        pub logger: &'a T,
        pub value: RefCell<usize>,
        pub max: usize,
    }
    impl<'a, T:Logger> Tracker<'a, T>{
        pub fn new(logger : &'a T, max: usize) -> Self{
            Tracker{
                logger,
                 value : RefCell::new(0),
                 max
            }
        }
        pub fn set_value(&self,  value: &Rc<usize>){
            let p = (Rc::strong_count(value) * 100) / self.max;
            if p >= 100 {
                self.logger.error("Error: you are over your quota!");
            }else if p >= 70  {
                self.logger.warning(&format!("Warning: you have used up over {}% of your quota! Proceeds with precaution", p));
            }

        }
        pub fn peek(&self, value: &Rc<usize>){
            let p = (Rc::strong_count(value) * 100) / self.max;
            self.logger.info(&format!("Info: you are using up to {}% of your quota", p));
        }
    }
