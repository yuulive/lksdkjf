use core::fmt::Debug;

use crate::Result;

/// A trait that allows to interactively evaluate a function and pass its result to the given closure.
///
/// This trait gets implemented automatically when you use the [`Function`] attribute.
///
/// [`Function`]: ./attr.Function.html
pub trait Function {
    /// Returns the functions name.
    ///
    /// Can be used to drive auto-completion in a CLI.
    fn function_name(&self) -> &'static str;

    /// Parses the args string into the expected arguments of the method,
    /// executes the method and
    /// passes the result as a `Ok(&dyn Debug)` to the given closure.
    ///
    /// On error the an `Err(InteractiveError)` is passed to the closure instead.
    fn eval(&self, args: &str, f: &mut dyn FnMut(Result<'_, &dyn Debug>));
}

// Implement inventory::Collect for ´&dyn Function´
#[cfg(feature = "std")]
inventory::collect!(&'static dyn Function);