# Brace Yourself Rust Is Coming - Announcing iceoryx2

Over a year ago, I embarked on a journey to learn Rust—a language heralded for its
emphasis on memory safety, concurrency, and reliability. The catalyst for this
exploration was the growing industry consensus favoring memory-safe languages, as
highlighted by the NSA's endorsement of
[Software Memory Safety](https://media.defense.gov/2022/Nov/10/2003112742/-1/-1/0/CSI_SOFTWARE_MEMORY_SAFETY.PDF)
and major companies such as Amazon advocating for
[Sustainability with Rust](https://aws.amazon.com/blogs/opensource/sustainability-with-rust/).

In the ever-evolving landscape where Microsoft embraces Rust for
[writing drivers](https://www.golem.de/news/entwicklung-microsoft-legt-rust-framework-fuer-windows-treiber-offen-2309-177932.html)
and the industry witnesses a paradigm shift away from C/C++ to Rust, it is worth
noting that Rust's commitment to safety is not merely a claim; its safety features can
be [rigorously proven](https://research.ralfj.de/phd/thesis-screen.pdf).
This substantiates Rust as a reliable and secure choice and raises the
question: Why does the automotive domain lag behind?

Enter iceoryx2—the fruition of my efforts to reimagine iceoryx from the ground up,
exclusively in Rust. This venture aimed not only to comprehend the language but also
to evaluate whether Rust truly lived up to its promises. The resounding answer is a
definitive "yes," and more.

My day job involves certifying iceoryx written in C++ for use in safety-critical
ASIL-D environments. While grappling with algorithmic challenges, the burden of C++
introduces a myriad of additional complications, consuming considerable time and
causing frustration.

* **Documentation:** Managing doxygen code examples in harmony with the codebase.
* **Lifetimes:** Navigating lifetime dependencies when working with RAII and factories.
* **Concurrency:** Preventing inadvertent multi-threaded usage and certifying
    concurrent and lock-free code.
* **Build-System:** Handling dependencies and third-party packages seamlessly.
* **Templates:** Certifying generic C++ code for various but valid types.
* **Static Code Analysis:** Choosing the right tools and adhering to Misra,
    Autosar, C++ Core Guidelines, or a combination.

Why dwell in the abstract when we can scrutinize a real-world example, comparing
iceoryx (C++) to iceoryx2 (Rust)? Brace yourself for a journey into a future where
Rust takes the lead in inter-process zero-copy communication within our specialized
domain. Iceoryx2 is not just an upgrade—it's a leap forward into a safer, more
efficient era.

## Example: Receiving Data

In both iterations of iceoryx, the communication revolves around ports as endpoints. In this context, the `Publisher` assumes the role of the sender in a publish-subscribe messaging pattern. When users intend to transmit data, they typically follow a set of common steps across both versions.

1. Invoke `Publisher::loan()` to obtain a `Sample` for storing the data to be sent.
2. Utilize `Publisher::send(sample)` to dispatch the data to all receiving endpoints (`Subscriber`).

Represented here is a simplified interface for this `Publisher`:

```cpp
// C++
class Publisher {
  public:
    Sample loan();
    void send(Sample &&sample);
};
```

```rust
// Rust
struct Publisher {}

impl Publisher {
    fn loan(&'publisher self) -> Sample<'publisher> {}
    fn send(&self, sample: Sample) {}
}
```

## Documentation Best Practices: C++ vs. Rust

When it comes to documenting code and ensuring the accuracy of code examples, both C++
and Rust offer distinct approaches. Let's delve into the practices and tools used in
each language:

### C++

In the C++ realm, internal documentation is commonly managed using Doxygen, a tool
providing `@code`/`@endcode` tags for embedding code examples. However, there's a
crucial limitation—Doxygen does not verify whether the code is running or compiling.
This introduces a potential risk, as refactoring in one part of the codebase may
inadvertently break examples located elsewhere.

```cpp
/// @code
///   std::cout << "hello world, without include iostream";
/// @endcode
class Publisher {};
```

While one might consider a workaround by compiling examples during the build process,
this proves cumbersome, especially when aiming for examples for every method.

### Rust

In the Rust ecosystem, documentation is ingrained in the language specification,
adopting markdown syntax. Code examples are not only part of the documentation but are
actively built and tested through Rust's build system, `cargo`.

```rust
/// # Examples
/// ```
/// println!("hello world");
/// ```
struct Publisher {}
```

What sets Rust apart is the execution of documentation examples during testing
(`cargo test --doc`). This means that not only is the code compiled, but its
functionality is verified. This proactive approach ensures that any internal changes
or refactoring issues are promptly exposed.

Moreover, Rust allows for the incorporation of contracts directly into code examples
using the `assert!` macro, adding an extra layer of visibility and validation.

```rust
/// ```
/// // the doc test fails if the assertion does not hold
/// assert!(2 + 2 == 4);
/// ```
struct Publisher {}
```

In summary, Rust's documentation practices, combined with built-in testing, provide
out-of-the-box robust and reliable means of ensuring code example correctness compared
to C++'s Doxygen-based approach. This not only enhances the clarity of documentation
but also strengthens the overall integrity of the codebase.

## Lifetimes And Accidental Concurrency

The `Sample` that is returned by the `Publisher` represents a memory
resource that could be leaked when not handled correctly. So we can use in both
languages the
[RAII](https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization)
idiom. In essence, we define the class/struct as the owner of the resource and
as soon as the object goes out of scope the resource is released. In our case,
the memory is deallocated.

When a user calls `Publisher::loan()` and decides that they no longer want
to send the `Sample`, all they have to do is destroy the acquired `Sample`. The
destructor of the `Sample` ensures that the memory is safely returned to the
`Publisher`.

So no problem?

Let me ask you:

 1. What happens when the `Publisher` goes out of scope before the `Sample`?
 2. What happens when the `Sample` is moved into another thread and goes
    out of scope? Then we accidentally access the `Publisher` concurrently
    without even realizing it.

### C++

### Rust

## Concurrency

## TODO

* loan_uninit,
* one argument was no certified compiler, now its there - so whats your excuse
    to not use rust in a new project of a mission critical system
