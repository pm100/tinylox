#![allow(nonstandard_style)]
// Generated from ../Lox.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::loxparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link LoxParser}.
 */
pub trait LoxVisitor<'input>: ParseTreeVisitor<'input,LoxParserContextType>{
	/**
	 * Visit a parse tree produced by {@link LoxParser#program}.
	 * @param ctx the parse tree
	 */
	fn visit_program(&mut self, ctx: &ProgramContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LoxParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_statement(&mut self, ctx: &StatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LoxParser#printStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_printStmt(&mut self, ctx: &PrintStmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code strval}
	 * labeled alternative in {@link LoxParser#term}.
	 * @param ctx the parse tree
	 */
	fn visit_strval(&mut self, ctx: &StrvalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code numval}
	 * labeled alternative in {@link LoxParser#term}.
	 * @param ctx the parse tree
	 */
	fn visit_numval(&mut self, ctx: &NumvalContext<'input>) { self.visit_children(ctx) }

}

pub trait LoxVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= LoxParserContextType>{
	/**
	 * Visit a parse tree produced by {@link LoxParser#program}.
	 * @param ctx the parse tree
	 */
		fn visit_program(&mut self, ctx: &ProgramContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LoxParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_statement(&mut self, ctx: &StatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LoxParser#printStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_printStmt(&mut self, ctx: &PrintStmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code strval}
	 * labeled alternative in {@link LoxParser#term}.
	 * @param ctx the parse tree
	 */
		fn visit_strval(&mut self, ctx: &StrvalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code numval}
	 * labeled alternative in {@link LoxParser#term}.
	 * @param ctx the parse tree
	 */
		fn visit_numval(&mut self, ctx: &NumvalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> LoxVisitor<'input> for T
where
	T: LoxVisitorCompat<'input>
{
	fn visit_program(&mut self, ctx: &ProgramContext<'input>){
		let result = <Self as LoxVisitorCompat>::visit_program(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statement(&mut self, ctx: &StatementContext<'input>){
		let result = <Self as LoxVisitorCompat>::visit_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_printStmt(&mut self, ctx: &PrintStmtContext<'input>){
		let result = <Self as LoxVisitorCompat>::visit_printStmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_strval(&mut self, ctx: &StrvalContext<'input>){
		let result = <Self as LoxVisitorCompat>::visit_strval(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_numval(&mut self, ctx: &NumvalContext<'input>){
		let result = <Self as LoxVisitorCompat>::visit_numval(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}