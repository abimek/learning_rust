/// Traits define shared behaviour between data types, Sometimes their also used as markers by the
/// rust compiler, such as Sync and Send. Im currently currious on how Associated Types benefit the
/// user instead of Generics.
///
/// I just ran into an issue where I struggled to implement a trait that requires a generic onto a
/// struct, I'll play around with that a little.
trait Run<G, T> {
    fn run(&self, data: G) -> T;
}

struct Manager {}
// So apparently this is how I define an implementation of a trait with a generic on a stuct, this
// makes sense since the generics are only on the Interfance
impl Run<&str, i32> for Manager {
    fn run(&self, data: &str) -> i32 {
        data.len() as i32
    }
}
// what happens if I implement the same struct on Manager but with different generics?
impl<'a> Run<&'a str, &'a [u8]> for Manager {
    fn run(&self, data: &'a str) -> &'a [u8] {
        data.as_bytes()
    }
}
// Since the Second implementation doesnt actually work due to my inability to call it isnce its
// overloaded and rust doesnt know which one to call and idk how to strictly specifiy which, I'm
// going to write anouther with a first generic parameter that isnt &str
impl<'a> Run<&'a [u8], &'a [u8]> for Manager {
    fn run(&self, data: &'a [u8]) -> &'a [u8] {
        &data[0..2]
    }
}
/// This trait uses a Associated Type instead of a Generic, you can use a generic type by
/// Self::<type name>
trait Speak1 {
    type Input;
    type Output;

    fn speak(&self, d: Self::Input) -> Self::Output;
}

/// Both Generics and Associated Types work in most situations, although associated types allows
/// the devleoper to restrict the number of implemenations of said trait on a data type to one. But
/// for generics you can implement it muliple types for different types.
trait Speak2<T> {
    fn speak(&self) -> T;
}

struct Human;
struct Person;

impl Speak1 for Human {
    type Input = String;
    type Output = String;

    fn speak(&self, d: Self::Input) -> Self::Output {
        d + &"HIII".to_string()
    }
}

impl Speak2<String> for Person {
    fn speak(&self) -> String {
        String::from("HOLA")
    }
}

impl Speak2<i32> for Person {
    fn speak(&self) -> i32 {
        28
    }
}

/// Generic Paramters are also an option to allow default implemenation using generics,
/// this is helpful in situations like the Add trait wich allows you to implement the method for
/// any type that you get in so that you can define different add different types together, but
/// simulationiously, it uses an associated type so that for every implemenation of the generic,
/// you know the exact output type, because if there was a generic for the output then you can
/// could add a Human with a Person and return a String, and also add a Human with a Person and
/// return a i32
trait SaySomething<T = String> {
    fn say_it(&self) -> T;
}

struct Abi;

impl SaySomething for Abi {
    fn say_it(&self) -> String {
        String::from("")
    }
}
/// I've seen the keywords Impl and Dyn used a lot in rust programming to implement functionality
/// over a type that implements a specific trait, I BELIEVE that they allow you to utelize traits
/// LIKE types, (Impl in method defs only and Dyn as a type (I THINK))
/// I want to know the difference and how these work, and why one would use one over the other. I
/// also want to play around with how associated types are put into the method signature when you
/// only want a specific value for an associated type.

trait Mood {
    type Item;
    fn mood(&self) -> Self::Item;
}

struct JviGuy;

impl Mood for JviGuy {
    type Item = String;

    fn mood(&self) -> Self::Item {
        String::from("Grumpy")
    }
}

struct Prim;

impl Mood for Prim {
    type Item = i32;
    fn mood(&self) -> Self::Item {
        20
    }
}

/// This works because I do Item = String, let the compiler know that if the Associated Type within
/// any impelementation of Mood isnt String, then it want match -> Error
/// The following Example doesnt work exactly because Mood doesnt have Item specified, meaning Item
/// can be of any type, that means Item, could happen to be a type thats not String (i32)
/// ```compile_fail
/// fn take_impl_without_associated(mood: impl Mood) -> String {
///    mood.mood()
/// }
///```
fn take_impl_with_associated(mood: impl Mood<Item = String>) -> String {
    mood.mood()
}
/// OR you can use a generic to return the value inside mood
fn take_impl_with_associated_and_generic<T>(mood: impl Mood<Item = T>) -> T {
    mood.mood()
}

trait DoSomething {
    fn do_something(&self) -> String;
}

struct Car;

impl DoSomething for Car {
    fn do_something(&self) -> String {
        String::from("Vroom")
    }
}

struct Plane;

impl DoSomething for Plane {
    fn do_something(&self) -> String {
        String::from("FLY")
    }
}
/// Without a reference you cant take in a trait object because apparently, dyn DoSomething refers
/// to the actual data type which could be of any size, so you need to get the fat pointer through
/// referencing, therefor we add &
fn take_dyn_ret_string(doer: &dyn DoSomething) -> String {
    doer.do_something()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_generics_paramters() {
        let a = Abi {};
        assert_eq!(a.say_it(), String::from(""));
    }

    #[test]
    fn generics_on_traits() {
        let manager = Manager {};
        assert_eq!(5, manager.run("HELLO"));
    }

    /// I'm currently running into an issue due to the fact taht the two implementations of Run on
    /// Manager both have &str as their first type, the compiler can't guess which one I need and
    /// therefor I have to explicitly state which, but idk how. I've been looking for the answer to
    /// this for 20-30 minutes and can't seem to find one.
    // #[test]
    //fn generics_on_traits_different() {
    //    let manager = Manager {};
    //    let data = String::from("WKJL:SHGOIGEHWOIEWHGOWEIHEG");
    //    assert_eq!(&data.as_bytes(), Run::<&str, &[u8]>(&manager, &data[..]));
    //}
    /// So Apparently one can overload aslong as the compiler can infer which function you're
    /// attemtping to call
    #[test]
    fn generics_on_traits_overload() {
        let manager = Manager {};
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(*(&data[0..2]), *manager.run(&data[..]));
    }
    /// When using a generic on a trait, you usally have to specify which method your calling
    /// exactly since the compiler can't guess which one since the method can be implemented
    /// multiple times. This is different from associated types which can be called without extra
    /// syntax because only one version of them can exist.
    #[test]
    fn associated_on_trait() {
        let h = Human;
        h.speak(String::from("Hell"));
    }

    #[test]
    fn take_impl_with_associated_test() {
        let jvi = JviGuy;
        assert_eq!(String::from("Grumpy"), take_impl_with_associated(jvi));
    }
    #[test]
    fn take_impl_with_associated_and_generic_test() {
        let prim = Prim;
        assert_eq!(20 as i32, take_impl_with_associated_and_generic(prim));
    }

    /// To have a vector of different types we need to utalize trait objects since if you use impl
    /// then the vec can only be filled with ONE type that implements that trait. Trait objects
    /// when used as a paramter in a method result in only one specific method being generated
    /// unlike what happens for impl. This is because TraitObjects consist of a reference that points to
    /// the actual data and a reference that points to a table of all the methods on that
    /// specicific trait.
    #[test]
    fn vec_of_different_types() {
        let doers: Vec<Box<dyn DoSomething>> = vec![Box::new(Car), Box::new(Plane)];
        for doer in doers {
            if doer.do_something() == String::from("FLY") {
                assert!(true);
            }
        }
    }

    #[test]
    fn pass_traitobject() {
        let doer: &dyn DoSomething = &Car;
        assert_eq!(String::from("Vroom"), take_dyn_ret_string(doer));
    }
}
