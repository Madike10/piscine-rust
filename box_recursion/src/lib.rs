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
        if let Some(Worker) == self.grade.take(){
            self.grade = Worker.next;
             Some(Worker.name) 
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
