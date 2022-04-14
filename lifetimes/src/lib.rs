//! Q1 & Q2 & Q3 are answered here
//! I've been rather curious on how lifetimes on structs work, do they make it
//! so that the struct only lives as long as its shortest lifetimes. I believe
//! so because if we move a struct with lifetimes in a smaller scope into a
//! larger scope it would no longer be valid. Which is the case in this test
//! which leads to a compile time error with us not being allowed to print out
//! the Person struct.
//! ```
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
//!
//! It appears that the lifetimes do not have relation to one another except for when
//! implicitly declared so by the programmer. This means that the only relation they
//! have is when using the 'c: 'b annotation which states that lifetime 'c will live
//! atleast as lifetime 'b. The only other way is to give them the same lifetime where
//! the lifetime would then be the shortest of the two.

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

#[cfg(test)]
mod tests {
    use super::*;

    ///One can simply
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
}
