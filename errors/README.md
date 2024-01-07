# Error Handling in Rust

> Techniques available in Rust when a program encounters an undesired situation

**Abstract**

- Outline the objective of the article: to provide a comprehensive guide to error handling in Rust.

## Error types

What is an error?

In programming, there are compile-time, runtime and logical errors.
We are only interested in runtime errors.
Runtime errors can be split into (borrowing from the Rust book):

- Recoverable: Unexpected situation that can be corrected with encapsulating code (failed API request, unretrievable file or `JSON` parsed response that does not contain a certain key). With further treatment, we are still able to run the program.
- Unrecoverable: When running the program is no longer viable and the program must be stopped (failing to make a database connection).

## Common Error Treatment

Different languages treat errors in different ways.
Javascript has [`try/catch`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Control_flow_and_error_handling#exception_handling_statements) blocks, Python has [`exceptions`](https://docs.python.org/3/tutorial/errors.html), C language has the [billion-dollar mistake](https://en.wikipedia.org/wiki/Tony_Hoare) (`*NULL` pointer) and in Go (Golang) an error is an [interface](https://go.dev/blog/error-handling-and-go) that defines a method `Error()` that returns a `string`.
Rust has `Result<T, E>`, `Option<T>`, `panic!()` and the `?` operator. This is what I pretend to explore in the following lines.

## Error in Rust

I think Rust is amazing at handling errors.
It prevents them in a way no other programming language does and it has great language constructs to handle errors while maintaining code readability.

### Preventing Errors

Before speaking about runtime errors we need to consider that Rust is a compiled language, strongly typed with a ruthless borrow checker and a unique ownership system.
Each one of those features adds a layer of protection avoiding errors being present during runtime in the first place.
I may cover those topics in a further article.

### Explicit vs Implicit Error Handling Paradigm

There are two paradigms when dealing with errors:

- Explicit: The programmer is required to write code to check for and handle errors when they appear. Exemple in [serde](https://github.com/serde-rs/serde/blob/d2d977a6c6dcff237ae956336d18b0c900c61aad/serde/build.rs#L11).

```rust
// rustc_minor_version() -> Option<u32>
// The function can return aa valid `unsigned` integer of `32` bits or `None`

// When this function is called the caller must verify right away if the return value is valid or not.
// `match` is a mechanism that Rust heavily relies on to check for all the possible values
let minor = match rustc_minor_version() {
    Some(minor) => minor,
    None => return,
};
```

- Implicit: The programmer relies on language features or conventions that automatically handle errors, like `throw` or `exceptions`. Example of [expressjs](https://expressjs.com/en/guide/error-handling.html).

```javascript
// Documentation: the example uses a try...catch block to catch errors in the asynchronous code and pass them to Express. If the try...catch block were omitted, Express would not catch the error since it is not part of the synchronous handler code.
app.get("/", (req, res, next) => {
  setTimeout(() => {
    try {
      throw new Error("BROKEN");
    } catch (err) {
      next(err);
    }
  }, 100);
});
```

And as you guessed, Rust handles errors explicitly.

Let's talk about the different methods, starting from the most Unrecoverable.

### Handling Errors

#### `panic!()`

> Unrecoverable error, halts all threads and the program returning its message to `stderr`

```rust
// Examples from: https://doc.rust-lang.org/std/macro.panic.html#examples
panic!();
panic!("this is a terrible mistake!");
panic!("this is a {} {message}", "fancy", message = "message");
std::panic::panic_any(4); // panic with the value of 4 to be collected elsewhere
```

`panic!("formatted string")` is a [macro](https://doc.rust-lang.org/std/macro.panic.html) that is (mainly) used for assertions.
Since edition `2021` and later, it [requires](https://doc.rust-lang.org/std/macro.panic.html#2021-and-later) a [formatted string](https://doc.rust-lang.org/rust-by-example/hello/print/fmt.html).
Upon calling `panic!` the program immediately stops all threads returning `101`. It is also invoked by the `unwrap()` methods on [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap) and [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap) with their respective `None` and `Err` variants.

There are two constructs in the language to report/propagate errors: `panic!` and `Result` (which we'll dive into next).
They both are used to capture the reason, but they are used in different cases:

- `panic!`: indicates a bug in the code, something unexpected, and halts the program
- `Result`: (in an unsuccessful case) reports a failure (like a returned `404` from an external API call) and communicates back to the caller the cause of the error

Some of the use cases:

- **Assert conditions in tests**: `panic!` (`unwrap` or `expect`) is what is used to signal failed tests
- **Example code**: it is possible to [add code in the documentation](https://practice.rs/comments-docs.html), which is run with tests
- **Prototyping**: using `unwrap` or `expect` than latter replacing them by a more robust error handling
- **Result will never fail**: the "Rust book" gives an excellent [example](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#cases-in-which-you-have-more-information-than-the-compiler).

```rust
// .parse() returns a Result.
// We can safelly call unwrap because we are absolutelly sure that
// the IP address 127.0.0.1 will always exist
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1"
  .parse()
  .expect("Hardcoded IP address should be valid");
```

- **Argument parsing**: when calling your code (and possibly setting up the execution's configuration) with bad input

> [!Node]
> I see calling `panic!` as: **fail fast**, there is no reason to try and recover.
> In every other case `Result` should be prefered

## Conclusion

<!-- Summarize key takeaways and encourage readers to embrace Rust's error-handling features for writing safe and reliable code. Invite them to explore additional resources for a deeper understanding of advanced error-handling techniques in Rust. -->
Just to give an idea, here are how many times each term appears in some libraries on `github.com`.

|   Library   | `panic!` | `Option<` | `Result<` | `?;` |
| :---------: | :------: | :-------: | :-------: | :--: |
| [starship]  |    0     |    94     |    84     |  87  |
| [actix-web] |    26    |    152    |    167    |  85  |
| [rust-lang] |   1.9k   |   4.9k    |   1.6k    | 1.4k |
|   [serde]   |    6     |    31     |    26     |  10  |

<!-- REFERENCES -->

[actix-web]: https://github.com/actix/actix-web
[rust-lang]: https://github.com/rust-lang/rust
[serde]: https://github.com/serde-rs/serde
[starship]: https://github.com/starship/starship
