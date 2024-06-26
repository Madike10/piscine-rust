pub use std::collections::HashMap;
pub use std::{cell::RefCell, rc::Rc};

// std::cell::RefCell;
pub use messenger::*;   
pub mod messenger;
#[derive(Debug, Clone, PartialEq, Eq)]

pub struct Worker{
   pub track_value: Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}
impl Worker {
    pub fn new(value:usize) -> Self{
        Worker{
             track_value : Rc::new(value),
             mapped_messages: RefCell::new(HashMap::new()),
             all_messages: RefCell::new(Vec::new()),
            }
    }
}
impl Logger for Worker{
    fn warning(&self, msg: &str){
        self.mapped_messages.borrow_mut().insert("Warning".to_string(), msg.to_string().replace("Warning: ", ""));
        self.all_messages.borrow_mut().push(msg.to_string());
    }
    fn info(&self, msg: &str){
        self.mapped_messages.borrow_mut().insert("Info".to_string(), msg.to_string().replace("Info: ", ""));
        self.all_messages.borrow_mut().push(msg.to_string());
    }
    fn error(&self, msg: &str){
        self.mapped_messages.borrow_mut().insert("Error".to_string(), msg.to_string().replace("Error: ", ""));
        self.all_messages.borrow_mut().push(msg.to_string());
    }

}


fn main() {
    // initialize the worker
    let logger = Worker::new(1);

    // initialize the tracker, with the max number of
    // called references as 10
    let track = Tracker::new(&logger, 10);

    let _a = logger.track_value.clone();    // |\
    let _a1 = logger.track_value.clone();   // | -> increase the Rc to 4 references
    let _a2 = logger.track_value.clone();   // |/

    // take a peek of how much we already used from our quota
    track.peek(&logger.track_value);

    let _b = logger.track_value.clone();  // |\
    let _b1 = logger.track_value.clone(); // |  -> increase the Rc to 8 references
    let _b2 = logger.track_value.clone(); // | /
    let _b3 = logger.track_value.clone(); // |/

    // this will set the value and making a verification of
    // how much we already used of our quota
    track.set_value(&logger.track_value);

    let _c = logger.track_value.clone(); // | -> increase the Rc to 9 references

    // this will set the value and making a verification of
    // how much we already used of our quota
    track.set_value(&logger.track_value);

    let _c1 = logger.track_value.clone(); // | -> increase the Rc to 10 references, this will be the limit

    track.set_value(&logger.track_value);

    for (k ,v) in logger.mapped_messages.into_inner() {
        println!("{:?}", (k ,v));
    }
    println!("{:?}", logger.all_messages.into_inner());
}
