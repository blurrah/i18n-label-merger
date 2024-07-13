use std::sync::Arc;

use swc_core::{
    common::SourceMap,
    ecma::{
        ast::{CallExpr, Callee, Expr},
        transforms::testing::test,
        utils::{ident, swc_common},
        visit::{noop_visit_mut_type, noop_visit_type, swc_ecma_ast, Visit},
    },
};

pub struct TranslationExtractor {
    namespace: Option<String>,
    usages: Vec<String>,
}

// Constants used for identifying translator caller
const USE_TRANSLATIONS: &str = "useTranslations";
const GET_TRANSLATIONS: &str = "getTranslations";

impl Visit for TranslationExtractor {
    noop_visit_type!();

    fn visit_call_expr(&mut self, call_expr: &CallExpr) {
        if let Callee::Expr(boxed_expr) = &call_expr.callee {
            if let Expr::Ident(ident) = &**boxed_expr {
                if ident.sym == GET_TRANSLATIONS {
                    self.usages.push(ident.sym.to_string());
                }
                if ident.
            }

            swc_core::ecma::visit::visit_call_expr(self, call_expr);
        }
    }
}

// fn get_translation_data(code: &str) -> &str {
//     let mut visitor = TranslationExtractor {
//         namespace: None,
//         usages: Vec::new(),
//     };

//     // Create a "file" from the input string
//     let cm: SourceMap = Default::default();
//     let fm = cm.new_source_file(swc_common::FileName::Custom("".into()), code.to_string());

//     let mut parser = Parser::new(Syntax::Es(Default::default()), (&*fm).clone(), None);

//     visitor.visit_module(&module);

//     visitor.namespace
// }

#[cfg(test)]
mod tests {}
