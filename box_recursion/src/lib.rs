#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment{
            grade : None,
        }
    } 
    pub fn add_worker(&mut self, role: String, name: String) {
      let b =  Box::new(Worker{
            role,
            name,
            next : self.grade.take(),
        });
        self.grade = Some(b);

    }
    pub fn remove_worker(&mut self) -> Option<String> {
        if let Some(worker) = self.grade.take(){
            self.grade = worker.next;
             Some(worker.name) 
        }else{
            None
        }
    }
    pub fn last_worker(&self) -> Option<(String, String)> {
        self.grade.as_ref().map(|w| {
            (w.name.clone(), w.role.clone())
        })
    }
}
fn main() {
    let mut list = WorkEnvironment::new();
    list.add_worker(String::from("CEO"), String::from("Marie"));
    list.add_worker(String::from("Manager"), String::from("Monica"));
    list.add_worker(String::from("Normal Worker"), String::from("Ana"));
    list.add_worker(String::from("Normal Worker"), String::from("Alice"));
    println!("{:#?}", list);

    println!("{:?}", list.last_worker());

    list.remove_worker();
    list.remove_worker();
    list.remove_worker();
    println!("{:?}", list);
    list.remove_worker();
    println!("{:?}", list);
}
