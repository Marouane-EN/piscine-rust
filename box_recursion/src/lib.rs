#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(value: &str) -> Self {
        match value {
            "CEO" => Self::CEO,
            "Manager" => Self::Manager,
            _ => Self::Worker,
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
        Self { grade: None }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let head = Box::new(Worker {
            role: Role::from(role),
            name: name.to_string(),
            next: self.grade.take(),
        });
        self.grade = Some(head);
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        let name = self.grade.take()?;
        self.grade = name.next;
        Some(name.name)
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        let name = self.grade.as_ref()?;
        Some((name.name.clone(), name.role))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
