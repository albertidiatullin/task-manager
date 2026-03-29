use serde::Serialize;

#[derive(Serialize,Debug)]
pub struct RegisterAuthReposnse<'a>{
    pub message:&'a str,
}