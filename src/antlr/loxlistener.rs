#![allow(nonstandard_style)]
// Generated from ../Lox.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::loxparser::*;

pub trait LoxListener<'input> : ParseTreeListener<'input,LoxParserContextType>{
/**
 * Enter a parse tree produced by {@link LoxParser#program}.
 * @param ctx the parse tree
 */
fn enter_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LoxParser#program}.
 * @param ctx the parse tree
 */
fn exit_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LoxParser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LoxParser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LoxParser#printStmt}.
 * @param ctx the parse tree
 */
fn enter_printStmt(&mut self, _ctx: &PrintStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LoxParser#printStmt}.
 * @param ctx the parse tree
 */
fn exit_printStmt(&mut self, _ctx: &PrintStmtContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code strval}
 * labeled alternative in {@link LoxParser#term}.
 * @param ctx the parse tree
 */
fn enter_strval(&mut self, _ctx: &StrvalContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code strval}
 * labeled alternative in {@link LoxParser#term}.
 * @param ctx the parse tree
 */
fn exit_strval(&mut self, _ctx: &StrvalContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code numval}
 * labeled alternative in {@link LoxParser#term}.
 * @param ctx the parse tree
 */
fn enter_numval(&mut self, _ctx: &NumvalContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code numval}
 * labeled alternative in {@link LoxParser#term}.
 * @param ctx the parse tree
 */
fn exit_numval(&mut self, _ctx: &NumvalContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : LoxListener<'input> }


