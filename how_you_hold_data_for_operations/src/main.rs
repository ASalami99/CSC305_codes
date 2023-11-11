mod datatypes;
use crate::datatypes::{
    derived::{functions, traits, user_defined},
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
    // println!("\n\nFunctions");
    // functions::run::functions();
    // println!("\n\nClosures");
    // functions::run::closures();
    // println!("\n\nCapture Modes");
    // functions::run::capture_modes();
    // println!("\n\nImmutable Borrows");
    // functions::run::immutable_borrows();
    // println!("\n\nCall Traits");
    // functions::run::call_traits();
    println!("\n\nCapturing");
    functions::run::capturing();
    println!("\n\nAs Input Parameters");
    functions::run::as_input_parameters();
    println!("\n\nType Anonymity");
    functions::run::type_anonymity();
    println!("\n\nInput Functions");
    functions::run::input_functions();
    println!("\n\nOutput Parameters");
    functions::run::as_output_parameters();
    println!("\n\nHigher Order Functions");
    functions::run::higher_order_functions();
    println!("\n\nDiverging Functions");
    functions::run::diverging_functions();
    println!("\n\nTraits");
    traits::run::traits_example();
    println!("\nDerived Traits");
    traits::run::derived_traits();
    println!("\nReturning Traits with dyn");
    traits::run::returning_traits_with_dyn();
    println!("\nDrop");
    traits::run::drop_example();
    println!("\nIterators");
    traits::run::iterators();
    println!("\nSuper Traits");
    traits::run::super_traits();
}
