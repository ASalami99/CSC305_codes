// pub fn default_greeting() -> String {
//     let message = String::from("Hello! This is scalar.rs!");
//     return message;
// }

pub mod run {
    pub fn variables() {
        // Variables can be type annotated.
        #![allow(unused_assignments)]
        let logical: bool = true;
        println!("The logical value is {}", logical);

        let a_float: f64 = 1.0; // Regular annotation
        println!("The float value is {}", a_float);

        let an_integer = 5i32; // Suffix annotation
        println!("The integer value is {}", an_integer);

        // Or a default will be used.
        let default_float = 3.0; // `f64`
        println!("The default float value is {}", default_float);
        let default_integer = 7; // `i32`
        println!("The default integer value is {}", default_integer);

        // A type can also be inferred from context.
        let mut inferred_type = 12; // Type i64 is inferred from another line.
        inferred_type = 4294967296i64;
        println!("The inferred_type value is {}", inferred_type);

        // A mutable variable's value can be changed.
        let mut mutable = 12; // Mutable `i32`
        mutable = 21;
        println!("The mutable value is {}", mutable);

        // Variables can be overwritten with shadowing.
        let mutable = true;
        println!("The new mutable value is {}", mutable)
    }

    pub fn literals() {
        // Integer addition
        println!("1 + 2 = {}", 1u32 + 2);
    
        // Integer subtraction
        println!("1 - 2 = {}", 1i32 - 2);
        // TODO ^ Try changing `1i32` to `1u32` to see why the type is important
    
        // Scientific notation
        println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);
    
        // Short-circuiting boolean logic
        println!("true AND false is {}", true && false);
        println!("true OR false is {}", true || false);
        println!("NOT true is {}", !true);
    
        // Bitwise operations
        println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
        println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
        println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
        println!("1 << 5 is {}", 1u32 << 5);
        println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    
        // Use underscores to improve readability!
        println!("One million is written as {}", 1_000_000u32);
    }
}
