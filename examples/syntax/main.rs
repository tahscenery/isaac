mod node;
mod token;

use cavea::Arena;
use node::{
    SyntaxNode as SN,
    SyntaxNodeKind as SNK,
};
use token::{
    RawSyntaxTokenData as RSData,
    SyntaxToken as ST,
    SyntaxTokenKind as STK,
};
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Syntax {
    Node(SN),
    Token(ST),
}

fn main() {
    // a * (-2 + a) - 10
    // -----------------
    //         -
    //        / \
    //       *   10
    //      / \
    //     a   +
    //        / \
    //       -   a
    //       |
    //       2
    let arena = &mut Arena::new();

    use Syntax as S;
    use token::*;
    use node::*;

    let raw_token_symbol_minus =
        Rc::new(RSData::new(STK::Symbol(Symbol::Minus), "-"));
    let raw_token_symbol_asterisk =
        Rc::new(RSData::new(STK::Symbol(Symbol::Asterisk), "*"));
    let raw_token_identifier_a =
        Rc::new(RSData::new(STK::Identifier, "a"));
    let raw_token_symbol_plus =
        Rc::new(RSData::new(STK::Symbol(Symbol::Plus), "+"));
    let raw_token_literal_two =
        Rc::new(RSData::new(STK::Literal(Literal::Integer), "2"));
    let raw_token_literal_ten =
        Rc::new(RSData::new(STK::Literal(Literal::Integer), "2"));

    let node_expr_binary =
        Rc::new(S::Node(SN::new(SNK::Expr(Expr::Binary))));
    let node_expr_unary =
        Rc::new(S::Node(SN::new(SNK::Expr(Expr::Unary))));

    let token_symbol_minus_1 =
        Rc::new(S::Token(ST::new(Rc::clone(&raw_token_symbol_minus), 13, 1)));
    let token_symbol_asterisk_1 =
        Rc::new(S::Token(ST::new(Rc::clone(&raw_token_symbol_asterisk), 2, 1)));
    let token_identifier_a_1 =
        Rc::new(S::Token(ST::new(Rc::clone(&raw_token_identifier_a), 0, 1)));
    let token_symbol_plus_1 =
        Rc::new(S::Token(ST::new(Rc::clone(&raw_token_symbol_plus), 8, 1)));
    let token_symbol_minus_2 =
        Rc::new(S::Token(ST::new(Rc::clone(&raw_token_symbol_minus), 5, 1)));
    let token_literal_two_1 =
        Rc::new(S::Token(ST::new(Rc::clone(&raw_token_literal_two), 6, 1)));
    let token_identifier_a_2 =
        Rc::new(S::Token(ST::new(Rc::clone(&raw_token_identifier_a), 10, 1)));
    let token_literal_ten_1 =
        Rc::new(S::Token(ST::new(Rc::clone(&raw_token_literal_ten), 15, 2)));

    let arena_token_minus_1 =
        arena.insert(Rc::clone(&token_symbol_minus_1));
    let arena_binary_minus_1 =
        arena.insert(Rc::clone(&node_expr_binary));
    let arena_token_asterisk_1 =
        arena.insert(Rc::clone(&token_symbol_asterisk_1));
    let arena_binary_asterisk_1 =
        arena.insert(Rc::clone(&node_expr_binary));
    let arena_token_identifier_a_1 =
        arena.insert(Rc::clone(&token_identifier_a_1));
    let arena_token_plus_1 =
        arena.insert(Rc::clone(&token_symbol_plus_1));
    let arena_binary_plus_1 =
        arena.insert(Rc::clone(&node_expr_binary));
    let arena_token_minus_2 =
        arena.insert(Rc::clone(&token_symbol_minus_2));
    let arena_unary_minus_1 =
        arena.insert(Rc::clone(&node_expr_unary));
    let arena_token_literal_two_1 =
        arena.insert(Rc::clone(&token_literal_two_1));
    let arena_token_identifier_a_2 =
        arena.insert(Rc::clone(&token_identifier_a_2));
    let arena_token_literal_ten =
        arena.insert(Rc::clone(&token_literal_ten_1));

    arena_binary_minus_1
        .add_child(arena, arena_binary_asterisk_1)
        .add_child(arena, arena_token_minus_1)
        .add_child(arena, arena_token_literal_ten);

    arena_binary_asterisk_1
        .add_child(arena, arena_token_identifier_a_1)
        .add_child(arena, arena_token_asterisk_1)
        .add_child(arena, arena_binary_plus_1);

    arena_binary_plus_1
        .add_child(arena, arena_unary_minus_1)
        .add_child(arena, arena_token_plus_1)
        .add_child(arena, arena_token_identifier_a_2);

    arena_unary_minus_1
        .add_child(arena, arena_token_minus_2)
        .add_child(arena, arena_token_literal_two_1);

    arena.set_root(arena_unary_minus_1);
    println!("{:#?}", arena);
}