mod datatypes;
use crate::datatypes::{
    derived::user_defined,
    primitive::{compound, scalar},
};

fn main() {
    //println!("{}\n{}\n{}\n", user_defined::default_greeting(), compound::default_greeting(), scalar::default_greeting() );
    //println!("{}\n{}", compound::array_example(), compound::tuple_example());
    println!("Primitive Data Types(Scalars)\nVariables");
    scalar::run::variables();
    println!("\n\nLiterls");
    scalar::run::literals();
    println!("\nPrimitive Data Types(Compounds)\nTuples");
    compound::run::tuples();
    println!("\n\nArrays");
    compound::run::arrays();
    println!("\n\nCasting");
    compound::run::casting();
    println!("\n\nAliasing");
    compound::run::aliasing();
    println!("\n\nDerived Data Types\nStructs");
    user_defined::run::structures();
    println!("\n\nEnums");
    user_defined::run::enumerators();
    println!("\n\nTraits");
    user_defined::run::traits_example();
    println!("\nDerived Traits");
    user_defined::run::derived_traits();
    println!("\nReturning Traits with dyn");
    user_defined::run::returning_traits_with_dyn();
    println!("\nDrop");
    user_defined::run::drop_example();
    println!("\nIterators");
    user_defined::run::iterators();
    println!("\nSuper Traits");
    user_defined::run::super_traits();
    // println!("\n\nFunctions");
    // functions::run::functions();
    println!("\n\nClosures");
    user_defined::run::closures();
    println!("\n\nCapture Modes");
    user_defined::run::capture_modes();
    println!("\n\nImmutable Borrows");
    user_defined::run::immutable_borrows();
    println!("\n\nCall Traits");
    user_defined::run::call_traits();
    println!("\n\nCapturing");
    user_defined::run::capturing();
    println!("\n\nAs Input Parameters");
    user_defined::run::as_input_parameters();
    println!("\n\nType Anonymity");
    user_defined::run::type_anonymity();
    println!("\n\nInput Functions");
    user_defined::run::input_functions();
    println!("\n\nOutput Parameters");
    user_defined::run::as_output_parameters();
    println!("\n\nHigher Order Functions");
    user_defined::run::higher_order_functions();
    println!("\n\nDiverging Functions");
    user_defined::run::diverging_functions();
   
}
