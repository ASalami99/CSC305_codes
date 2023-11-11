// use std::i64;

// pub fn default_greeting() -> String {
//     let message = String::from("Hello! This is compound.rs!");
//     return message;
// }

pub mod run {
    pub fn tuples() {
        // Tuples can be used as function arguments and as return values.
        fn reverse(pair: (i32, bool)) -> (bool, i32) {
            // `let` can be used to bind the members of a tuple to variables.
            let (int_param, bool_param) = pair;

            (bool_param, int_param)
        }

        // The following struct is for the activity.
        #[derive(Debug)]
        struct Matrix(f32, f32, f32, f32);
        // A tuple with a bunch of different types.
        let long_tuple = (
            1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
        );

        // Values can be extracted from the tuple using tuple indexing.
        println!("Long tuple first value: {}", long_tuple.0);
        println!("Long tuple second value: {}", long_tuple.1);

        // Tuples can be tuple members.
        let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

        // Tuples are printable.
        println!("tuple of tuples: {:?}", tuple_of_tuples);
        // But long Tuples (more than 12 elements) cannot be printed.

        let pair = (1, true);
        println!("Pair is {:?}", pair);

        println!("The reversed pair is {:?}", reverse(pair));

        // To create one element tuples, the comma is required to tell them apart
        // from a literal surrounded by parentheses.
        println!("One element tuple: {:?}", (5u32,));
        println!("Just an integer: {:?}", (5u32));

        // Tuples can be destructured to create bindings.
        let tuple = (1, "hello", 4.5, true);

        let (a, b, c, d) = tuple;
        println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

        let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
        println!("{:?}", matrix);
    }

    pub fn arrays() {
        use std::mem;
        // This function borrows a slice.
        fn analyze_slice(slice: &[i32]) {
            println!("First element of the slice: {}", slice[0]);
            println!("The slice has {} elements", slice.len());
        }

        // Fixed-size array (type signature is superfluous).
        let xs: [i32; 5] = [1, 2, 3, 4, 5];

        // All elements can be initialized to the same value.
        let ys: [i32; 500] = [0; 500];

        // Indexing starts at 0.
        println!("First element of the array: {}", xs[0]);
        println!("Second element of the array: {}", xs[1]);

        // `len` returns the count of elements in the array.
        println!("Number of elements in array: {}", xs.len());

        // Arrays are stack allocated.
        println!("Array occupies {} bytes", mem::size_of_val(&xs));

        // Arrays can be automatically borrowed as slices.
        println!("Borrow the whole array as a slice.");
        analyze_slice(&xs);

        // Slices can point to a section of an array.
        // They are of the form [starting_index..ending_index].
        // `starting_index` is the first position in the slice.
        // `ending_index` is one more than the last position in the slice.
        println!("Borrow a section of the array as a slice.");
        analyze_slice(&ys[1..4]);

        // Example of empty slice `&[]`:
        let empty_array: [u32; 0] = [];
        assert_eq!(&empty_array, &[]);
        assert_eq!(&empty_array, &[][..]); // Same but more verbose

        // Arrays can be safely accessed using `.get`, which returns an
        // `Option`. This can be matched as shown below, or used with
        // `.expect()` if you would like the program to exit with a nice
        // message instead of happily continue.
        for i in 0..xs.len() + 1 {
            // Oops, one element too far!
            match xs.get(i) {
                Some(xval) => println!("{}: {}", i, xval),
                None => println!("Slow down! {} is too far!", i),
            }
        }
    }

    pub fn casting() {
        // Suppress all warnings from casts which overflow.
        #![allow(overflowing_literals)]

        let decimal = 65.4321_f32;

        // Error! No implicit conversion
        //let integer: u8 = decimal;
        // FIXME ^ Comment out this line

        // Explicit conversion
        let integer = decimal as u8;
        let character = integer as char;

        // Error! There are limitations in conversion rules.
        // A float cannot be directly converted to a char.
        //let character = decimal as char;
        // FIXME ^ Comment out this line

        println!("Casting: {} -> {} -> {}", decimal, integer, character);

        // when casting any value to an unsigned type, T,
        // T::MAX + 1 is added or subtracted until the value
        // fits into the new type

        // 1000 already fits in a u16
        println!("1000 as a u16 is: {}", 1000 as u16);

        // 1000 - 256 - 256 - 256 = 232
        // Under the hood, the first 8 least significant bits (LSB) are kept,
        // while the rest towards the most significant bit (MSB) get truncated.
        println!("1000 as a u8 is : {}", 1000 as u8);
        // -1 + 256 = 255
        println!("  -1 as a u8 is : {}", (-1i8) as u8);

        // For positive numbers, this is the same as the modulus
        println!("1000 mod 256 is : {}", 1000 % 256);

        // When casting to a signed type, the (bitwise) result is the same as
        // first casting to the corresponding unsigned type. If the most significant
        // bit of that value is 1, then the value is negative.

        // Unless it already fits, of course.
        println!(" 128 as a i16 is: {}", 128 as i16);

        // In boundary case 128 value in 8-bit two's complement representation is -128
        println!(" 128 as a i8 is : {}", 128 as i8);

        // repeating the example above
        // 1000 as u8 -> 232
        println!("1000 as a u8 is : {}", 1000 as u8);
        // and the value of 232 in 8-bit two's complement representation is -24
        println!(" 232 as a i8 is : {}", 232 as i8);

        // Since Rust 1.45, the `as` keyword performs a *saturating cast*
        // when casting from float to int. If the floating point value exceeds
        // the upper bound or is less than the lower bound, the returned value
        // will be equal to the bound crossed.

        // 300.0 as u8 is 255
        println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);
        // -100.0 as u8 is 0
        println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
        // nan as u8 is 0
        println!("   nan as u8 is : {}", f32::NAN as u8);

        // This behavior incurs a small runtime cost and can be avoided
        // with unsafe methods, however the results might overflow and
        // return **unsound values**. Use these methods wisely:
        unsafe {
            // 300.0 as u8 is 44
            println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
            // -100.0 as u8 is 156
            println!(
                "-100.0 as u8 is : {}",
                (-100.0_f32).to_int_unchecked::<u8>()
            );
            // nan as u8 is 0
            println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
        }
    }

    pub fn aliasing() {
        // `NanoSecond`, `Inch`, and `U64` are new names for `u64`.
        type NanoSecond = u64;
        type Inch = u64;
        type U64 = u64;

        // `NanoSecond` = `Inch` = `U64` = `u64`.
        let nanoseconds: NanoSecond = 5 as U64;
        let inches: Inch = 2 as U64;

        // Note that type aliases *don't* provide any extra type safety, because
        // aliases are *not* new types
        println!(
            "{} nanoseconds + {} inches = {} unit?",
            nanoseconds,
            inches,
            nanoseconds + inches
        );
    }
}
