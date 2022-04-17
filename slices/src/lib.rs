/// READ AND WRITE STDIO BUFFER SLICES
/// Slices are used to read and write inside the Read and Write traits within std::io. For the read
/// trait, it takes in a &[u8] which isn't mutable but can be read which is perfect for the role. I
/// did begin to question why not just pass in some arbitrary struct, but then i realised that
/// a lot of the data types within rust can be coerced into a slice so this would allow reading of
/// more then just one type. And the Write is similar but it takes in a &mut [u8] allowing for the
/// function to write to data since its mutable
/// -------------------------------------------------------------------------------------------
/// This function takes a integar slice referance.
/// I'm currious if I can pass in a slice to an usize array AND a slice to a usize vec.
/// (2 minutes later)
/// Yes I guess I can pass in a vec, I believe this is because in the Deref implementation on Vec
/// it returns a slice.
fn first_two(v: &[usize]) -> &[usize] {
    &v[0..2]
}
///I just came accross something intresting, for the function integar_slices, if i do *v[0], that
///works, which I beleive means rust dereferences v for me. I think this happens because if I
///explicitlly dereferance v (*v)[0], that also works. TODO Might have to look into this some more.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_array_slice() {
        let abis_numbers = [1, 0, 0, 4, 2, 7, 3];
        let ft = first_two(&abis_numbers[3..7]);
        assert_eq!(ft, &abis_numbers[3..5]);
    }
    /// this test passed so you can take in a slice and the user can pass in a slice from a vec or
    /// int, this could be helpfull, this makes me wonder if you can pass in a vec as an int array.
    /// WELL NOW, Apparently you cant just pass arrays into functions, you have to pass in a
    /// mutable reference. This is probably because you wont be able to know the size and rust
    /// needs to know its size. This can be fixed by using the type [type; length] inside the
    /// method signature so that rust kows the exact size. I also find it interesting that the
    /// actual type of an array is [T]
    #[test]
    fn vec_array_slice() {
        let abis_vec = vec![1, 0, 0, 4, 2, 7, 3];
        let ft = first_two(&abis_vec[3..7]);
        assert_eq!(ft, &abis_vec[3..5]);
    }
}
