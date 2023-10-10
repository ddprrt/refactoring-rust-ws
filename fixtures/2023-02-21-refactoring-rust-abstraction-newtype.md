---
title: "Refactoring in Rust: Abstraction with the Newtype Pattern"
categories:
- Rust
- Practical Rust
---

The following piece of code takes a `PathBuf` and extracts the file name, eventually converting it to an _owned_ `String`.

```rust
let file_name = function_path // => &PathBuf
    .file_name() // => Option<&OsStr>
    .and_then(|os_str| os_str.to_str()) // => Option<&str>
    .unwrap() // => &str
    .to_string(); // => String
```

There's a lot going on. We need to convert over several types, from `&PathBuf` over `&OsStr` to `&str` to eventually `String`. It's also an operation that might not contain a value, as paths might be directories, not necessarily files. Calling `unwrap` puts a lot of trust into this piece of code; things might go wrong!

That's a big setup to get to the desired outcome: A `String`.

The same piece of code exists in a few other places of your application, which can be problematic:

- What if we need to do more changes, e.g. stripping away file name endings? We need to be aware of all occurrences.
- What if `unwrap()` works in certain occasions but suddenly doesn't in others? Nothing tells us that this piece of code won't panic.
- What if have multiple file names we want to extract? How do we name our variables and bindings?
- What if we don't want to have `String` as output, but something different?
- What if ownership changes and the file name should live as long as `function_path`?
- What if the code needs to diverge in one scenario but not the other? How do you then keep track of your changes?

So many possible things that could change, and the fact that we use this piece of code multiple times in our code bases increases the chance of friction.

This calls for an abstraction!

Let's put aside the process of getting the desired result and think about the purpose of the owned `String`. We want to have the name of a file. There are a lot of semantics attached to this concept that goes beyond what a simple `String` has to offer. And even if we don't actually offer access to those semantics, we can still offer an abstraction in form of a type that tells us exactly what we're dealing with.

We introduce the `FileName` struct, a tuple struct containing an owned `String`.

```rust
struct FileName(String);
```

`FileName` has no constructor, nor does it have any direct `impl` blocks. Its sole purpose is to carry `String` for us. However, we can create a `FileName` out of an `&PathBuf`, using the `From` trait.

```rust
impl From<&PathBuf> for FileName {
    fn from(function_path: &PathBuf) -> Self {
        let path = function_path
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap()
            .to_string();
        Self(path)
    }
}
```

This implementation contains the actual conversion from `&PathBuf` to a `String`, however, those details are hidden from us. We don't care about how we get to a `FileName`, the only thing important for us is that we have a `FileName` in the end.

I also want to get rid of `unwrap()`. This is too brittle and can cause panic if somebody uses our new type in the wrong way. We have a couple of options.

Maybe our program doesn't actually care about existing file names but instead can work with defaults. We can describe this by implementing the `Default` trait for `FileName`, and defaulting to it if `&PathBuf` doesn't contain a file name for us.

```rust
impl Default for FileName {
    fn default() -> Self {
        Self("script.py".to_string())
    }
}
impl From<&PathBuf> for FileName {
    fn from(function_path: &PathBuf) -> Self {
        let path = function_path
            .file_name()
            .and_then(|os_str| os_str.to_str());
        match path {
            Some(path) => Self(path.to_string()),
            None => Self::default()
        }
    }
}
```

If a real file on disk is absolutely necessary, we should use `TryFrom` instead of `From` and prepare for errors.

```rust
// An error if PathBuf has no file name for us
#[derive(Debug)]
struct FileNameError;

impl std::error::Error for FileNameError {}

impl std::fmt::Display for FileNameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: No file name found")
    }
}

// Cautious way to parse file names
impl TryFrom<&PathBuf> for FileName {
    type Error = FileNameError;

    fn try_from(function_path: &PathBuf) -> Result<Self, Self::Error> {
         let path = function_path
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .ok_or(FileNameError {})?
            .to_string();
        Ok(Self(path))
    }
}
```

In both cases, the API speaks volumes to us. In the first case, we know that a `FileName` can always be created from a `&PathBuf` no matter what, in the other case we see from the interface that stuff might break. It's not only an `Option`, but an actual `Result` with an `Error`, that points at us saying _"You better deal with this, this is important!"_, and _"You wouldn't unwrap a `Result`, would you?"_. The nice thing is that we defer the decision of what to do with the `Result` to the point where we are actually converting. This can mean different things in different scenarios.

```rust
// Scenario 1: Defaults are OK for us
FileName::try_from(function_path).unwrap_or_default();

// Scenario 2: Bubble up the error and stop control flow
FileName::try_from(function_path)?;
```

Now that we converted `&PathBuf` to `FileName`, we also need some way to get to the `String`, which will be eventually used.

```rust
impl From<FileName> for String {
    fn from(name: FileName) -> Self {
        name.0
    }
}
```

The `From` trait indicates a conversion, which means that afterward `FileName` ceases to exist. That might be desired behavior, but since you're eventually working with `String`, you could also think of implementing `ToString` to indicate that you are always getting a `String` representation, keeping the original struct intact.

```rust
impl ToString for FileName {
    fn to_string(&self) -> String {
        self.0.to_owned()
    }
}
```

Both implementations tell us a lot about their purpose. We can decide which one is the best for our scenario.

And that's the final type. This _newtype_ doesn't contain any methods -- yet! -- but already tells us a lot about its purpose and its intent:

- It can be created from `&PathBuf`
- It owns its contents.
- It can be converted to a `String`.
- If you implemented `From`, you know that this conversion will never fail.
- If you implemented `TryFrom`, you know that this is an error you need to handle.
- It has a default value.

Also, we get a lot of flexibility:

- Creating a new file name is as easy as calling `FileName::from(...)`. But we stay flexible about what we want to do with the results.
- If the contents or even the type of a `FileName` should change, we have one spot where we can work on that change.
- We can name things better: `let script_name = FileName::from(...)` or `let file_id = FileName::from(...)`.
- If we want `FileName` to contain a reference and not an owned value, all the change that's necessary to happen, happens within the bounds of our abstraction: Changing the wrapped type, defining lifetime parameters, the conversion to a string, etc.
- We find all occurrences of `FileName` by looking for the type.

And when you create a new `FileName`, you use fewer lines of code. Which at least helps my attention a lot.

Those are the strengths of the _newtype pattern_: You create your name and slap a bunch of traits on it to explicitly and intentionally tell what the purpose of this struct is. The API speaks for itself, and it becomes much clearer what we can expect.
