mod a{
    pub fn hello(){
        println!("Hello from a");
    }
}

pub use a::hello;

