use crate::scripting_language::function::Function;
use crate::scripting_language::garbage_collection::collector::Collector as GarbageCollector;
use crate::scripting_language::garbage_collection::reference::Reference;
use crate::scripting_language::parser::Parser;
use crate::scripting_language::scanner::Scanner;

pub mod error;
use error::Error;

/// The `Interpreter` type.
///
/// This corresponds to the `compile()` function.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Display, Hash, PartialEq, Serialize)]
pub struct Interpreter {}

impl Interpreter {
  /// Compile the source.
  #[named]
  pub fn compile<'source>(
    &mut self,
    source: &'source str,
    garbage_collector: &mut GarbageCollector,
  ) -> Result<Reference<Function>, Error> {
    let scanner = Scanner::new(source);

    let parser = Parser::new(scanner, garbage_collector);

    let result = parser.compile()?;

    Ok(result)
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_interpreter() {
    init();

    let interpreter = Interpreter::default();
    print_var!(interpreter);
  }
}
