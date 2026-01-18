#[derive(Debug, PartialEq)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(value: &str) -> Self {
        match value {
            "CEO" => Self::CEO,
            "Manager" => Self::CEO,
            _ => Self::Worker,
        }
    }
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Worker>;

#[derive(Debug)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        Self { grade: None }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let mut work = self.grade.as_mut().unwrap().next;
        while let Some(w) = work {
            if w.next.is_none() {
                let new_worker = Worker {
                    role: Role::from(role),
                    name: name.to_string(),
                    next: None,
                };
                w.next = Some(new_worker);
            }
            work = w.next;
        }
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        todo!()
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
