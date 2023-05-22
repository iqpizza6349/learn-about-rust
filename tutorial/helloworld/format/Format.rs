
fn main() {
    // if there's only one variable to format
    println!("{} days", 31);
    
    // Specifying an integer inside in `{}` determines witch additional 
    // arguments will be replaced. Arguments start at 0 immediately after 
    // the format string.
    println!("`{0}`'s friend `{1}`. `{1}`'s best friend `{0}`", "Emily", "James");

    // or use like code under below
    println!("{subject} {verb} {object}", 
        object="an apple Box",
        verb="jump over",
        subject="Litte Fox"
    );
    
    // if formating to different formatting can be invoked by specifying the
    // format character after a `:`
    println!("Base 10:                 {}", 69420);    // prints '69420'
    println!("Base 2(binary):          {:b}", 69420);    // prints '10000111100101100'
    println!("Base 8(octal):           {:o}", 69420);    // prints '207454'
    println!("Base 16(hex):            {:x}", 69420);    // prints '10f2c'
    println!("Base 16(hex):            {:X}", 69420);    // prints '10F2C'

    println!("{number:>5}", number=1);      // print 5 spaces before prints

    println!("{number:0<5}", number=1);     // print 5 zeros in right. such as 10000

    // can use named arguments in the format specifier by appending a `$`
    println!("{number:0>width$}", number=1, width=5);   // print 5 zeros in left

    #[allow(dead_code)]     // these means `disable dead_code` which warn against unused module
    struct Structure(i32);

    // will not compiles.. with error
    // println!("this struct `{}` won't print.. cause `Structure` is not implement `fmt::Display`..");

    // For Rust version is same or higher then 1.58
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
