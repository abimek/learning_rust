# lifetimes

**Ensuring all barrows are valid and that invalid references are not
held**

Lifetime declarations are used by the rust compiler to guarantee that rusts core principal of memory safety are insured in circumstances where the compiler can not deduce the lifetime of objects.


### Questions
I've constantly seen lifetimes used and understand a them slightly but I've had many questions about how they should be used. 

##### Q1
When declaring a struct that holds a references to some data type, such as

```rust
struct Person<'a> {
    name: &'a String
}
```
Does this ensure that the Person struct will only live as long as the field **name**, or is there something else that occurs?

##### Q2
When declaring a struct that holds multiple references, is the lifetime of the struct the shortest of the two lifetimes of the references?

```rust
struct Person<'a> {
    name: &'a String,
    mothers_name: &'a String
}
```

##### Q3
If we declare a struct with multiple references with different lifetimes, what happens then, does the compiler ensure that the Person struct only lives as long as the shortest of the two lifetime. How exactly is a situation like this helpful?
```rust
struct Person<'a, 'b> {
    name: &'a String,
    address: &'b String
}
```
##### Q4
If we call a method on a struct to return a refrence to its content,
does that refrence outlive the struct.

```rust
struct Address<'address> {
    address: &'address String
}

struct Person<'a, 'b>{
    name: &'a String,
    address: &'b Address<'b>
}

impl<'a, 'b> Person<'a, 'b> {
    //fn new()

    fn get_address(&self) -> &'b String {
	self.address;
    }
}
```

### Conclusion

I've learned that lifetime annotation isn't just for dictating how long
a specifc struct will live, it also gives the developer control over
what values get retuned and how long said values will live. This means
that the developer can choose to allow a said return type to live as long
as the struct that the method was called on or a specific lifetime of a
refrence within the struct. I love the system which rust has developed
to keep refrences valid and the system to make relationships between
different lifetimes. 

