// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.

use std::{
    ffi::OsStr,
    ops::{Deref, DerefMut, MulAssign},
    os::unix::prelude::OsStrExt,
};

// Obtain the number of bytes (not characters) in the given argument.
fn byte_counter<T>(arg: T) -> usize
where
    T: AsRef<OsStr>,
{
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
fn char_counter<T>(arg: T) -> usize
where
    T: AsRef<OsStr>,
{
    let arg_str = arg.as_ref().to_str().unwrap();
    arg_str.chars().count()
}

// Squares a number using `as_mut()`.
fn num_sq<T>(arg: &mut T)
where
    T: DerefMut + Clone,
    T::Target: MulAssign + Clone,
{
    let cloned_value: <T as Deref>::Target = (**arg).clone();
    **arg *= cloned_value;
}

fn main() {
    // You can optionally experiment here.
    let s = String::from("Café au lait");
    println!("Number of characters: {}", char_counter(s.clone()));
    println!("Number of bytes: {}", byte_counter(s.clone()));
    let mut num: Box<u32> = Box::new(3);
    println!("{:?}", num_sq(&mut num));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
