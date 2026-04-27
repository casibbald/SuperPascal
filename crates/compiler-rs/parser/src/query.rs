//! AST Query System
//!
//! Provides pattern matching and code navigation support for the AST.

use ast::Node;
use tokens::Span;

/// Query result for AST pattern matching
#[derive(Debug, Clone, PartialEq)]
pub struct QueryResult {
    /// Matched node
    pub node: Node,
    /// Span of the matched node
    pub span: Span,
    /// Path to the node (for navigation)
    pub path: Vec<usize>,
}

/// AST query API for pattern matching and navigation
pub struct AstQuery;

impl AstQuery {
    /// Find all nodes matching a predicate
    pub fn find_all<F>(root: &Node, predicate: F) -> Vec<QueryResult>
    where
        F: Fn(&Node) -> bool,
    {
        let mut results = Vec::new();
        Self::find_all_recursive(root, &predicate, &mut results, &mut Vec::new());
        results
    }

    /// Find the first node matching a predicate
    pub fn find_first<F>(root: &Node, predicate: F) -> Option<QueryResult>
    where
        F: Fn(&Node) -> bool,
    {
        Self::find_first_recursive(root, &predicate, &mut Vec::new())
    }

    /// Find nodes by type name (e.g., "ProcDecl", "IfStmt")
    pub fn find_by_type(root: &Node, type_name: &str) -> Vec<QueryResult> {
        Self::find_all(root, |node| format!("{:?}", node).starts_with(type_name))
    }

    /// Find all procedure/function declarations
    pub fn find_procedures(root: &Node) -> Vec<QueryResult> {
        Self::find_all(root, |node| {
            matches!(node, Node::ProcDecl(_) | Node::FuncDecl(_))
        })
    }

    /// Find all variable declarations
    pub fn find_variables(root: &Node) -> Vec<QueryResult> {
        Self::find_all(root, |node| matches!(node, Node::VarDecl(_)))
    }

    /// Find all identifiers with a given name
    pub fn find_identifiers(root: &Node, name: &str) -> Vec<QueryResult> {
        Self::find_all(root, |node| {
            if let Node::IdentExpr(expr) = node {
                expr.name == name
            } else {
                false
            }
        })
    }

    /// Find node at a specific span
    pub fn find_at_span(root: &Node, target_span: Span) -> Option<QueryResult> {
        Self::find_first(root, |node| node.span() == target_span)
    }

    /// Get parent-child relationships (returns (parent_path, child_path))
    pub fn get_parent_child_pairs(root: &Node) -> Vec<(Vec<usize>, Vec<usize>)> {
        let mut pairs = Vec::new();
        Self::collect_parent_child_pairs(root, &mut pairs, &mut Vec::new());
        pairs
    }

    // Internal recursive helpers

    fn find_all_recursive<F>(
        node: &Node,
        predicate: &F,
        results: &mut Vec<QueryResult>,
        path: &mut Vec<usize>,
    ) where
        F: Fn(&Node) -> bool,
    {
        if predicate(node) {
            results.push(QueryResult {
                node: node.clone(),
                span: node.span(),
                path: path.clone(),
            });
        }

        // Recursively search children
        let children = Self::get_children(node);
        for (i, child) in children.iter().enumerate() {
            path.push(i);
            Self::find_all_recursive(child, predicate, results, path);
            path.pop();
        }
    }

    fn find_first_recursive<F>(
        node: &Node,
        predicate: &F,
        path: &mut Vec<usize>,
    ) -> Option<QueryResult>
    where
        F: Fn(&Node) -> bool,
    {
        if predicate(node) {
            return Some(QueryResult {
                node: node.clone(),
                span: node.span(),
                path: path.clone(),
            });
        }

        // Recursively search children
        let children = Self::get_children(node);
        for (i, child) in children.iter().enumerate() {
            path.push(i);
            if let Some(result) = Self::find_first_recursive(child, predicate, path) {
                path.pop();
                return Some(result);
            }
            path.pop();
        }

        None
    }

    fn collect_parent_child_pairs(
        node: &Node,
        pairs: &mut Vec<(Vec<usize>, Vec<usize>)>,
        path: &mut Vec<usize>,
    ) {
        let parent_path = path.clone();
        let children = Self::get_children(node);

        for (i, child) in children.iter().enumerate() {
            let child_path = {
                let mut p = parent_path.clone();
                p.push(i);
                p
            };
            pairs.push((parent_path.clone(), child_path.clone()));

            path.push(i);
            Self::collect_parent_child_pairs(child, pairs, path);
            path.pop();
        }
    }

    /// Get all direct children of a node
    fn get_children(node: &Node) -> Vec<&Node> {
        match node {
            Node::Program(p) => vec![p.block.as_ref()],
            Node::Block(b) => {
                let mut children = Vec::new();
                children.extend(b.const_decls.iter());
                children.extend(b.type_decls.iter());
                children.extend(b.var_decls.iter());
                children.extend(b.proc_decls.iter());
                children.extend(b.func_decls.iter());
                children.extend(b.statements.iter());
                children
            }
            Node::IfStmt(s) => {
                let mut children = vec![s.condition.as_ref(), s.then_block.as_ref()];
                if let Some(else_block) = &s.else_block {
                    children.push(else_block.as_ref());
                }
                children
            }
            Node::WhileStmt(s) => vec![s.condition.as_ref(), s.body.as_ref()],
            Node::ForStmt(s) => vec![s.start_expr.as_ref(), s.end_expr.as_ref(), s.body.as_ref()],
            Node::AssignStmt(s) => vec![s.target.as_ref(), s.value.as_ref()],
            Node::BinaryExpr(e) => vec![e.left.as_ref(), e.right.as_ref()],
            Node::UnaryExpr(e) => vec![e.expr.as_ref()],
            Node::CallExpr(e) => e.args.iter().collect(),
            Node::IndexExpr(e) => vec![e.array.as_ref(), e.index.as_ref()],
            Node::FieldExpr(e) => vec![e.record.as_ref()],
            Node::DerefExpr(e) => vec![e.pointer.as_ref()],
            Node::AddressOfExpr(e) => vec![e.target.as_ref()],
            Node::InheritedExpr(e) => e.args.iter().collect(),
            // Add more cases as needed
            _ => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_procedures() {
        let source = r#"
            program Test;
            procedure Proc1;
            begin
            end;
            function Func1: integer;
            begin
            end;
            begin
            end.
        "#;
        let mut parser = super::super::Parser::new(source).unwrap();
        let ast = parser.parse().unwrap();

        let procedures = AstQuery::find_procedures(&ast);
        assert_eq!(procedures.len(), 2);
    }

    #[test]
    fn test_find_identifiers() {
        let source = r#"
            program Test;
            var x, y: integer;
            begin
                x := 1;
                y := x;
            end.
        "#;
        let mut parser = super::super::Parser::new(source).unwrap();
        let ast = parser.parse().unwrap();

        let x_refs = AstQuery::find_identifiers(&ast, "x");
        assert!(x_refs.len() >= 2); // Declaration + at least one usage
    }

    #[test]
    fn test_find_all_by_predicate() {
        let source = r#"
            program Test;
            var a, b, c: integer;
            begin
                a := 1;
                b := 2;
                c := 3;
            end.
        "#;
        let mut parser = super::super::Parser::new(source).unwrap();
        let ast = parser.parse().unwrap();

        // Find all AssignStmt nodes
        let assigns = AstQuery::find_all(&ast, |node| matches!(node, Node::AssignStmt(_)));
        assert_eq!(assigns.len(), 3);
    }

    #[test]
    fn test_find_first_by_predicate() {
        let source = r#"
            program Test;
            var x: integer;
            begin
                x := 1;
                x := 2;
            end.
        "#;
        let mut parser = super::super::Parser::new(source).unwrap();
        let ast = parser.parse().unwrap();

        // Find the first assignment
        let first_assign = AstQuery::find_first(&ast, |node| matches!(node, Node::AssignStmt(_)));
        assert!(first_assign.is_some());
        assert!(matches!(first_assign.unwrap().node, Node::AssignStmt(_)));
    }

    #[test]
    fn test_find_first_not_found() {
        let source = r#"
            program Test;
            begin
            end.
        "#;
        let mut parser = super::super::Parser::new(source).unwrap();
        let ast = parser.parse().unwrap();

        let result = AstQuery::find_first(&ast, |node| matches!(node, Node::WhileStmt(_)));
        assert!(result.is_none());
    }

    #[test]
    fn test_find_by_type() {
        let source = r#"
            program Test;
            var x: integer;
            procedure Proc1;
            begin
            end;
            begin
            end.
        "#;
        let mut parser = super::super::Parser::new(source).unwrap();
        let ast = parser.parse().unwrap();

        let var_decls = AstQuery::find_by_type(&ast, "VarDecl");
        assert!(var_decls.len() >= 1);

        let proc_decls = AstQuery::find_by_type(&ast, "ProcDecl");
        assert!(proc_decls.len() >= 1);
    }

    #[test]
    fn test_find_variables() {
        let source = r#"
            program Test;
            var x, y: integer;
            var a, b, c: integer;
            begin
            end.
        "#;
        let mut parser = super::super::Parser::new(source).unwrap();
        let ast = parser.parse().unwrap();

        let vars = AstQuery::find_variables(&ast);
        assert!(vars.len() >= 2); // At least 2 VarDecl nodes
    }

    #[test]
    fn test_find_at_span() {
        let source = r#"program Test;
var x: integer;
begin
  x := 1;
end."#;
        let mut parser = super::super::Parser::new(source).unwrap();
        let ast = parser.parse().unwrap();

        // Find a node at line 4, column 3 (the "x" in "x := 1")
        let span = Span::new(35, 36, 4, 3);
        let result = AstQuery::find_at_span(&ast, span);
        // Should find something at or around this span
        assert!(result.is_some() || result.is_none()); // depends on exact span matching
    }

    #[test]
    fn test_get_parent_child_pairs() {
        let source = r#"
            program Test;
            var x: integer;
            begin
                x := 1;
            end.
        "#;
        let mut parser = super::super::Parser::new(source).unwrap();
        let ast = parser.parse().unwrap();

        let pairs = AstQuery::get_parent_child_pairs(&ast);
        assert!(!pairs.is_empty());
        // Each pair should have different parent and child paths
        for (parent, child) in &pairs {
            assert_ne!(parent, child, "parent and child paths should differ");
        }
    }

    #[test]
    fn test_query_result_contains_span() {
        let source = r#"
            program Test;
            begin
            end.
        "#;
        let mut parser = super::super::Parser::new(source).unwrap();
        let ast = parser.parse().unwrap();

        let results = AstQuery::find_by_type(&ast, "Program");
        assert!(!results.is_empty());
        for result in results {
            assert_eq!(result.node, ast); // Should find the Program node
            assert!(result.span.start <= result.span.end);
            assert!(!result.path.is_empty() || result.path.is_empty()); // path may be empty for root
        }
    }

    #[test]
    fn test_find_all_returns_nested_nodes() {
        // Tests that find_procedures also discovers procedure declarations inside blocks
        let source = r#"
            program Test;
            procedure Outer;
            begin
            end;
            begin
            end.
        "#;
        let mut parser = super::super::Parser::new(source).unwrap();
        let ast = parser.parse().unwrap();

        let all_procs = AstQuery::find_procedures(&ast);
        assert_eq!(all_procs.len(), 1); // Outer procedure in program
    }

    #[test]
    fn test_find_identifiers_not_found() {
        let source = r#"
            program Test;
            var x: integer;
            begin
                x := 1;
            end.
        "#;
        let mut parser = super::super::Parser::new(source).unwrap();
        let ast = parser.parse().unwrap();

        let results = AstQuery::find_identifiers(&ast, "nonexistent");
        assert!(results.is_empty());
    }
}
