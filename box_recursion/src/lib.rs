#[derive(Debug, PartialEq , Clone)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
     fn from(s :&str) -> Self{
        match s.to_lowercase().as_str(){
            "ceo" => Role::CEO,
            "manager" => Role::Manager,
            "worker" => Role::Worker,
            _ => Role::Worker,
        }
    }
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;
#[derive(Debug)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        Self{grade : None}
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
            let new_workr = Worker{
                role : Role::from(role),
                name : name.to_string(),
                next : self.grade.take(),
            };
            self.grade = Some(Box::new(new_workr))
    }


 pub fn remove_worker(&mut self) -> Option<String> {
        if let Some(worker) = self.grade.take() {
            self.grade = worker.next; 
            Some(worker.name)
        } else {
            None
        }
    }
    


 pub fn last_worker(&self) -> Option<(String, Role)> {
    self.grade.as_ref().map(|worker| (worker.name.clone(), worker.role.clone()))
}


}