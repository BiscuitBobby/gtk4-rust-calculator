//this do be a simple value printing macro
macro_rules! mfunc {
    (x => $e:expr) => (println!("this is an expression"));
    (y => $e:expr) => (println!("this is the y expression"));
}
//behold...a simple code generation macro
macro_rules! mfuncode {
    ($($name:ident),*) => {
        $(fn $name(){
            println!("{} good",stringify!($name));
        })*
    }
}

fn main() {
    println!("hello there");
    mfunc!(y =>10+3);
    mfuncode!(lemonade);
    lemonade();
}
