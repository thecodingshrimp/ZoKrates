// Isolate branches means making sure that any branch is enclosed in a block.
// This is important, because we want any statement resulting from inlining any branch to be isolated from the coller, so that its panics can be conditional to the branch being logically run

// `if c then a else b fi` becomes `if c then { a } else { b } fi`, and down the line any statements resulting from trating `a` and `b` can be safely kept inside the respective blocks.

use crate::typed_absy::folder::*;
use crate::typed_absy::*;
use zokrates_field::Field;

pub struct Isolator;

impl Isolator {
    #[zokrates_macro::stopwatch]
    pub fn isolate<T: Field>(p: TypedProgram<T>) -> TypedProgram<T> {
        let mut isolator = Isolator;
        isolator.fold_program(p)
    }
}

impl<'ast, T: Field> Folder<'ast, T> for Isolator {
    fn fold_if_else_expression<
        E: Expr<'ast, T> + Block<'ast, T> + Fold<'ast, T> + IfElse<'ast, T>,
    >(
        &mut self,
        _: &E::Ty,
        e: IfElseExpression<'ast, T, E>,
    ) -> IfElseOrExpression<'ast, T, E> {
        IfElseOrExpression::IfElse(IfElseExpression::new(
            self.fold_boolean_expression(*e.condition),
            E::block(vec![], e.consequence.fold(self)),
            E::block(vec![], e.alternative.fold(self)),
        ))
    }
}
