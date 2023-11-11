pub mod run {
    pub fn traits_example() {
        struct Sheep {
            naked: bool,
            name: &'static str,
        }

        trait Animal {
            // Associated function signature; `Self` refers to the implementor type.
            fn new(name: &'static str) -> Self;

            // Method signatures; these will return a string.
            fn name(&self) -> &'static str;
            fn noise(&self) -> &'static str;

            // Traits can provide default method definitions.
            fn talk(&self) {
                println!("{} says {}", self.name(), self.noise());
            }
        }

        impl Sheep {
            fn is_naked(&self) -> bool {
                self.naked
            }

            fn shear(&mut self) {
                if self.is_naked() {
                    // Implementor methods can use the implementor's trait methods.
                    println!("{} is already naked...", self.name());
                } else {
                    println!("{} gets a haircut!", self.name);

                    self.naked = true;
                }
            }
        }

        // Implement the `Animal` trait for `Sheep`.
        impl Animal for Sheep {
            // `Self` is the implementor type: `Sheep`.
            fn new(name: &'static str) -> Sheep {
                Sheep {
                    name: name,
                    naked: false,
                }
            }

            fn name(&self) -> &'static str {
                self.name
            }

            fn noise(&self) -> &'static str {
                if self.is_naked() {
                    "baaaaah?"
                } else {
                    "baaaaah!"
                }
            }

            // Default trait methods can be overridden.
            fn talk(&self) {
                // For example, we can add some quiet contemplation.
                println!("{} pauses briefly... {}", self.name, self.noise());
            }
        }

        //fn main() {
        // Type annotation is necessary in this case.
        let mut dolly: Sheep = Animal::new("Dolly");
        // TODO ^ Try removing the type annotations.

        dolly.talk();
        dolly.shear();
        dolly.talk();
        //}
    }

    pub fn derived_traits() {
        // `Centimeters`, a tuple struct that can be compared
        #[derive(PartialEq, PartialOrd)]
        struct Centimeters(f64);

        // `Inches`, a tuple struct that can be printed
        #[derive(Debug)]
        struct Inches(i32);

        impl Inches {
            fn to_centimeters(&self) -> Centimeters {
                let &Inches(inches) = self;

                Centimeters(inches as f64 * 2.54)
            }
        }

        // `Seconds`, a tuple struct with no additional attributes
        struct Seconds(i32);

        //fn main() {
        let _one_second = Seconds(1);

        // Error: `Seconds` can't be printed; it doesn't implement the `Debug` trait
        //println!("One second looks like: {:?}", _one_second);
        // TODO ^ Try uncommenting this line

        // Error: `Seconds` can't be compared; it doesn't implement the `PartialEq` trait
        //let _this_is_true = (_one_second == _one_second);
        // TODO ^ Try uncommenting this line

        let foot = Inches(12);

        println!("One foot equals {:?}", foot);

        let meter = Centimeters(100.0);

        let cmp = if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

        println!("One foot is {} than one meter.", cmp);
        //}
    }

    pub fn returning_traits_with_dyn() {
        struct Sheep {}
        struct Cow {}

        trait Animal {
            // Instance method signature
            fn noise(&self) -> &'static str;
        }

        // Implement the `Animal` trait for `Sheep`.
        impl Animal for Sheep {
            fn noise(&self) -> &'static str {
                "baaaaah!"
            }
        }

        // Implement the `Animal` trait for `Cow`.
        impl Animal for Cow {
            fn noise(&self) -> &'static str {
                "moooooo!"
            }
        }

        // Returns some struct that implements Animal, but we don't know which one at compile time.
        fn random_animal(random_number: f64) -> Box<dyn Animal> {
            if random_number < 0.5 {
                Box::new(Sheep {})
            } else {
                Box::new(Cow {})
            }
        }

        //fn main() {
        let random_number = 0.234;
        let animal = random_animal(random_number);
        println!(
            "You've randomly chosen an animal, and it says {}",
            animal.noise()
        );
        // }
    }

    pub fn drop_example() {
        struct Droppable {
            name: &'static str,
        }

        // This trivial implementation of `drop` adds a print to console.
        impl Drop for Droppable {
            fn drop(&mut self) {
                println!("> Dropping {}", self.name);
            }
        }

        //fn main() {
        let _a = Droppable { name: "a" };

        // block A
        {
            let _b = Droppable { name: "b" };

            // block B
            {
                let _c = Droppable { name: "c" };
                let _d = Droppable { name: "d" };

                println!("Exiting block B");
            }
            println!("Just exited block B");

            println!("Exiting block A");
        }
        println!("Just exited block A");

        // Variable can be manually dropped using the `drop` function
        //drop(_a);
        // TODO ^ Try commenting this line

        println!("end of the main function");

        // `_a` *won't* be `drop`ed again here, because it already has been
        // (manually) `drop`ed
        //}
    }

    pub fn iterators() {
        struct Fibonacci {
            curr: u32,
            next: u32,
        }

        // Implement `Iterator` for `Fibonacci`.
        // The `Iterator` trait only requires a method to be defined for the `next` element.
        impl Iterator for Fibonacci {
            // We can refer to this type using Self::Item
            type Item = u32;

            // Here, we define the sequence using `.curr` and `.next`.
            // The return type is `Option<T>`:
            //     * When the `Iterator` is finished, `None` is returned.
            //     * Otherwise, the next value is wrapped in `Some` and returned.
            // We use Self::Item in the return type, so we can change
            // the type without having to update the function signatures.
            fn next(&mut self) -> Option<Self::Item> {
                let current = self.curr;

                self.curr = self.next;
                self.next = current + self.next;

                // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
                // will never return `None`, and `Some` is always returned.
                Some(current)
            }
        }

        // Returns a Fibonacci sequence generator
        fn fibonacci() -> Fibonacci {
            Fibonacci { curr: 0, next: 1 }
        }

        //fn main() {
            // `0..3` is an `Iterator` that generates: 0, 1, and 2.
            let mut sequence = 0..3;

            println!("Four consecutive `next` calls on 0..3");
            println!("> {:?}", sequence.next());
            println!("> {:?}", sequence.next());
            println!("> {:?}", sequence.next());
            println!("> {:?}", sequence.next());

            // `for` works through an `Iterator` until it returns `None`.
            // Each `Some` value is unwrapped and bound to a variable (here, `i`).
            println!("Iterate through 0..3 using `for`");
            for i in 0..3 {
                println!("> {}", i);
            }

            // The `take(n)` method reduces an `Iterator` to its first `n` terms.
            println!("The first four terms of the Fibonacci sequence are: ");
            for i in fibonacci().take(4) {
                println!("> {}", i);
            }

            // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
            println!("The next four terms of the Fibonacci sequence are: ");
            for i in fibonacci().skip(4).take(4) {
                println!("> {}", i);
            }

            let array = [1u32, 3, 3, 7];

            // The `iter` method produces an `Iterator` over an array/slice.
            println!("Iterate the following array {:?}", &array);
            for i in array.iter() {
                println!("> {}", i);
            }
        //}
    }

    pub fn super_traits(){
        #![allow(dead_code)]
        trait Person {
            fn name(&self) -> String;
        }
        
        // Person is a supertrait of Student.
        // Implementing Student requires you to also impl Person.
        trait Student: Person {
            fn university(&self) -> String;
        }
        
        trait Programmer {
            fn fav_language(&self) -> String;
        }
        
        // CompSciStudent (computer science student) is a subtrait of both Programmer 
        // and Student. Implementing CompSciStudent requires you to impl both supertraits.
        trait CompSciStudent: Programmer + Student {
            fn git_username(&self) -> String;
        }
        
        fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
            format!(
                "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
                student.name(),
                student.university(),
                student.fav_language(),
                student.git_username()
            )
        }
    }
}
