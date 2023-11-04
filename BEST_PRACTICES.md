# Best Practices

### When To Use What Log Level

* **information dedicated to the developer of the application**
  * `TRACE` - interesting application events, e.g. whenever a resource
        is created/destroyed
  * `DEBUG` - only when the function returns a result that contains an error
* **information dedicated to the user of the application**
  * `INFO` - some interesting stuff for the user
  * `WARN` - warnings dedicated to the user of the application, the
     functionality is not restricted but some kind of internal recoverable
     misbehavior occurred
  * `ERROR` - some severe failure occurred that is still handled with a result
    containing an error but the application can continue if the user can
    recover from it
  * `FATAL` - the application panics after the log output

### Error Handling

* Never return `Err(...)`, always use `fail!` macro.
  * Elkodon shall always log a message to `DEBUG` whenever an `Err(...)` is.

  ```rust
  // bad
  fn do_stuff() -> Result<u64, u64> {
    Err(123)
  }

  // good
  use elkodon_bb_log::fail;
  fn do_stuff() -> Result<u64, u64> {
    fail!(from "do_stuff", with 123, "Failed to do stuff!");
  }
  ```

### Fatal Error Handling

* Never call `panic!(...)` directly, always use the `fatal_panic!` macro.
  * Elkodon shall always log a message to `FATAL` whenever a panic occurs.

  ```rust
  // bad
  panic!("whatever");

  // good
  fatal_panic!(from "some origin", "whatever");
  ```

### Re-Exports And Preludes

* The most common functionality of a construct shall be fully usable by only
    using:

    ```rust
    use my::construct::*;
    ```

  * Use `pub use ...` to re-export requirements.
  * For more complex constructs use preludes,
        <https://doc.rust-lang.org/beta/reference/names/preludes.html>

### Documentation And Examples

* Use `?` operator in documentation examples to reduce error handling clutter.
* Use `#` to hide boilerplate code in documentation

### Self-Referencing Structs

* Use the `ouroboros` crate for self-referencing structs, required for non-movable types like
    `Mutex` & `MutexHandle`, `Barrier` & `BarrierHandle`, `UnnamedSemaphore` & `SemaphoreHandle`
