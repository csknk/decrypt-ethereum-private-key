# Parse JSON

Notes re main:

```rs

use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```
See: [ref][1].

> The Box<dyn Error> type is called a trait object, which we’ll talk about in the “Using Trait Objects that Allow for Values of Different Types” section in Chapter 17. For now, you can read Box<dyn Error> to mean “any kind of error.” Using ? in a main function with this return type is allowed.

Returning a Result Type from a Function
---------------------------------------
See [ref][2].

The return keyword can only be ommitted from the last `Err()/Ok()` statement?

```rs

fn function_with_error() -> Result<u64, String> {

    //if error happens
    return Err("The error message".to_string());

    // else, return valid output
    Ok(255)
}
```

References
----------
* [Rust book, recoverable errors][1]

[1]: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#the--operator-can-be-used-in-functions-that-return-result
[2]: https://learning-rust.github.io/docs/e3.option_and_result.html#Basic-usages-of-Result
