// Generated from ../Lox.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use antlr_rust::atn::ATN;
use antlr_rust::char_stream::CharStream;
use antlr_rust::int_stream::IntStream;
use antlr_rust::lexer::{BaseLexer, Lexer, LexerRecog};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::lexer_atn_simulator::{LexerATNSimulator, ILexerATNSimulator};
use antlr_rust::PredictionContextCache;
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::TokenSource;
use antlr_rust::token_factory::{TokenFactory,CommonTokenFactory,TokenAware};
use antlr_rust::token::*;
use antlr_rust::rule_context::{BaseRuleContext,EmptyCustomRuleContext,EmptyContext};
use antlr_rust::parser_rule_context::{ParserRuleContext,BaseParserRuleContext,cast};
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};

use antlr_rust::{lazy_static,Tid,TidAble,TidExt};

use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};


	pub const T__0:isize=1; 
	pub const T__1:isize=2; 
	pub const WS:isize=3; 
	pub const NUMBER:isize=4; 
	pub const STRING:isize=5; 
	pub const DIGIT:isize=6;
	pub const channelNames: [&'static str;0+2] = [
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	];

	pub const modeNames: [&'static str;1] = [
		"DEFAULT_MODE"
	];

	pub const ruleNames: [&'static str;6] = [
		"T__0", "T__1", "WS", "NUMBER", "STRING", "DIGIT"
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


pub type LexerContext<'input> = BaseRuleContext<'input,EmptyCustomRuleContext<'input,LocalTokenFactory<'input> >>;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

type From<'a> = <LocalTokenFactory<'a> as TokenFactory<'a> >::From;

pub struct LoxLexer<'input, Input:CharStream<From<'input> >> {
	base: BaseLexer<'input,LoxLexerActions,Input,LocalTokenFactory<'input>>,
}

antlr_rust::tid! { impl<'input,Input> TidAble<'input> for LoxLexer<'input,Input> where Input:CharStream<From<'input> > }

impl<'input, Input:CharStream<From<'input> >> Deref for LoxLexer<'input,Input>{
	type Target = BaseLexer<'input,LoxLexerActions,Input,LocalTokenFactory<'input>>;

	fn deref(&self) -> &Self::Target {
		&self.base
	}
}

impl<'input, Input:CharStream<From<'input> >> DerefMut for LoxLexer<'input,Input>{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.base
	}
}


impl<'input, Input:CharStream<From<'input> >> LoxLexer<'input,Input>{
    fn get_rule_names(&self) -> &'static [&'static str] {
        &ruleNames
    }
    fn get_literal_names(&self) -> &[Option<&str>] {
        &_LITERAL_NAMES
    }

    fn get_symbolic_names(&self) -> &[Option<&str>] {
        &_SYMBOLIC_NAMES
    }

    fn get_grammar_file_name(&self) -> &'static str {
        "LoxLexer.g4"
    }

	pub fn new_with_token_factory(input: Input, tf: &'input LocalTokenFactory<'input>) -> Self {
		antlr_rust::recognizer::check_version("0","3");
    	Self {
			base: BaseLexer::new_base_lexer(
				input,
				LexerATNSimulator::new_lexer_atnsimulator(
					_ATN.clone(),
					_decision_to_DFA.clone(),
					_shared_context_cache.clone(),
				),
				LoxLexerActions{},
				tf
			)
	    }
	}
}

impl<'input, Input:CharStream<From<'input> >> LoxLexer<'input,Input> where &'input LocalTokenFactory<'input>:Default{
	pub fn new(input: Input) -> Self{
		LoxLexer::new_with_token_factory(input, <&LocalTokenFactory<'input> as Default>::default())
	}
}

pub struct LoxLexerActions {
}

impl LoxLexerActions{
}

impl<'input, Input:CharStream<From<'input> >> Actions<'input,BaseLexer<'input,LoxLexerActions,Input,LocalTokenFactory<'input>>> for LoxLexerActions{
	}

	impl<'input, Input:CharStream<From<'input> >> LoxLexer<'input,Input>{

}

impl<'input, Input:CharStream<From<'input> >> LexerRecog<'input,BaseLexer<'input,LoxLexerActions,Input,LocalTokenFactory<'input>>> for LoxLexerActions{
}
impl<'input> TokenAware<'input> for LoxLexerActions{
	type TF = LocalTokenFactory<'input>;
}

impl<'input, Input:CharStream<From<'input> >> TokenSource<'input> for LoxLexer<'input,Input>{
	type TF = LocalTokenFactory<'input>;

    fn next_token(&mut self) -> <Self::TF as TokenFactory<'input>>::Tok {
        self.base.next_token()
    }

    fn get_line(&self) -> isize {
        self.base.get_line()
    }

    fn get_char_position_in_line(&self) -> isize {
        self.base.get_char_position_in_line()
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn IntStream> {
        self.base.get_input_stream()
    }

	fn get_source_name(&self) -> String {
		self.base.get_source_name()
	}

    fn get_token_factory(&self) -> &'input Self::TF {
        self.base.get_token_factory()
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
		"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x02\
		\x08\x36\x08\x01\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\
		\x09\x05\x04\x06\x09\x06\x04\x07\x09\x07\x03\x02\x03\x02\x03\x02\x03\x02\
		\x03\x02\x03\x02\x03\x03\x03\x03\x03\x04\x06\x04\x19\x0a\x04\x0d\x04\x0e\
		\x04\x1a\x03\x04\x03\x04\x03\x05\x06\x05\x20\x0a\x05\x0d\x05\x0e\x05\x21\
		\x03\x05\x03\x05\x06\x05\x26\x0a\x05\x0d\x05\x0e\x05\x27\x05\x05\x2a\x0a\
		\x05\x03\x06\x03\x06\x07\x06\x2e\x0a\x06\x0c\x06\x0e\x06\x31\x0b\x06\x03\
		\x06\x03\x06\x03\x07\x03\x07\x03\x2f\x02\x08\x03\x03\x05\x04\x07\x05\x09\
		\x06\x0b\x07\x0d\x08\x03\x02\x03\x05\x02\x0b\x0c\x0f\x0f\x22\x22\x02\x3a\
		\x02\x03\x03\x02\x02\x02\x02\x05\x03\x02\x02\x02\x02\x07\x03\x02\x02\x02\
		\x02\x09\x03\x02\x02\x02\x02\x0b\x03\x02\x02\x02\x02\x0d\x03\x02\x02\x02\
		\x03\x0f\x03\x02\x02\x02\x05\x15\x03\x02\x02\x02\x07\x18\x03\x02\x02\x02\
		\x09\x1f\x03\x02\x02\x02\x0b\x2b\x03\x02\x02\x02\x0d\x34\x03\x02\x02\x02\
		\x0f\x10\x07\x72\x02\x02\x10\x11\x07\x74\x02\x02\x11\x12\x07\x6b\x02\x02\
		\x12\x13\x07\x70\x02\x02\x13\x14\x07\x76\x02\x02\x14\x04\x03\x02\x02\x02\
		\x15\x16\x07\x3d\x02\x02\x16\x06\x03\x02\x02\x02\x17\x19\x09\x02\x02\x02\
		\x18\x17\x03\x02\x02\x02\x19\x1a\x03\x02\x02\x02\x1a\x18\x03\x02\x02\x02\
		\x1a\x1b\x03\x02\x02\x02\x1b\x1c\x03\x02\x02\x02\x1c\x1d\x08\x04\x02\x02\
		\x1d\x08\x03\x02\x02\x02\x1e\x20\x05\x0d\x07\x02\x1f\x1e\x03\x02\x02\x02\
		\x20\x21\x03\x02\x02\x02\x21\x1f\x03\x02\x02\x02\x21\x22\x03\x02\x02\x02\
		\x22\x29\x03\x02\x02\x02\x23\x25\x07\x30\x02\x02\x24\x26\x05\x0d\x07\x02\
		\x25\x24\x03\x02\x02\x02\x26\x27\x03\x02\x02\x02\x27\x25\x03\x02\x02\x02\
		\x27\x28\x03\x02\x02\x02\x28\x2a\x03\x02\x02\x02\x29\x23\x03\x02\x02\x02\
		\x29\x2a\x03\x02\x02\x02\x2a\x0a\x03\x02\x02\x02\x2b\x2f\x07\x24\x02\x02\
		\x2c\x2e\x0b\x02\x02\x02\x2d\x2c\x03\x02\x02\x02\x2e\x31\x03\x02\x02\x02\
		\x2f\x30\x03\x02\x02\x02\x2f\x2d\x03\x02\x02\x02\x30\x32\x03\x02\x02\x02\
		\x31\x2f\x03\x02\x02\x02\x32\x33\x07\x24\x02\x02\x33\x0c\x03\x02\x02\x02\
		\x34\x35\x04\x32\x3b\x02\x35\x0e\x03\x02\x02\x02\x08\x02\x1a\x21\x27\x29\
		\x2f\x03\x08\x02\x02";
