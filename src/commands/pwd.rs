use std::env;

pub fn pwd(){
    // match en
    match env::current_dir() {
        Ok(path) => {
            println!("{}",path.display())
        },
        Err(e)=>{
            eprintln!("{}",e)
        }
    }
}