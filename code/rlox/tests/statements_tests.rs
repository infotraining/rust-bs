use rlox::ast::*;
use rlox::parser::Parser;

#[test]
fn parsing_print_statement() {
    let source_code: &str = "print 42;";

    let mut parser = Parser::new(source_code);
    let statements = parser.parse_source();

    let expected_statements = vec![
        Statement::PrintStmt(Expression::Literal(Value::Number(42.0)))
    ];

    assert_eq!(statements, expected_statements);
}
