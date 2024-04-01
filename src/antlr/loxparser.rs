// Generated from ../Lox.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::loxlistener::*;
use super::loxvisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const T__0:isize=1; 
		pub const T__1:isize=2; 
		pub const WS:isize=3; 
		pub const NUMBER:isize=4; 
		pub const STRING:isize=5; 
		pub const DIGIT:isize=6;
	pub const RULE_program:usize = 0; 
	pub const RULE_statement:usize = 1; 
	pub const RULE_printStmt:usize = 2; 
	pub const RULE_term:usize = 3;
	pub const ruleNames: [&'static str; 4] =  [
		"program", "statement", "printStmt", "term"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;3] = [
		None, Some("'print'"), Some("';'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;7]  = [
		None, None, None, Some("WS"), Some("NUMBER"), Some("STRING"), Some("DIGIT")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,LoxParserExt<'input>, I, LoxParserContextType , dyn LoxListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type LoxTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, LoxParserContextType , dyn LoxListener<'input> + 'a>;

/// Parser for Lox grammar
pub struct LoxParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> LoxParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				LoxParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> LoxParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> LoxParser<'input, I, DefaultErrorStrategy<'input,LoxParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for LoxParser
pub trait LoxParserContext<'input>:
	for<'x> Listenable<dyn LoxListener<'input> + 'x > + 
	for<'x> Visitable<dyn LoxVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=LoxParserContextType>
{}

antlr_rust::coerce_from!{ 'input : LoxParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn LoxParserContext<'input> + 'input
where
    T: LoxVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn LoxVisitor<'input> + 'x))
    }
}

impl<'input> LoxParserContext<'input> for TerminalNode<'input,LoxParserContextType> {}
impl<'input> LoxParserContext<'input> for ErrorNode<'input,LoxParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn LoxParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn LoxListener<'input> + 'input }

pub struct LoxParserContextType;
antlr_rust::tid!{LoxParserContextType}

impl<'input> ParserNodeType<'input> for LoxParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn LoxParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for LoxParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for LoxParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct LoxParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> LoxParserExt<'input>{
}
antlr_rust::tid! { LoxParserExt<'a> }

impl<'input> TokenAware<'input> for LoxParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for LoxParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for LoxParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "Lox.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- program ----------------
pub type ProgramContextAll<'input> = ProgramContext<'input>;


pub type ProgramContext<'input> = BaseParserRuleContext<'input,ProgramContextExt<'input>>;

#[derive(Clone)]
pub struct ProgramContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LoxParserContext<'input> for ProgramContext<'input>{}

impl<'input,'a> Listenable<dyn LoxListener<'input> + 'a> for ProgramContext<'input>{
		fn enter(&self,listener: &mut (dyn LoxListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_program(self);
		}
		fn exit(&self,listener: &mut (dyn LoxListener<'input> + 'a)) {
			listener.exit_program(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LoxVisitor<'input> + 'a> for ProgramContext<'input>{
	fn accept(&self,visitor: &mut (dyn LoxVisitor<'input> + 'a)) {
		visitor.visit_program(self);
	}
}

impl<'input> CustomRuleContext<'input> for ProgramContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LoxParserContextType;
	fn get_rule_index(&self) -> usize { RULE_program }
	//fn type_rule_index() -> usize where Self: Sized { RULE_program }
}
antlr_rust::tid!{ProgramContextExt<'a>}

impl<'input> ProgramContextExt<'input>{
	fn new(parent: Option<Rc<dyn LoxParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProgramContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProgramContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ProgramContextAttrs<'input>: LoxParserContext<'input> + BorrowMut<ProgramContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,LoxParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ProgramContextAttrs<'input> for ProgramContext<'input>{}

impl<'input, I, H> LoxParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn program(&mut self,)
	-> Result<Rc<ProgramContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProgramContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_program);
        let mut _localctx: Rc<ProgramContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(11);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__0 {
				{
				{
				/*InvokeRule statement*/
				recog.base.set_state(8);
				recog.statement()?;

				}
				}
				recog.base.set_state(13);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(14);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- statement ----------------
pub type StatementContextAll<'input> = StatementContext<'input>;


pub type StatementContext<'input> = BaseParserRuleContext<'input,StatementContextExt<'input>>;

#[derive(Clone)]
pub struct StatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LoxParserContext<'input> for StatementContext<'input>{}

impl<'input,'a> Listenable<dyn LoxListener<'input> + 'a> for StatementContext<'input>{
		fn enter(&self,listener: &mut (dyn LoxListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_statement(self);
		}
		fn exit(&self,listener: &mut (dyn LoxListener<'input> + 'a)) {
			listener.exit_statement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LoxVisitor<'input> + 'a> for StatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn LoxVisitor<'input> + 'a)) {
		visitor.visit_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LoxParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr_rust::tid!{StatementContextExt<'a>}

impl<'input> StatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn LoxParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StatementContextAttrs<'input>: LoxParserContext<'input> + BorrowMut<StatementContextExt<'input>>{

fn printStmt(&self) -> Option<Rc<PrintStmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input>{}

impl<'input, I, H> LoxParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn statement(&mut self,)
	-> Result<Rc<StatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule printStmt*/
			recog.base.set_state(16);
			recog.printStmt()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- printStmt ----------------
pub type PrintStmtContextAll<'input> = PrintStmtContext<'input>;


pub type PrintStmtContext<'input> = BaseParserRuleContext<'input,PrintStmtContextExt<'input>>;

#[derive(Clone)]
pub struct PrintStmtContextExt<'input>{
	pub exp: Option<Rc<TermContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> LoxParserContext<'input> for PrintStmtContext<'input>{}

impl<'input,'a> Listenable<dyn LoxListener<'input> + 'a> for PrintStmtContext<'input>{
		fn enter(&self,listener: &mut (dyn LoxListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_printStmt(self);
		}
		fn exit(&self,listener: &mut (dyn LoxListener<'input> + 'a)) {
			listener.exit_printStmt(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LoxVisitor<'input> + 'a> for PrintStmtContext<'input>{
	fn accept(&self,visitor: &mut (dyn LoxVisitor<'input> + 'a)) {
		visitor.visit_printStmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for PrintStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LoxParserContextType;
	fn get_rule_index(&self) -> usize { RULE_printStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_printStmt }
}
antlr_rust::tid!{PrintStmtContextExt<'a>}

impl<'input> PrintStmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn LoxParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PrintStmtContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrintStmtContextExt{
				exp: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait PrintStmtContextAttrs<'input>: LoxParserContext<'input> + BorrowMut<PrintStmtContextExt<'input>>{

fn term(&self) -> Option<Rc<TermContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PrintStmtContextAttrs<'input> for PrintStmtContext<'input>{}

impl<'input, I, H> LoxParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn printStmt(&mut self,)
	-> Result<Rc<PrintStmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrintStmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_printStmt);
        let mut _localctx: Rc<PrintStmtContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(18);
			recog.base.match_token(T__0,&mut recog.err_handler)?;

			/*InvokeRule term*/
			recog.base.set_state(19);
			let tmp = recog.term()?;
			 cast_mut::<_,PrintStmtContext >(&mut _localctx).exp = Some(tmp.clone());
			  

			recog.base.set_state(20);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- term ----------------
#[derive(Debug)]
pub enum TermContextAll<'input>{
	NumvalContext(NumvalContext<'input>),
	StrvalContext(StrvalContext<'input>),
Error(TermContext<'input>)
}
antlr_rust::tid!{TermContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for TermContextAll<'input>{}

impl<'input> LoxParserContext<'input> for TermContextAll<'input>{}

impl<'input> Deref for TermContextAll<'input>{
	type Target = dyn TermContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use TermContextAll::*;
		match self{
			NumvalContext(inner) => inner,
			StrvalContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn LoxVisitor<'input> + 'a> for TermContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn LoxVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn LoxListener<'input> + 'a> for TermContextAll<'input>{
    fn enter(&self, listener: &mut (dyn LoxListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn LoxListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type TermContext<'input> = BaseParserRuleContext<'input,TermContextExt<'input>>;

#[derive(Clone)]
pub struct TermContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LoxParserContext<'input> for TermContext<'input>{}

impl<'input,'a> Listenable<dyn LoxListener<'input> + 'a> for TermContext<'input>{
}

impl<'input,'a> Visitable<dyn LoxVisitor<'input> + 'a> for TermContext<'input>{
}

impl<'input> CustomRuleContext<'input> for TermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LoxParserContextType;
	fn get_rule_index(&self) -> usize { RULE_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_term }
}
antlr_rust::tid!{TermContextExt<'a>}

impl<'input> TermContextExt<'input>{
	fn new(parent: Option<Rc<dyn LoxParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TermContextAll<'input>> {
		Rc::new(
		TermContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TermContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait TermContextAttrs<'input>: LoxParserContext<'input> + BorrowMut<TermContextExt<'input>>{


}

impl<'input> TermContextAttrs<'input> for TermContext<'input>{}

pub type NumvalContext<'input> = BaseParserRuleContext<'input,NumvalContextExt<'input>>;

pub trait NumvalContextAttrs<'input>: LoxParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token NUMBER
	/// Returns `None` if there is no child corresponding to token NUMBER
	fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,LoxParserContextType>>> where Self:Sized{
		self.get_token(NUMBER, 0)
	}
}

impl<'input> NumvalContextAttrs<'input> for NumvalContext<'input>{}

pub struct NumvalContextExt<'input>{
	base:TermContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NumvalContextExt<'a>}

impl<'input> LoxParserContext<'input> for NumvalContext<'input>{}

impl<'input,'a> Listenable<dyn LoxListener<'input> + 'a> for NumvalContext<'input>{
	fn enter(&self,listener: &mut (dyn LoxListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_numval(self);
	}
	fn exit(&self,listener: &mut (dyn LoxListener<'input> + 'a)) {
		listener.exit_numval(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LoxVisitor<'input> + 'a> for NumvalContext<'input>{
	fn accept(&self,visitor: &mut (dyn LoxVisitor<'input> + 'a)) {
		visitor.visit_numval(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumvalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LoxParserContextType;
	fn get_rule_index(&self) -> usize { RULE_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_term }
}

impl<'input> Borrow<TermContextExt<'input>> for NumvalContext<'input>{
	fn borrow(&self) -> &TermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TermContextExt<'input>> for NumvalContext<'input>{
	fn borrow_mut(&mut self) -> &mut TermContextExt<'input> { &mut self.base }
}

impl<'input> TermContextAttrs<'input> for NumvalContext<'input> {}

impl<'input> NumvalContextExt<'input>{
	fn new(ctx: &dyn TermContextAttrs<'input>) -> Rc<TermContextAll<'input>>  {
		Rc::new(
			TermContextAll::NumvalContext(
				BaseParserRuleContext::copy_from(ctx,NumvalContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StrvalContext<'input> = BaseParserRuleContext<'input,StrvalContextExt<'input>>;

pub trait StrvalContextAttrs<'input>: LoxParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token STRING
	/// Returns `None` if there is no child corresponding to token STRING
	fn STRING(&self) -> Option<Rc<TerminalNode<'input,LoxParserContextType>>> where Self:Sized{
		self.get_token(STRING, 0)
	}
}

impl<'input> StrvalContextAttrs<'input> for StrvalContext<'input>{}

pub struct StrvalContextExt<'input>{
	base:TermContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StrvalContextExt<'a>}

impl<'input> LoxParserContext<'input> for StrvalContext<'input>{}

impl<'input,'a> Listenable<dyn LoxListener<'input> + 'a> for StrvalContext<'input>{
	fn enter(&self,listener: &mut (dyn LoxListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_strval(self);
	}
	fn exit(&self,listener: &mut (dyn LoxListener<'input> + 'a)) {
		listener.exit_strval(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LoxVisitor<'input> + 'a> for StrvalContext<'input>{
	fn accept(&self,visitor: &mut (dyn LoxVisitor<'input> + 'a)) {
		visitor.visit_strval(self);
	}
}

impl<'input> CustomRuleContext<'input> for StrvalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LoxParserContextType;
	fn get_rule_index(&self) -> usize { RULE_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_term }
}

impl<'input> Borrow<TermContextExt<'input>> for StrvalContext<'input>{
	fn borrow(&self) -> &TermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TermContextExt<'input>> for StrvalContext<'input>{
	fn borrow_mut(&mut self) -> &mut TermContextExt<'input> { &mut self.base }
}

impl<'input> TermContextAttrs<'input> for StrvalContext<'input> {}

impl<'input> StrvalContextExt<'input>{
	fn new(ctx: &dyn TermContextAttrs<'input>) -> Rc<TermContextAll<'input>>  {
		Rc::new(
			TermContextAll::StrvalContext(
				BaseParserRuleContext::copy_from(ctx,StrvalContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> LoxParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn term(&mut self,)
	-> Result<Rc<TermContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TermContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_term);
        let mut _localctx: Rc<TermContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(24);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 STRING 
				=> {
					let tmp = StrvalContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(22);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}

			 NUMBER 
				=> {
					let tmp = NumvalContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(23);
					recog.base.match_token(NUMBER,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x08\x1d\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x03\x02\x07\x02\x0c\x0a\x02\x0c\x02\x0e\x02\x0f\x0b\x02\x03\x02\x03\x02\
	\x03\x03\x03\x03\x03\x04\x03\x04\x03\x04\x03\x04\x03\x05\x03\x05\x05\x05\
	\x1b\x0a\x05\x03\x05\x02\x02\x06\x02\x04\x06\x08\x02\x02\x02\x1a\x02\x0d\
	\x03\x02\x02\x02\x04\x12\x03\x02\x02\x02\x06\x14\x03\x02\x02\x02\x08\x1a\
	\x03\x02\x02\x02\x0a\x0c\x05\x04\x03\x02\x0b\x0a\x03\x02\x02\x02\x0c\x0f\
	\x03\x02\x02\x02\x0d\x0b\x03\x02\x02\x02\x0d\x0e\x03\x02\x02\x02\x0e\x10\
	\x03\x02\x02\x02\x0f\x0d\x03\x02\x02\x02\x10\x11\x07\x02\x02\x03\x11\x03\
	\x03\x02\x02\x02\x12\x13\x05\x06\x04\x02\x13\x05\x03\x02\x02\x02\x14\x15\
	\x07\x03\x02\x02\x15\x16\x05\x08\x05\x02\x16\x17\x07\x04\x02\x02\x17\x07\
	\x03\x02\x02\x02\x18\x1b\x07\x07\x02\x02\x19\x1b\x07\x06\x02\x02\x1a\x18\
	\x03\x02\x02\x02\x1a\x19\x03\x02\x02\x02\x1b\x09\x03\x02\x02\x02\x04\x0d\
	\x1a";

