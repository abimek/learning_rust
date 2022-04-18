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

    fn speak(d: Self::Input) -> Self::Output;
}

/// Both Generics and Associated Types work in most situations, although associated types allows
/// the devleoper to restrict the number of implemenations of said trait on a data type to one. But
/// for generics you can implement it muliple types for different types.
trait Speak2<T> {
    fn speak() -> T;
}

struct Human;

struct Person;

impl Speak1 for Human {
    type Input = String;
    type Output = String;

    fn speak(d: Self::Input) -> Self::Output {
        d + &"HIII".to_string()
    }
}

impl Speak2<String> for Person {
    fn speak() -> String {
        String::from("HOLA")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn genreics_on_trains_overload() {
        let manager = Manager {};
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(*(&data[0..2]), *manager.run(&data[..]));
    }
}
