# `memusage`

`memusage` is a (some) batteries included memory usage reporting.
This crate includes a simple trait `MemoryReport` that makes it easy to track 
memory usage in your projects.

## Usage

`Cargo.toml`

```
[dependencies]
memusage = "0.1.0"
```

The `MemoryReport` trait includes three associated methods: `direct`, `indirect` and `children`.

### `direct` memory report

Function signature: `fn direct() -> usize`

The spirit (and implementation) of this memory report is to be equivalent to `core::mem::size_of::<Self>()`.
This method is used mainly for ease of use.

### `indirect` memory report

Function signature: `fn indirect(&self) -> usize`

This function is used to report the amount of heap (and stack in case of arrays) allocated memory of structs.
This method has an associated constant in the trait (`ALLOC: bool`) which is used to indicate that the object
implementing the trait owns memory other than the struct itself.

For example, a `Vec<T: MemoryReport>` will report the full memory capacity it has reserved: `self.capacity() * T::direct()`.

Code example:
```
let vec_of_usizes: Vec<usize> = vec![1, 2, 3, 4];

println!("{}", vec_of_usizes.indirect());
```

### `children` memory report

Function signature: `fn children(&self) -> usize`

This function is used to report the amount of heap allocated memory of the children of a struct.
This method has an associated constant in the trait (`CHILD: bool`) which is used to indicate that the
object implementing the trait has children that may allocate memory.

For example, a `Vec<T: MemoryReport>` or `&[T]` will report the memory that the elements they contain have allocated.
A `Vec<usize>` will report 0, as an `usize` does not allocate memory, but a `&[Vec<usize>]` will report some memory, as
a `Vec<usize>` does allocate memory.

Code example:
```
let vec_of_vecs_of_usize: Vec<Vec<usize>> = vec![vec![1], vec![2], vec![3]];

println!("{}", vec_of_vecs_of_usize.children());
```


## Implementors

By default, this crate has some batteries included, and the `MemoryReport` trait is already implemented for
some `core` and `std` objects.

See below a list of all default implementors. This list may change in the future.

#### Integers

* i8
* i16
* i32
* i64
* i128
* isize

#### Unsigned integers

* u8
* u16
* u32
* u64
* u128
* usize

#### Floats and misc.

* f32
* f64
* bool
* char

#### Heap allocated

* Vec
* HashMap

#### Pointers and references

* &T


## Future plans

- [] Creation of a derive macro
- [] Further implementations of the `core` and `std` objects


## License

Mozilla Public License Version 2.0
