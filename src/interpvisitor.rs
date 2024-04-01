use crate::antlr::loxparser::PrintStmtContextExt;

use crate::{
    antlr::{
        self,
        loxparser::{LoxParserContextType, PrintStmtContext},
        loxvisitor::LoxVisitorCompat,
    },
    trace,
};
// use antlr::loxparser::{
//     Assignment_altContext, Bool_falseContext, Bool_trueContext, ComparisonContext, EqualityContext,
//     ExpressionContext, GroupContext, Logic_andContext, Logic_orContext, NilContext, NumberContext,
//     StrvalContext, Unary_altContext,
// };
use antlr_rust::tree::{ErrorNode, ParseTree, ParseTreeVisitorCompat, Tree};

use std::unreachable;
use std::{collections::HashMap, rc::Rc};
pub struct InterpVisit {
    val: TermValue,
}

impl InterpVisit {
    pub fn new() -> Self {
        Self {
            val: TermValue::Empty,
        }
    }
    pub fn value(&self) -> &TermValue {
        &self.val
    }
}
impl ParseTreeVisitorCompat<'_> for InterpVisit {
    type Node = LoxParserContextType;
    type Return = TermValue;
    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.val
    }

    fn aggregate_results(&self, _aggregate: Self::Return, next: Self::Return) -> Self::Return {
        //  unreachable!();
        next
    }
    fn visit_error_node(&mut self, node: &ErrorNode<'_, Self::Node>) -> Self::Return {
        println!("visit_error_node");
        TermValue::Error(node.get_text())
    }
}
#[derive(Debug, Default, Clone)]
pub enum TermValue {
    Number(f64),
    True,
    False,
    Nil,
    StringValue(String),
    #[default]
    Empty,
    Error(String),
    Function(usize),
}
impl PartialEq for TermValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a == b,
            (Self::True, Self::True) => true,
            (Self::False, Self::False) => true,
            (Self::Nil, Self::Nil) => true,
            (Self::StringValue(a), Self::StringValue(b)) => a == b,
            (Self::Empty, Self::Empty) => true,
            (Self::Error(a), Self::Error(b)) => a == b,
            (Self::Function(_), Self::Function(_)) => true,
            _ => false,
        }
    }
}
impl<'a> LoxVisitorCompat<'a> for InterpVisit {
    fn visit_program(&mut self, ctx: &antlr::loxparser::ProgramContext<'a>) -> Self::Return {
        trace!("visit_program");

        let mut result = Self::Return::default();
        for node in ctx.get_children() {
            result = self.visit(node.as_ref());

            if let TermValue::Error(_) = result {
                return result;
            }
        }
        return result;
    }
    fn visit_printStmt(&mut self, ctx: &PrintStmtContext<'a>) -> Self::Return {
        trace!("visit_printStmt {:?}", ctx.get_text());
        //  let x = ctx.get_child(0);
        let res = self.visit(&*ctx.exp.as_ref().unwrap().as_ref());
        match &res {
            TermValue::Error(_) => {
                return res;
            }
            TermValue::Nil => {
                println!("nil");
            }
            TermValue::True => {
                println!("true");
            }
            TermValue::False => {
                println!("false");
            }
            TermValue::Number(x) => {
                println!("{}", x);
            }
            TermValue::StringValue(x) => {
                println!("{}", x);
            }
            _ => {
                println!("unknown");
            }
        }
        res
    }
    fn visit_numval(&mut self, ctx: &antlr::loxparser::NumvalContext<'a>) -> Self::Return {
        trace!("visit_numval {:?}", ctx.get_text());
        let x = ctx.get_text().parse::<f64>().unwrap();
        TermValue::Number(x)
    }
    fn visit_strval(&mut self, ctx: &antlr::loxparser::StrvalContext<'a>) -> Self::Return {
        trace!("visit_strval {:?}", ctx.get_text());
        let x = ctx.get_text();
        TermValue::StringValue(x)
    }
}
