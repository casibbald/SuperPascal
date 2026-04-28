//! Parser performance benchmarks
//!
//! Run with: cargo bench --package parser

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use parser::Parser;

fn bench_parse_simple_program(c: &mut Criterion) {
    let source = r#"
        program Test;
        var x: integer;
        begin
            x := 42;
            writeln(x);
        end.
    "#;

    c.bench_function("parse_simple_program", |b| {
        b.iter(|| {
            let mut parser = Parser::new(black_box(source)).unwrap();
            black_box(parser.parse()).unwrap();
        });
    });
}

fn bench_parse_large_program(c: &mut Criterion) {
    // Generate a larger program with many declarations and statements
    let mut source = String::from("program LargeTest;\n");
    source.push_str("var\n");
    for i in 0..100 {
        source.push_str(&format!("    x{}: integer;\n", i));
    }
    source.push_str("begin\n");
    for i in 0..100 {
        source.push_str(&format!("    x{} := {};\n", i, i));
    }
    source.push_str("end.\n");

    c.bench_function("parse_large_program", |b| {
        b.iter(|| {
            let mut parser = Parser::new(black_box(&source)).unwrap();
            black_box(parser.parse()).unwrap();
        });
    });
}

fn bench_parse_nested_blocks(c: &mut Criterion) {
    let mut source = String::from("program NestedTest;\n");
    source.push_str("procedure Outer;\n");
    source.push_str("    procedure Inner1;\n");
    source.push_str("        procedure Inner2;\n");
    source.push_str("        begin\n");
    source.push_str("        end;\n");
    source.push_str("    begin\n");
    source.push_str("    end;\n");
    source.push_str("begin\n");
    source.push_str("end;\n");
    source.push_str("begin\n");
    source.push_str("    Outer;\n");
    source.push_str("end.\n");

    c.bench_function("parse_nested_blocks", |b| {
        b.iter(|| {
            let mut parser = Parser::new(black_box(&source)).unwrap();
            black_box(parser.parse()).unwrap();
        });
    });
}

fn bench_parse_expressions(c: &mut Criterion) {
    let source = r#"
        program ExprTest;
        var x, y, z: integer;
        begin
            x := (1 + 2) * (3 - 4) / (5 mod 6);
            y := x * 2 + 3 * 4 - 5;
            z := (x + y) * (y - x) / (x * y);
        end.
    "#;

    c.bench_function("parse_complex_expressions", |b| {
        b.iter(|| {
            let mut parser = Parser::new(black_box(source)).unwrap();
            black_box(parser.parse()).unwrap();
        });
    });
}

criterion_group!(
    benches,
    bench_parse_simple_program,
    bench_parse_large_program,
    bench_parse_nested_blocks,
    bench_parse_expressions
);
criterion_main!(benches);
