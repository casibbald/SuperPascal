# SuperPascal Error Design Strategy

## Overview

SuperPascal's error handling should **closely match FreePascal's format** for familiarity and compatibility, while **enriching messages** with additional diagnostic information, suggestions, and context.

## Design Principles

1. **FPC Compatibility First**: Match FPC's error message format and severity levels
2. **Enhanced Diagnostics**: Add helpful suggestions, context, and explanations
3. **Progressive Enhancement**: Core messages match FPC, extras are additive
4. **User Choice**: Allow users to control verbosity (FPC-compat vs. enhanced mode)

## FreePascal Error Format

FPC uses a structured message system with severity levels:

```
<filename>(<line>,<column>) <severity>: <message>
```

**Severity Levels:**
- **Note**: Informational (not an error)
- **Hint**: Suggestions for improvement
- **Warning**: Potential issues (compilation continues)
- **Error**: Compilation errors (compilation continues to find more)
- **Fatal**: Critical errors (compilation stops)

**Example FPC Messages:**
```
test.pas(10,15) Error: Identifier not found "xyz"
test.pas(12,5) Warning: Variable "x" does not seem to be initialized
test.pas(15,20) Hint: Local variable "y" is assigned but never used
```

## SuperPascal Enhanced Format

### Base Format (FPC-Compatible)

```
<filename>(<line>,<column>) <severity>: <message>
```

### Enhanced Format (Default)

```
<filename>(<line>,<column>) <severity>: <message>
  └─ <context>
  └─ <suggestion>
  └─ <related_locations>
```

### Example Enhanced Messages

**Basic (FPC-compatible):**
```
test.pas(10,15) Error: Identifier not found "xyz"
```

**Enhanced (SuperPascal default):**
```
test.pas(10,15) Error: Identifier not found "xyz"
  └─ In procedure 'DoSomething' (declared at test.pas:5,3)
  └─ Suggestion: Did you mean 'xy'? (declared at test.pas:2,10)
  └─ Related: Variable 'xyz' was used here (test.pas:8,12)
```

**With code snippet:**
```
test.pas(10,15) Error: Identifier not found "xyz"
  └─ In procedure 'DoSomething' (declared at test.pas:5,3)
  └─ Suggestion: Did you mean 'xy'? (declared at test.pas:2,10)
  └─ Related: Variable 'xyz' was used here (test.pas:8,12)
  
  8  |   var xyz: integer;
  9  |   begin
>10 |     writeln(xyz);  // Error here
     |            ^^^
```

## Error Categories

### 1. Lexer Errors
- **FPC Format**: `Error: Illegal character`
- **Enhanced**: Add character code, suggest encoding issues

### 2. Parser Errors
- **FPC Format**: `Error: Syntax error, ";" expected but "end" found`
- **Enhanced**: Show expected vs. found, suggest fixes, show context

### 3. Semantic Errors
- **FPC Format**: `Error: Identifier not found "xyz"`
- **Enhanced**: Suggest similar identifiers, show declaration locations

### 4. Type Errors
- **FPC Format**: `Error: Incompatible types: got "Integer" expected "String"`
- **Enhanced**: Show type hierarchy, suggest conversions, show related types

### 5. Symbol Errors
- **FPC Format**: `Error: Variable "x" already declared`
- **Enhanced**: Show previous declaration location, suggest rename

## Implementation Strategy

### Phase 1: FPC-Compatible Base (Current)
- Match FPC's severity levels
- Match FPC's message format
- Basic location information

### Phase 2: Enhanced Diagnostics
- Add suggestions (did you mean...)
- Add context (in procedure X...)
- Add related locations
- Add code snippets

### Phase 3: Interactive Mode
- Clickable locations in IDEs
- Quick fixes
- Error explanations

## Error Structure

```rust
pub enum ErrorSeverity {
    Note,
    Hint,
    Warning,
    Error,
    Fatal,
}

pub struct Diagnostic {
    pub severity: ErrorSeverity,
    pub message: String,           // FPC-compatible message
    pub span: Span,
    pub file: Option<String>,
    
    // Enhanced fields (optional)
    pub context: Option<String>,           // "In procedure X..."
    pub suggestion: Option<String>,        // "Did you mean Y?"
    pub related_locations: Vec<RelatedLocation>,
    pub code_snippet: Option<CodeSnippet>,
    pub explanation: Option<String>,        // Detailed explanation
}

pub struct RelatedLocation {
    pub message: String,
    pub span: Span,
    pub file: Option<String>,
}

pub struct CodeSnippet {
    pub lines: Vec<(usize, String)>,  // (line_number, content)
    pub highlight_span: Span,
}
```

## Display Modes

### 1. FPC-Compatible Mode (`--fpc-errors`)
```
test.pas(10,15) Error: Identifier not found "xyz"
```

### 2. Enhanced Mode (default)
```
test.pas(10,15) Error: Identifier not found "xyz"
  └─ In procedure 'DoSomething' (declared at test.pas:5,3)
  └─ Suggestion: Did you mean 'xy'? (declared at test.pas:2,10)
```

### 3. Verbose Mode (`--verbose-errors`)
```
test.pas(10,15) Error: Identifier not found "xyz"
  └─ In procedure 'DoSomething' (declared at test.pas:5,3)
  └─ Suggestion: Did you mean 'xy'? (declared at test.pas:2,10)
  └─ Related: Variable 'xyz' was used here (test.pas:8,12)
  
  8  |   var xyz: integer;
  9  |   begin
>10 |     writeln(xyz);  // Error here
     |            ^^^
  
  Explanation: The identifier 'xyz' is not visible in this scope.
  The variable 'xyz' was declared in procedure 'OtherProc' but is
  not accessible from 'DoSomething'.
```

## Migration Path

1. **Start with FPC format** (current implementation)
2. **Add enhanced fields** (optional, backward compatible)
3. **Enable by default** (users can opt-out with `--fpc-errors`)
4. **Add IDE integration** (structured output for tooling)

## Benefits

1. **Familiarity**: FPC users feel at home
2. **Compatibility**: Scripts/tools expecting FPC format work
3. **Better UX**: Enhanced mode helps new users
4. **Flexibility**: Users choose their preferred verbosity
5. **Tooling**: Structured format enables IDE integration

## Example Implementation

```rust
impl Diagnostic {
    /// Format as FPC-compatible message
    pub fn format_fpc(&self) -> String {
        let file = self.file.as_deref().unwrap_or("unknown");
        format!(
            "{}({},{}) {}: {}",
            file,
            self.span.line,
            self.span.column,
            self.severity.as_str(),
            self.message
        )
    }
    
    /// Format as enhanced message
    pub fn format_enhanced(&self) -> String {
        let mut output = self.format_fpc();
        
        if let Some(context) = &self.context {
            output.push_str(&format!("\n  └─ {}", context));
        }
        
        if let Some(suggestion) = &self.suggestion {
            output.push_str(&format!("\n  └─ Suggestion: {}", suggestion));
        }
        
        // ... more enhancements
        
        output
    }
}
```

## Conclusion

**Yes, we should closely match FreePascal's error format**, then add enriched information as optional enhancements. This provides:
- **Compatibility** with existing tooling
- **Familiarity** for FPC users
- **Better UX** for new users
- **Flexibility** for different use cases

