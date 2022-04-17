# Slices
** Referencing Portions of Collections*

Slices are references to some portion of a collection such as an array,
String, or Vector, they could even be references to a portion of the
binary which is what occurs when we assign a string literal to a
variable. String slices don't take ownership of the values which they
point at so they tend to be more memory efficient.

### Questions


##### Question 1
I'm curious on what happens if I use [T] as a parammeter instead of &[T]
since slices are references. Is [T] not valid?

```rust

fn foo(maybe_invalid: [i32]){
}

fn barr(not_invalid: &[i32]){
}
```

##### Question 2
I see that slices are used a lot for buffers and byte arrays, I'm
curious on how exactly this works.



### Conclusion

Even though I haven't looked into how the Deref Trait is used in this
context since its really important on the specific reasoning why read
and write and other structs use slices, I certainly will look into that
when I later make a crate specifically about that. I learned that you
cant just take in an [i32] since the compiler wont be able to know the
actual length, a better way would either be to do [i32; 30]. This is
helpful as now I know where and why to use slices, I will definitely be
implementing them in my future projects. 

