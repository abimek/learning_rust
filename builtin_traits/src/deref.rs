use std::ops::Deref;

/// DEREF TRAIT
/// The deref trait is a very important trait as it lets you overload the deref operator in rust,
/// it also plays a role in type coercion which I would like to tests out and expirement with so
/// that I can understand type coercion.

struct StringHolder<'a> {
    string: &'a str,
}

/// I could use the From trait from the standard library but thats not relavent rn sooooo
impl<'a> StringHolder<'a> {
    fn from(data: &'a str) -> Self {
        StringHolder { string: data }
    }
}

fn take_string_holder(holder: StringHolder) -> String {
    holder.string.to_string()
}

fn take_any_str(holder: &str) -> String {
    holder.to_string()
}

/// If we implement Deref for BetterStringHolder, we end up making better and more developer
/// friendly data structure
struct BetterStringHolder<'a> {
    string: &'a str,
}

impl<'a> BetterStringHolder<'a> {
    fn from(data: &'a str) -> Self {
        BetterStringHolder { string: data }
    }

    fn get_value(&self) -> String {
        self.string.to_string()
    }
}

impl<'a> Deref for BetterStringHolder<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.string
    }
}

/// Im curious on how Smart Pointers generic over some type T utalize Deref so that you can call
/// call T methods on the smart point.
/// 1 min later
/// So appranetly the compile lets you call any method of T where Deref<Target=T> on your
/// structure.

fn call_str_method_on_better_string_holder() {
    let holder = BetterStringHolder::from("Abi");
    let i = holder.len();
    print!("{}", i);
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn pass_string_holder() {
        let holder = StringHolder::from("Abi");
        assert_eq!(String::from("Abi"), take_string_holder(holder));
    }

    /// If we pass in a reference to a String, then thats valid, but if we pass a reference of
    /// StringHolder it's not!
    /// ```compile_fail
    /// fn pass_in_string_holder_to_take_any_str() {
    ///    let holder = StringHolder::from("Abi");
    ///    assert_eq!(String::from("Abi"), take_any_str(&holder));
    /// }
    /// ```
    #[test]
    fn pass_in_string_to_take_any_str() {
        let st = String::from("Jvi");
        assert_eq!(String::from("Jvi"), take_any_str(&st));
    }

    /// If we pass in a reference of BetterStringHolder, not it should work
    #[test]
    fn pass_in_better_string_holder_to_take_any_str() {
        let holder = BetterStringHolder::from("Abi");
        assert_eq!(String::from("Abi"), take_any_str(&holder));
    }

    #[test]
    fn better_string_coercion() {
        let holder = BetterStringHolder::from("Abi");
        let c = &holder;
        assert_eq!(String::from("Abi"), c.get_value());
    }
}
