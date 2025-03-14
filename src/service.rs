use crate::{
    models::{CreateUser, User},
    repository::Repository,
};
use nject::injectable;

#[injectable]
pub struct UserService<'a> {
    repository: &'a dyn Repository,
}

impl<'a> UserService<'a> {
    pub fn create(&self, user: CreateUser) -> User {
        self.repository.create(user)
    }

    pub fn get(&self, user_id: usize) -> Option<User> {
        self.repository.get(user_id)
    }
}

#[injectable]
pub struct CpuIntensiveService<'a> {
    repository: &'a dyn Repository,
}

impl<'a> CpuIntensiveService<'a> {
    pub fn process_cpu(&self) -> String {
        let mut s = String::new();
        for _ in 0..300000 {
            s.push_str(&self.repository.get(1).unwrap().name);
        }
        s
    }
}
