//! Implements builtin `random` module.

use rand::{thread_rng, Rng};

use error::Error;
use exec::ExecError;
use function::Arity::Exact;
use module::{Module, ModuleBuilder};
use scope::Scope;
use value::Value;

/// Loads the `random` module into the given scope.
pub fn load(scope: Scope) -> Module {
    ModuleBuilder::new("random", scope)
        .add_function("random",  fn_random,  Exact(0))
        .add_function("shuffle", fn_shuffle, Exact(1))
        .finish()
}

/// `random` returns a random float value in the range `[0.0, 1.0]`.
fn fn_random(_scope: &Scope, _args: &mut [Value]) -> Result<Value, Error> {
    Ok(thread_rng().next_f64().into())
}

/// `shuffle` shuffles the values of a list.
fn fn_shuffle(_scope: &Scope, args: &mut [Value]) -> Result<Value, Error> {
    let mut v = args[0].take();

    match v {
        Value::Unit => (),
        Value::List(ref mut li) => thread_rng().shuffle(li),
        ref v => return Err(From::from(ExecError::expected("list", v)))
    }

    Ok(v)
}
