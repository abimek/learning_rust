///Q1 & Q2 & Q3 are answered here
///I've been rather curious on how lifetimes on structs work, do they make it
///so that the struct only lives as long as its shortest lifetimes. I believe
///so because if we move a struct with lifetimes in a smaller scope into a
///larger scope it would no longer be valid. Which is the case in this test
///which leads to a compile time error with us not being allowed to print out
///the Person struct.
///```
///
///#[derive(Debug)]
///struct Person<'a, 'b> {
///    name: &'a String,
///    age: &'b usize,
///}
///
///impl<'a, 'b> Person<'a, 'b> {
///    fn new(name: &'a String, age: &'b usize) -> Self {
///        Person { name, age }
///    }
///}
///
///fn main() {
///    let name = String::from("abi");
///     let x: Person;
///     {
///         let age: usize = 16;
///         //Because one of the refrences passed in lives shorter then the other,
///         //when we move the person instance outside of the scope of the smallest
///         //lifetimes the person struct no longer becomes valid
///         let abi = Person::new(&name, &age);
///         x = abi;
///     }
///     let j: usize = 16;
///     assert_eq!(format!("{:?}", x), format!("{:?}", Person::new(&name, &j)));
///}
///```
///This basically means structs only live as long as their shortest lifetime

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
}
