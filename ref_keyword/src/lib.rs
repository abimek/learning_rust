/// This should be a simple one, ref is helpfull in a lot of situations to stop values from being
/// consumed
///```should_fail
///fn take_string(v: ref String){}
///```
///You are not able to use ref as a parameter
///{expected tpye, found keyword `ref` expected type}

fn take_string_ownership(v: String) {
    let ref temp = v;
    println!("{}", *temp);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn taking_a_reference() {
        let name = String::from("abi");
        let ref my_reference = name;
        //my my_reference2 should be the exact same as my_reference
        //let my_reference2 = &name;

        //WE SHOULD STILL BE ABLE TO USE NAME SINCE WE TOOK A REF
        assert_eq!(name, *my_reference);
    }

    #[test]
    fn calling_take_string_ownership() {
        let name = String::from("abi");
        take_string_ownership(name);
        //Ownership is still taken
        //println!("My name is {}", name);
    }

    #[test]
    fn pattern_matching_consumption() {
        let name = Some(String::from("abi"));
        match name {
            Some(n) => assert_eq!(String::from("abi"), n),
            None => assert!(false),
        }
        // The following statement isnt valid because the match statement consumes the value/ takes
        // ownership of it so it can nolonger be used
        //println!("{}", name.unwrap());
    }

    #[test]
    fn pattern_matching_ref() {
        let name = Some(String::from("abi"));
        match name {
            Some(ref n) => println!("{}", n),
            None => assert!(false),
        }
        //This time I can still use name
        assert_eq!(name.unwrap(), String::from("abi"));
    }
}
