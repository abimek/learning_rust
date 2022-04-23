# Built In Traits

Rust has a lot of built in traits that are helpful to know as a
programmer, this include Marker traits. I would like to experiment with
some of these traits and understand how they work and where to use them.
I'm especially interested in how the Sized and Deref traits work as I've
wondered how Box<T> can have methods called on it that belong on T,
since Box is a struct. I believe this is because of the Deref trait but
I would like to learn more about how this works and where to implement
it in my own code. I'll also experiment with Sized in relation to trait
objects and how it's helpful. My boy Jon Gjengset has an amazing video
on this topic and trait objects in general.
