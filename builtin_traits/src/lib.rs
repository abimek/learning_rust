mod deref;

/// SIZED
/// The sized trait is a very unique trait in rust as its an auto trait that signifies whether a
/// type is sized or not, it's implied on every type paramter where it can be. It can be used alongside
/// TraitObjects to opt out of trait objects as adding the Sized bound makes it incapatible with
/// TraitObjects because TraitObjects dont have a known size at compile time. This trait can be
/// opted out of by using ?Sized which basically means could or couldnt not be sized. This
/// basically casts a Sized object into a Not Sized one.

/// This trait
trait Speak {
    fn speak(&self) -> String;
}

struct Adult;

impl Speak for Adult {
    fn speak(&self) -> String {
        String::from("Hello Good Sir")
    }
}

struct Baby;

impl Speak for Baby {
    fn speak(&self) -> String {
        String::from("Gooo GOo")
    }
}

fn get_vec_of_speakers() -> Vec<Box<dyn Speak>> {
    vec![Box::new(Baby), Box::new(Adult)]
}

/// Another Speaker Trait, but this one cant be made into a trait object unless we bound Self to
/// sized on pitch

trait Speak2 {
    fn speak(&self) -> String;

    /// Here we have an associated function. But first lets take a moment to think about how Trait
    /// Objects dont have a specific size. The fact that trait objects dont have a specific size is
    /// why bounding Sized to Self makes it so that pitch can't be put into the vtable, a
    /// TraitObjects isnt sized meaning you cant call pitch on a trait objects. Now rust could
    /// probably implement pitch, but they would need to do some king of weird syntax like
    /// (s)::speak() where s is a Trait Object of Speak2, but if you need an instance of a trait to
    /// call an associated function that removes the whole point of associated functions. This is
    /// why you cant have an associated function in a trait and use that trait as a TraitObject.
    fn pitch() -> i32
    where
        Self: Sized;
}

impl Speak2 for Adult {
    fn speak(&self) -> String {
        String::from("Hello Good Sir")
    }

    fn pitch() -> i32 {
        20
    }
}

impl Speak2 for Baby {
    fn speak(&self) -> String {
        String::from("Gooo GOo")
    }

    fn pitch() -> i32 {
        400
    }
}

fn get_vec_of_speakers2() -> Vec<Box<dyn Speak2>> {
    vec![Box::new(Baby), Box::new(Adult)]
}

/// this is a type that is unsized
/// This type can not be instantiated
struct MyUnsizedType {
    bytes: [u8],
}

struct MyUnsizedTypeInstantiatable<T: ?Sized + AsRef<[u8]>> {
    bytes: T,
}

impl<T: ?Sized + AsRef<[u8]>> Speak for MyUnsizedTypeInstantiatable<T> {
    fn speak(&self) -> String {
        String::from("idc")
    }
}

impl Speak for MyUnsizedType {
    fn speak(&self) -> String {
        String::from("idc")
    }
}

/// opting out of sized on a trait
/// Sized can be opted out of using the ?Sized syntax, traits are by default ?Sized which is why
/// they by default can be used as Trait Objects, The FOllowing piece of code cant be used as a
/// trait objects
/// ```compile_fail
///
/// trait Example
/// where
///    Self: Sized,
/// {
///    fn give_example(&self) -> i32;
/// }
/// struct Exampler;
///
/// impl Example for Exampler {
///     fn give_example(&self) -> i32 {
///         35
///     }
/// }
///
/// fn not_trait_object() {
///     let x: &dyn Example = &Exampler;
/// }
/// ```

trait Example
where
    Self: Sized,
{
    fn give_example(&self) -> i32;
}

/// Sized and Generics
/// Generics are by defaulat sized as if you use a generic as paramter and it opts out of Sized
/// (?Sized) then that means it cant be used as a paramter since it doesnt have a concrete size
/// which rust needs for allocating memory on the stack. In this situation we can bound to T because
/// associated types dont better for staticaly dispatched things.
fn take_speak2<T: Speak2>(thetype: T) -> String {
    thetype.speak()
}

/// Without relaxing the Sized bound here we wouldn't be able to take in T even if T was hiding
/// behind a reference since rust bounds all generics to Sized
fn take_speak<T: Speak + ?Sized>(thespeaker: &T) -> String {
    thespeaker.speak()
}

/// We can take try to see what happens if we pass in MyUnsizedType which contains a str which has
/// a size not know at compilation to a function thats generic over T.
fn call_take_speak() {
    let s: MyUnsizedTypeInstantiatable<[u8; 3]> = MyUnsizedTypeInstantiatable {
        bytes: [20, 30, 40],
    };
    let unsizeds: &MyUnsizedTypeInstantiatable<[u8]> = &s;
    // If we dereference unsizeds here then our type would have an unknown size, so we need to hide
    // it behind reference
    take_speak(unsizeds);
}
/// ----------------------------------------------------------------------------------------------
/// ----------------------------------------------------------------------------------------------
/// ----------------------------------------------------------------------------------------------
/// ----------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_vec_speakers() {
        let vec = get_vec_of_speakers();
        for v in vec {
            if v.speak() == String::from("Gooo GOo") {
                assert!(true);
                return;
            }
        }
        assert!(false);
    }

    #[test]
    fn default_vec_speakers2() {
        let vec = get_vec_of_speakers2();
        for v in vec {
            if v.speak() == String::from("Gooo GOo") {
                assert!(true);
                return;
            }
        }
        assert!(false);
    }
}
