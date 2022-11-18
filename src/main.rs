#[warn(non_snake_case)]
macro_rules! _ola{
    () =>{
        println!("Hello World");
    }
}

fn main(){
    _ola!();
}