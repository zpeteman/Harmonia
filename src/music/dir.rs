pub fn main() -> String {                      
    loop {                                     
        let username = whoami::username(); 
        let path = format!("/home/{}/Music/Harmonia",username);             
        return path;                           
    }                                          
}                                              
