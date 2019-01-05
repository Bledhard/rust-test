use std::fmt;

pub struct User {
    pub username:String, 
    pub email:String, 
    pub sign_in_count:u64, 
    pub active:bool, 
}

impl fmt::Display for User {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nusername: {}\nemail: {}\nsign_in_count: {}\nactive: {}", self.username, self.email, self.sign_in_count, self.active)
    }
}