mod company{
    pub mod department{
        pub fn design(){
             println!("You are in Circuit Design Department");
        }

    }
}

fn main() {
    crate::company::department::design();
} 
