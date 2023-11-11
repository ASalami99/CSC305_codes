pub mod run {
    use std::collections::HashSet;

    pub fn functions() {
        fn main() {
            println!("Hello, world!");
        }

        fn passing_parameters() {
            another_function(5);
        }

        fn another_function(x: i32) {
            println!("The value of x is: {x}");
        }

        fn multiple_parameters() {
            print_labeled_measurement(5, 'h');
        }

        fn print_labeled_measurement(value: i32, unit_label: char) {
            println!("The measurement is: {value}{unit_label}");
        }

        fn expressions() {
            let y = 6;
        }

        // fn main() {
        //     let x = (let y = 6);
        // }

        fn expressions_2() {
            let y = {
                let x = 3;
                x + 1
            };

            println!("The value of y is: {y}");
        }

        fn five() -> i32 {
            5
        }

        fn function_with_return_type() {
            let x = 3;
            x + 1;

            let x = five();

            println!("The value of x is: {x}");
        }

        fn return_type_2() {
            let x = plus_one(5);

            println!("The value of x is: {x}");
        }

        fn plus_one(x: i32) -> i32 {
            x + 1
        }
    }

    pub fn closures() {
        fn f<F: FnOnce() -> String>(g: F) {
            println!("{}", g());
        }

        let mut s = String::from("foo");
        let t = String::from("bar");

        f(|| {
            s += &t;
            s
        });
        // Prints "foobar".
    }

    pub fn capture_modes() {
        struct SetVec {
            set: HashSet<u32>,
            vec: Vec<u32>,
        }

        impl SetVec {
            fn populate(&mut self) {
                let vec = &mut self.vec;
                self.set.iter().for_each(|&n| {
                    vec.push(n);
                })
            }
        }
    }

    pub fn immutable_borrows() {
        let mut b = false;
        let x = &mut b;
        {
            let mut c = || {
                *x = true;
            };
            // The following line is an error:
            // let y = &x;
            c();
        }
        let z = &x;
        println!("")
    }

    pub fn call_traits() {
        let add = |x, y| x + y;

        let mut x = add(5, 7);

        type Binop = fn(i32, i32) -> i32;
        let bo: Binop = add;
        x = bo(5, 7);
    }

    pub fn capturing() {
        use std::mem;

        let color = String::from("green");

        // A closure to print `color` which immediately borrows (`&`) `color` and
        // stores the borrow and closure in the `print` variable. It will remain
        // borrowed until `print` is used the last time.
        //
        // `println!` only requires arguments by immutable reference so it doesn't
        // impose anything more restrictive.
        let print = || println!("`color`: {}", color);

        // Call the closure using the borrow.
        print();

        // `color` can be borrowed immutably again, because the closure only holds
        // an immutable reference to `color`.
        let _reborrow = &color;
        print();

        // A move or reborrow is allowed after the final use of `print`
        let _color_moved = color;

        let mut count = 0;
        // A closure to increment `count` could take either `&mut count` or `count`
        // but `&mut count` is less restrictive so it takes that. Immediately
        // borrows `count`.
        //
        // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
        // calling the closure mutates the closure which requires a `mut`.
        let mut inc = || {
            count += 1;
            println!("`count`: {}", count);
        };

        // Call the closure using a mutable borrow.
        inc();

        // The closure still mutably borrows `count` because it is called later.
        // An attempt to reborrow will lead to an error.
        // let _reborrow = &count;
        // ^ TODO: try uncommenting this line.
        inc();

        // The closure no longer needs to borrow `&mut count`. Therefore, it is
        // possible to reborrow without an error
        let _count_reborrowed = &mut count;

        // A non-copy type.
        let movable = Box::new(3);

        // `mem::drop` requires `T` so this must take by value. A copy type
        // would copy into the closure leaving the original untouched.
        // A non-copy must move and so `movable` immediately moves into
        // the closure.
        let consume = || {
            println!("`movable`: {:?}", movable);
            mem::drop(movable);
        };

        // `consume` consumes the variable so this can only be called once.
        consume();
        //consume();
        // ^ TODO: Try uncommenting this line.

        // `Vec` has non-copy semantics.
        let haystack = vec![1, 2, 3];

        let contains = move |needle| haystack.contains(needle);

        println!("{}", contains(&1));
        println!("{}", contains(&4));

        // println!("There're {} elements in vec", haystack.len());
        // ^ Uncommenting above line will result in compile-time error
        // because borrow checker doesn't allow re-using variable after it
        // has been moved.

        // Removing `move` from closure's signature will cause closure
        // to borrow _haystack_ variable immutably, hence _haystack_ is still
        // available and uncommenting above line will not cause an error.
    }

    pub fn as_input_parameters() {
        // A function which takes a closure as an argument and calls it.
        // <F> denotes that F is a "Generic type parameter"
        fn apply<F>(f: F)
        where
            // The closure takes no input and returns nothing.
            F: FnOnce(),
        {
            // ^ TODO: Try changing this to `Fn` or `FnMut`.

            f();
        }

        // A function which takes a closure and returns an `i32`.
        fn apply_to_3<F>(f: F) -> i32
        where
            // The closure takes an `i32` and returns an `i32`.
            F: Fn(i32) -> i32,
        {
            f(3)
        }

        use std::mem;
        //fn main
        let greeting = "hello";
        // A non-copy type.
        // `to_owned` creates owned data from borrowed one
        let mut farewell = "goodbye".to_owned();

        // Capture 2 variables: `greeting` by reference and
        // `farewell` by value.
        let diary = || {
            // `greeting` is by reference: requires `Fn`.
            println!("I said {}.", greeting);

            // Mutation forces `farewell` to be captured by
            // mutable reference. Now requires `FnMut`.
            farewell.push_str("!!!");
            println!("Then I screamed {}.", farewell);
            println!("Now I can sleep. zzzzz");

            // Manually calling drop forces `farewell` to
            // be captured by value. Now requires `FnOnce`.
            mem::drop(farewell);
        };

        // Call the function which applies the closure.
        apply(diary);

        // `double` satisfies `apply_to_3`'s trait bound
        let double = |x| 2 * x;

        println!("3 doubled: {}", apply_to_3(double));
    }

    pub fn type_anonymity() {
        // `F` must implement `Fn` for a closure which takes no
        // inputs and returns nothing - exactly what is required
        // for `print`.
        fn apply<F>(f: F)
        where
            F: Fn(),
        {
            f();
        }
        //fn main
        let x = 7;

        // Capture `x` into an anonymous type and implement
        // `Fn` for it. Store it in `print`.
        let print = || println!("{}", x);

        apply(print);
    }

    pub fn input_functions() {
        // Define a function which takes a generic `F` argument
        // bounded by `Fn`, and calls it
        fn call_me<F: Fn()>(f: F) {
            f();
        }

        // Define a wrapper function satisfying the `Fn` bound
        fn function() {
            println!("I'm a function!");
        }
        //fn main
        // Define a closure satisfying the `Fn` bound
        let closure = || println!("I'm a closure!");

        call_me(closure);
        call_me(function);
    }

    pub fn as_output_parameters() {
        fn create_fn() -> impl Fn() {
            let text = "Fn".to_owned();

            move || println!("This is a: {}", text)
        }

        fn create_fnmut() -> impl FnMut() {
            let text = "FnMut".to_owned();

            move || println!("This is a: {}", text)
        }

        fn create_fnonce() -> impl FnOnce() {
            let text = "FnOnce".to_owned();

            move || println!("This is a: {}", text)
        }
        //fn main
        let fn_plain = create_fn();
        let mut fn_mut = create_fnmut();
        let fn_once = create_fnonce();

        fn_plain();
        fn_mut();
        fn_once();
    }

    pub fn higher_order_functions() {
        fn is_odd(n: u32) -> bool {
            n % 2 == 1
        }

        //fn main() {
        println!("Find the sum of all the squared odd numbers under 1000");
        let upper = 1000;

        // Imperative approach
        // Declare accumulator variable
        let mut acc = 0;
        // Iterate: 0, 1, 2, ... to infinity
        for n in 0.. {
            // Square the number
            let n_squared = n * n;

            if n_squared >= upper {
                // Break loop if exceeded the upper limit
                break;
            } else if is_odd(n_squared) {
                // Accumulate value, if it's odd
                acc += n_squared;
            }
        }
        println!("imperative style: {}", acc);

        // Functional approach
        let sum_of_squared_odd_numbers: u32 = (0..)
            .map(|n| n * n) // All natural numbers squared
            .take_while(|&n_squared| n_squared < upper) // Below upper limit
            .filter(|&n_squared| is_odd(n_squared)) // That are odd
            .sum(); // Sum them
        println!("functional style: {}", sum_of_squared_odd_numbers);
    }

    pub fn diverging_functions() {
        

        //fn main() {
        // let x: ! = panic!("This call never returns.");
        //println!("You will never see this line!");
        //}

        //fn main() {
        fn sum_odd_numbers(up_to: u32) -> u32 {
            let mut acc = 0;
            for i in 0..up_to {
                // Notice that the return type of this match expression must be u32
                // because of the type of the "addition" variable.
                let addition: u32 = match i % 2 == 1 {
                    // The "i" variable is of type u32, which is perfectly fine.
                    true => i,
                    // On the other hand, the "continue" expression does not return
                    // u32, but it is still fine, because it never returns and therefore
                    // does not violate the type requirements of the match expression.
                    false => continue,
                };
                acc += addition;
            }
            acc
        }
        println!(
            "Sum of odd numbers up to 9 (excluding): {}",
            sum_odd_numbers(9)
        );
   // }
    }
}
