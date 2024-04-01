use std::path::PathBuf;
use std::{env, fs};

use antlr_rust::tree::Visitable;

use antlr_rust::{common_token_stream::CommonTokenStream, InputStream};
use trace::initlog;

use crate::errorvisitor::{ErrDetectVisit, ErrVal};
use crate::{
    antlr::{loxlexer::LoxLexer, loxparser::LoxParser},
    interpvisitor::InterpVisit,
};
mod antlr {
    pub mod loxlexer;
    pub mod loxlistener;
    pub mod loxparser;
    pub mod loxvisitor;
}
mod errorvisitor;
mod interpvisitor;
mod trace;

fn main() {
    initlog();

    // read file
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(PathBuf::from(args[1].clone()))
        .expect("Should have been able to read the file");
    // lex and parse it
    let lexer = LoxLexer::new(InputStream::new(contents.as_str()));
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = LoxParser::new(token_source);

    let root = parser.program();
    match root {
        Ok(root) => {
            // run error detect visitor
            let mut ed_visitor = ErrDetectVisit::new();
            root.accept(&mut ed_visitor);
            if ed_visitor.val != ErrVal::Empty {
                println!("Error: {:?}", ed_visitor.val);
                return;
            }
            // run interpreter visitor
            let mut visitor = InterpVisit::new();
            root.accept(&mut visitor);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
