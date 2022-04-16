//! Q1 & Q2 & Q3 are answered here
//! I've been rather curious on how lifetimes on structs work, do they make it
//! so that the struct only lives as long as its shortest lifetimes. I believe
//! so because if we move a struct with lifetimes in a smaller scope into a
//! larger scope it would no longer be valid. Which is the case in this test
//! which leads to a compile time error with us not being allowed to print out
//! the Person struct.
//! ```compile_fail
//!
//! #[derive(Debug)]
//! struct Person<'a, 'b> {
//!     name: &'a String,
//!     age: &'b usize,
//! }
//!
//! impl<'a, 'b> Person<'a, 'b> {
//!     fn new(name: &'a String, age: &'b usize) -> Self {
//!         Person { name, age }
//!     }
//! }
//!
//! fn struct_lifetime_length() {
//!     let name = String::from("abi");
//!     let x: Person;
//!     {
//!         let age: usize = 16;
//!         //Because one of the refrences passed in lives shorter then the other,
//!         //when we move the person instance outside of the scope of the smallest
//!         //lifetimes the person struct no longer becomes valid
//!         let abi = Person::new(&name, &age);
//!         x = abi;
//!     }
//!     let j: usize = 16;
//!     println!("{:?}", x);
//!     //this does not compile because the Person struct does not live long enough
//!     //as it only lives as long as its smallest lifetime
//! }
//! ```
//! This basically means structs only live as long as their shortest lifetime

/// MULTIPLE LIFETIMES AND THEIR RELATIONSHIPS
///
/// It appears that the lifetimes do not have relation to one another except for when
/// implicitly declared so by the programmer. This means that the only relation they
/// have is when using the 'c: 'b annotation which states that lifetime 'c will live
/// atleast as lifetime 'b. The only other way is to give them the same lifetime where
/// the lifetime would then be the shortest of the two.
///
/// For our implementation of address in Person, we give the String inside address
/// a different lifetime then Address itself, this allows us to use the lifetime of
/// that string later on. This is helpful in situations where we want to return a
/// lifetime of the String but Address becomes deallocated.
#[derive(Debug)]
struct Address<'address> {
    address: &'address String,
}
#[derive(Debug)]
struct Person<'a, 'b, 'c: 'b> {
    name: &'a String,
    address: &'b Address<'c>,
}

impl<'a, 'b, 'c> Person<'a, 'b, 'c> {
    fn get_address(&self) -> &'b Address {
        self.address
    }

    fn get_name(&self) -> &'a String {
        self.name
    }
}

/// STATIC LIFETIME
/// Static is a reserved lifetime in rust which states that the refrences will live
/// for the entire duration of the application. This has an intresting application in
/// terms of Trait bounds as one can use it to gaurantee that an incoming value has
/// no refrences or refrences that live for the entire program which means the reciever
/// does not have to worry about invalid lifetimes as the lifetimes cannot become invalid.
///
/// In this implementation T can be anything that impelements Clone and has no refrences
/// or refrences with lifetimes that live for the entire program
fn clone_me<T: Clone + 'static>(whatever: &T) -> T {
    whatever.clone()
}

/// This will work as a parameter of clone_me() because it only has static lifetimes and owned
/// data alongside implementing the Clone trait.
#[derive(Clone)]
struct Human {
    name: &'static str,
    age: usize,
}

/// This will work as a parameter of clone_me() because it takes ownership of everything. Since
/// all lifetimes within must have the 'static lifetime and there are no references, then
/// that parameter is met.
#[derive(Clone)]
struct OwningHuman {
    name: String,
    age: usize,
}

/// This will not work as a paramter of clone_me() because it contains a lifetime that is not
/// static meaning the lifetime bound is not met
/// ```compile_fail
///    fn static_bound_error_human() {
///        let aname = String::from("Abi");
///        let barrowing_human = BarrowingHuman {
///            name: &aname,
///            age: 16,
///        };
///        assert_eq!(*(barrowing_human.name), *(clone_me(&barrowing_human).name));
///    }
/// ```
#[derive(Clone)]
struct BarrowingHuman<'a> {
    name: &'a String,
    age: usize,
}

/// LIFETIMES ON GENERICS
/// Although lifetime bounds on generics arently used as often as they were once
/// used due to the compiler inferring them on most situations, it's still good
/// to understand how they work. Lifetime bounds on generics have the Syntax
/// T: 'a, which gaurantees that all the lifetimes within T will live atleast
/// as long as A. There are circumstances where they are still required
/// ```rust
/// trait Job<'a> {
///     type Item;
/// }
///
/// struct SoftwareEngineer<T> {
///    name: String,
///    previous_postions: Vec<T>
/// }
///
/// struct LaterUsed<'a, T>(&'a T);
/// //Since the compiler infeers that for LaterUsed all the lifetimes within T
/// //live aslong as 'a, but without the where clause in our implementation that
/// //might not always be true we must explicitly tell rust that all the lifetimes
/// //in T will live atleast aslong as 'a
/// impl<'a, T> Job<'a> for SoftwareEngineer<T>
///     where T: 'a
/// {
///     type Item =  LaterUsed<'a, T>;
///    
/// }
///
/// ```
///
#[cfg(test)]
mod tests {
    use super::*;

    /// Calling a function that returns a refrence on a struct that contains
    /// refrences means that you get the lifetime specifed by that method. One
    /// can setup their lifetimes in such a way where that returned refrence
    /// lives longer than the actual struct that the method was called on.
    #[test]
    fn method_on_struct_of_refrences() {
        let name = String::from("abi");
        let r;
        {
            let address = String::from("white house");
            let addy = Address { address: &address };
            let abi = Person {
                name: &name,
                address: &addy,
            };
            let n = abi.get_name();
            assert_eq!(String::from("abi"), *n);
            r = n;
        }
        assert_eq!(String::from("abi"), *r);
    }
    #[test]
    fn static_bound_human() {
        let human = Human {
            name: "Abi",
            age: 16,
        };
        assert_eq!(human.name, clone_me(&human).name);
    }

    #[test]
    fn static_bound_owning_human() {
        let owning_human = OwningHuman {
            name: String::from("Abi"),
            age: 16,
        };
        assert_eq!(owning_human.name, clone_me(&owning_human).name);
    }
}
