// TODO: We sometimes encourage you to keep trying things on a given exercise
// even after you already figured it out. If you got everything working and feel
// ready for the next exercise, enter `n` in the terminal.
//
// The exercise file will be reloaded when you change one of the lines below!
// Try adding a new `println!` and check the updated output in the terminal.

fn main() {
    println!(r#"       Welcome to...                      "#);
    println!(r#"                 _   _ _                  "#);
    println!(r#"  _ __ _   _ ___| |_| (_)_ __   __ _ ___  "#);
    println!(r#" | '__| | | / __| __| | | '_ \ / _` / __| "#);
    println!(r#" | |  | |_| \__ \ |_| | | | | | (_| \__ \ "#);
    println!(r#" |_|   \__,_|___/\__|_|_|_| |_|\__, |___/ "#);
    println!(r#"                               |___/      "#);
    println!();
    println!("This exercise compiles successfully. The remaining exercises contain a compiler");
    println!("or logic error. The central concept behind Rustlings is to fix these errors and");
    println!("solve the exercises. Good luck!");
    println!();
    println!("The file of this exercise is `exercises/00_intro/intro1.rs`. Have a look!");
    println!("The current exercise path will be always shown under the progress bar.");
    println!("You can click on the path to open the exercise file in your editor.");
    println!("my change!!!hei~hei~hei");
    println!("I'm {} years old",24); //be stringified.
    println!("{0} is my name,{1} is my age,my name is {0},my age is {1}","Jay","24"); //Positional arguments

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);


    let number:f64=1.0;
    let width:usize=5;
    println!("{number:0>width$}");
    println!("{number:0<width$}");
}
