# String Interpolation Specification

## Overview

String interpolation allows embedding Pascal expressions within string literals using the syntax `"text {expression} more text"`. This is similar to Python's f-strings or JavaScript's template literals.

## Syntax

### Basic Syntax
```pascal
"Hello {name}"
"Value: {x + y}"
"Count: {items.length}"
```

### Multiple Interpolations
```pascal
"Hello {firstName} {lastName}"
"Result: {a} + {b} = {sum(a, b)}"
```

### Empty String Parts
```pascal
"{expression}"           // No leading text
"text {expr}"            // No trailing text
"{expr1}{expr2}"         // No text between expressions
```

### Nested Braces in Expressions
```pascal
"Array: {arr[{index}]}"           // Array indexing with brackets
"Set: {mySet[{key}]}"              // Set access with brackets
"Record: {rec.field[{idx}]}"        // Complex nested access
```

### Edge Cases
```pascal
"Text {x} {y} more"                // Multiple interpolations
"Text {x + {y}}"                   // INVALID - nested interpolation not supported
"Text {x} {y} {z}"                 // Three interpolations
""                                 // Empty string (no interpolation)
"Just text"                        // No interpolation
```

## Token Sequence

For the string `"Hello {name} world"`, the lexer should produce:

1. `StringLiteral("Hello ")` - text before `{`
2. `LeftBrace` - opening brace
3. `Identifier("name")` - the expression tokens
4. `RightBrace` - closing brace
5. `StringLiteral(" world")` - text after `}`
6. `Eof`

## Lexer State Machine

### States

1. **Normal State**: Scanning regular tokens
2. **String Scanning State**: Inside a string literal, accumulating characters
3. **Interpolation State**: After encountering `{` in a string, parsing expression
4. **String Continuation State**: After expression closes with `}`, continuing string

### State Transitions

```
Normal State
  └─> [encounter "] → String Scanning State

String Scanning State
  ├─> [encounter {] → Store accumulated text, set string_continuation, return StringLiteral, store LeftBrace in lookahead
  ├─> [encounter "] → Return StringLiteral, exit string
  └─> [other chars] → Accumulate character

Interpolation State (after LeftBrace)
  ├─> [encounter {] → Increment brace_depth (nested brace in expression)
  ├─> [encounter }] → 
  │   ├─> [brace_depth > 1] → Decrement brace_depth, return RightBrace (nested)
  │   └─> [brace_depth == 1] → Decrement to 0, return RightBrace (closing interpolation)
  └─> [other tokens] → Return normally (part of expression)

String Continuation State (after RightBrace closes interpolation)
  ├─> [encounter {] → Another interpolation, store continuation, return StringLiteral, store LeftBrace
  ├─> [encounter "] → End of string, return StringLiteral
  └─> [other chars] → Accumulate character
```

## Parsing Flow

### Lexer Flow (Textual)

```
1. Start scanning string literal
   └─> Accumulate characters until '{' or closing quote

2. Encounter '{' in string
   ├─> Store accumulated text in string_continuation
   ├─> Store LeftBrace in lookahead
   ├─> Set brace_depth = 1
   └─> Return StringLiteral(accumulated_text)

3. Parser consumes LeftBrace from lookahead
   └─> Parser calls parse_expression()

4. During expression parsing:
   ├─> If encounter '{' → increment brace_depth, return LeftBrace
   ├─> If encounter '}' → 
   │   ├─> If brace_depth > 1 → decrement, return RightBrace (nested)
   │   └─> If brace_depth == 1 → decrement to 0, return RightBrace (closing)
   └─> Other tokens → return normally

5. Parser consumes RightBrace (closing interpolation)
   └─> Lexer detects brace_depth == 0

6. Next token call after RightBrace:
   ├─> Check string_continuation (brace_depth == 0)
   ├─> Continue scanning string from after '}'
   └─> Return StringLiteral(remaining_text)

7. If another '{' encountered:
   └─> Repeat from step 2

8. If closing quote encountered:
   └─> Return StringLiteral, end interpolation
```

### Lexer Flow (Flowchart)

```
START next_token()
  │
  ├─> skip_whitespace()
  │
  ├─> Check directives (skip if directive)
  │
  ├─> skip_comments()
  │   └─> If comment found, skip it completely
  │
  ├─> Check EOF → return Eof if at end
  │
  ├─> Check string_continuation
  │   │
  │   ├─> If Some AND current char is '{'
  │   │   └─> Increment brace_depth, restore state, continue
  │   │
  │   ├─> If Some AND current char is '}'
  │   │   ├─> Decrement brace_depth
  │   │   ├─> If brace_depth == 0
  │   │   │   └─> Return RightBrace, set brace_depth=0
  │   │   └─> If brace_depth > 0
  │   │       └─> Restore state, continue (nested brace)
  │   │
  │   └─> If Some AND brace_depth == 0 (after RightBrace consumed)
  │       └─> continue_interpolated_string()
  │           └─> Return StringLiteral(remaining_text)
  │
  └─> Normal tokenization
      ├─> If '"' → scan_string_double()
      │   ├─> If '{' found in string
      │   │   ├─> Store string_continuation (brace_depth=1)
      │   │   ├─> Store LeftBrace in lookahead
      │   │   └─> Return StringLiteral(accumulated)
      │   └─> Else → Return StringLiteral(full_string)
      │
      └─> Other tokens → scan normally
```

### Parser Flow (Flowchart)

```
parse_prefix() encounters StringLiteral
  │
  ├─> Check if next token is LeftBrace
  │   │
  │   ├─> Yes → parse_interpolated_string()
  │   │   │
  │   │   ├─> Add first string part to parts[]
  │   │   │
  │   │   └─> While next token is LeftBrace:
  │   │       │
  │   │       ├─> Consume LeftBrace
  │   │       │
  │   │       ├─> parse_expression()
  │   │       │   └─> Returns expression node
  │   │       │
  │   │       ├─> Add expression to parts[]
  │   │       │
  │   │       ├─> Consume RightBrace
  │   │       │   └─> Lexer returns RightBrace when brace_depth==0
  │   │       │
  │   │       ├─> Check next token
  │   │       │   │
  │   │       │   ├─> If StringLiteral (non-empty)
  │   │       │   │   └─> Add to parts[], consume
  │   │       │   │
  │   │       │   └─> If LeftBrace
  │   │       │       └─> Continue loop (another interpolation)
  │   │       │
  │   │       └─> Else → break (end of string)
  │   │
  │   └─> No → Return LiteralExpr(String)
```

### Parser Flow

```
1. Parse string literal token
   └─> Check if next token is LeftBrace
       ├─> Yes → parse_interpolated_string()
       └─> No → return LiteralExpr(String)

2. parse_interpolated_string():
   ├─> Add first string part to parts[]
   ├─> While next token is LeftBrace:
   │   ├─> Consume LeftBrace
   │   ├─> Parse expression (parse_expression())
   │   ├─> Add expression to parts[]
   │   ├─> Consume RightBrace
   │   ├─> If next token is StringLiteral:
   │   │   ├─> Add string part to parts[] (if non-empty)
   │   │   └─> Consume StringLiteral
   │   └─> Check if another LeftBrace (more interpolations)
   └─> Return InterpolatedString(parts)
```

## Brace Depth Tracking

### Purpose
Track nested braces within the interpolated expression to correctly identify the closing brace of the interpolation.

### Rules
- **Initial depth**: 1 (the opening `{` of the interpolation)
- **Increment**: When encountering `{` while in interpolation state
- **Decrement**: When encountering `}` while in interpolation state
- **Return RightBrace**: Only when depth reaches 0 (closing the interpolation)
- **Continue expression**: When depth > 0 (nested brace in expression)

### Examples

```
"Text {x}"                    // depth: 1 → 0 (simple)
"Text {arr[{idx}]}"           // depth: 1 → 2 → 1 → 0 (nested brackets, but braces tracked)
"Text {set[{key}]}"           // depth: 1 → 2 → 1 → 0 (nested)
```

**Note**: In Pascal, `[` and `]` are brackets (for arrays), not braces. Only `{` and `}` are braces. So `arr[{idx}]` has:
- `{` - opening brace (depth 1)
- `[` - bracket (not tracked)
- `{` - nested brace? NO - this is invalid syntax in Pascal expressions
- Actually, `arr[{idx}]` is invalid - you can't have braces inside brackets in expressions

Wait, let me reconsider. In Pascal expressions:
- `arr[index]` - array indexing uses brackets
- `{` and `}` are only used for:
  - Comments: `{ comment }`
  - Sets: `[1, 2, 3]` uses brackets, not braces
  - Records don't use braces

So nested braces in expressions would be:
- Invalid syntax (braces are only for comments, not expressions)
- OR we're allowing some special syntax

Actually, for string interpolation, we need to handle the case where the expression itself might contain braces if we're allowing set literals or other constructs. But in standard Pascal, expressions don't contain braces.

**Revised Understanding**: 
- Braces in Pascal are primarily for comments
- In string interpolation `"text {expression}"`, the expression is a Pascal expression
- Pascal expressions don't contain braces (they use parentheses, brackets, etc.)
- So nested braces in interpolation expressions would be invalid Pascal syntax

**However**, we might want to support:
- `"Text {x + y}"` - simple expression
- `"Text {arr[i]}"` - array indexing (uses brackets, not braces)
- `"Text {func(x)}"` - function call (uses parentheses, not braces)

So brace depth tracking is mainly to handle the case where we might have:
- Comments inside expressions? No, comments are handled by lexer before parsing
- Set literals? No, sets use brackets `[1, 2, 3]`
- Record constructors? No, they use parentheses or special syntax

**Conclusion**: Brace depth tracking is primarily to correctly identify the closing `}` of the interpolation, not to handle nested braces in expressions (which shouldn't exist in valid Pascal).

## Edge Cases and Error Handling

### Valid Cases
1. `"Text {expr}"` - Simple interpolation
2. `"{expr}"` - Expression only, no text
3. `"Text {expr1} {expr2}"` - Multiple interpolations
4. `"Text {expr} more"` - Text before and after

### Invalid Cases
1. `"Text {unclosed"` - Missing closing brace (syntax error)
2. `"Text {expr} {"` - Unclosed second interpolation (syntax error)
3. `"Text {invalid expr}"` - Invalid expression (semantic error, handled later)

### Special Cases
1. **Empty string parts**: `"{expr}"` or `"text {expr}"` - empty parts are valid but not stored
2. **Whitespace**: `"Text { x }"` - whitespace around braces is part of expression parsing
3. **Comments**: `"Text {x (* comment *)}"` - comments in expressions are handled by lexer

## Current Implementation Issues

### Issue 1: Comment Interference
**Problem**: The brace depth check is interfering with comment parsing.

**Root Cause**: After `skip_comment_curly()` processes `{ Outer { Inner } }`, it stops at the first `}`. Then `skip_whitespace()` skips the space, leaving us at the second `}`. The brace depth check then sees this `}` and `string_continuation` might be incorrectly set, causing it to return `RightBrace` instead of continuing to scan `program`.

**Analysis**:
- Input: `"{ Outer { Inner } } program"`
- `skip_comment_curly()`: Skips `{ Outer { Inner } }` (stops at first `}`)
- Position after comment: at the space before second `}`
- `skip_whitespace()`: Skips space, now at second `}`
- `string_continuation` check: Should be `None` (we're not in a string), but might be incorrectly set
- Expected: Continue scanning and get `KwProgram`
- Actual: Returning `RightBrace`

**Solution**: 
1. Ensure `string_continuation` is ONLY set when actually inside a string literal
2. Clear `string_continuation` when string ends (closing quote encountered)
3. The brace depth check should only process `}` when `string_continuation` is actually set AND we're in a string context
4. After comment skipping, if we encounter `}` and `string_continuation` is `None`, it should be scanned as a regular `RightBrace` token (which might be an error, but shouldn't interfere)

### Issue 2: State Management
**Problem**: `string_continuation` state might persist incorrectly.

**Root Cause**: State is not properly cleared when not in a string context, or might be set incorrectly.

**Solution**: 
1. Only set `string_continuation` when actually scanning a string (in `scan_string_double`)
2. Clear `string_continuation` when string ends (closing quote in `continue_interpolated_string`)
3. Never set `string_continuation` outside of string scanning context

### Issue 3: Brace Depth Initialization
**Problem**: Brace depth should start at 1 (for the opening `{`), but might be initialized incorrectly.

**Solution**: When storing `string_continuation` after encountering `{` in a string, set `brace_depth = 1`.

### Issue 4: Comment Processing Order
**Problem**: The order of operations in `next_token()` might cause issues.

**Current Order**:
1. `skip_whitespace()`
2. Check directives
3. `skip_comments()`
4. Check `string_continuation` (brace depth)
5. Continue tokenization

**Issue**: After `skip_comments()`, if we're at a `}` character and `string_continuation` is incorrectly set, we return `RightBrace` instead of continuing.

**Solution**: 
1. Ensure `string_continuation` is only checked when it's actually relevant (we're in a string)
2. After comment skipping, if `string_continuation` is `None`, don't do brace depth checks
3. The `}` character should be scanned normally through `scan_operator_or_delimiter()` if not in string context

## Test Scenarios

### Scenario 1: Simple Interpolation
```
Input: "Hello {name}"
Expected tokens:
  1. StringLiteral("Hello ")
  2. LeftBrace
  3. Identifier("name")
  4. RightBrace
  5. StringLiteral("")
  6. Eof
```

### Scenario 2: Multiple Interpolations
```
Input: "Hello {firstName} {lastName}"
Expected tokens:
  1. StringLiteral("Hello ")
  2. LeftBrace
  3. Identifier("firstName")
  4. RightBrace
  5. StringLiteral(" ")
  6. LeftBrace
  7. Identifier("lastName")
  8. RightBrace
  9. StringLiteral("")
  10. Eof
```

### Scenario 3: Expression Only
```
Input: "{x + y}"
Expected tokens:
  1. StringLiteral("")
  2. LeftBrace
  3. Identifier("x")
  4. Plus
  5. Identifier("y")
  6. RightBrace
  7. StringLiteral("")
  8. Eof
```

### Scenario 4: Comments in Source (Not in String)
```
Input: "{ comment } program"
Expected tokens:
  1. KwProgram (comment is skipped, extra } causes error or is skipped)
```

### Scenario 5: String with Comments Around It
```
Input: { comment } "text {x}" { another }
Expected tokens:
  1. StringLiteral("text ")
  2. LeftBrace
  3. Identifier("x")
  4. RightBrace
  5. StringLiteral("")
  6. Eof
```

## Implementation Checklist

- [x] AST node for InterpolatedString
- [x] Lexer detects `{` in strings
- [x] Lexer returns StringLiteral + LeftBrace
- [x] Lexer tracks brace depth
- [x] Lexer returns RightBrace when depth reaches 0
- [x] Lexer continues string after RightBrace
- [x] Parser parses interpolated strings
- [ ] Fix comment interference issue
- [ ] Semantic analysis for interpolated strings
- [ ] IR generation for string interpolation
- [ ] Runtime support for string formatting
- [ ] Comprehensive tests

## Root Cause Analysis: Comment Test Failure

### The Problem
Test: `"{ Outer { Inner } } program"` expects `KwProgram` but gets `RightBrace`

### Step-by-Step Analysis

1. **Input**: `"{ Outer { Inner } } program"`
2. **Initial State**: `string_continuation = None` (not in a string)
3. **`skip_whitespace()`**: No whitespace at start, position still at `{`
4. **Directive Check**: `{` followed by ` ` (space), not `$`, so not a directive
5. **`skip_comments()`**: 
   - Calls `skip_comment_curly()`
   - Scans `{ Outer { Inner } }`
   - Stops at first `}` (Pascal doesn't support nested comments)
   - Position: after first `}`, at space before second `}`
6. **After `skip_comments()`**:
   - Position: at space before second `}`
   - `string_continuation`: Should still be `None`
7. **Brace Depth Check**:
   - `if let Some(...) = self.string_continuation.take()`: Should be `None`, so skip
   - But wait... if `string_continuation` is somehow set, we'd process it
8. **Continue Tokenization**:
   - `skip_whitespace()` is called again? No, we already skipped whitespace
   - Actually, we need to check: after `skip_comments()`, do we call `skip_whitespace()` again?

### The Real Issue

Looking at the code flow:
- `skip_whitespace()` is called at the START of `next_token()`
- After `skip_comments()`, we don't call `skip_whitespace()` again
- So if `skip_comment_curly()` leaves us at a space, we're still at that space
- Then we check `string_continuation` - if it's `None`, we skip the check
- Then we continue to tokenization, which should scan the next character

**But wait**: After `skip_comment_curly()` processes `{ Outer { Inner } }`, it stops at the first `}`. The comment is `{ Outer { Inner } }`, so:
- It scans `{ Outer { Inner } }`
- It encounters the first `}`
- It advances past that `}`
- It returns

So after the comment, we should be at the **space** before the second `}`.

Then when we continue tokenization:
- We're at a space
- We call `skip_whitespace()` again? No, we already did at the start
- Actually, we DON'T call it again after comments
- So we're at a space, and `current_char()` would be a space
- But we're checking `string_continuation` first...

**Ah!** The issue is that after `skip_comments()`, we check `string_continuation` BEFORE we've skipped the whitespace that might be after the comment. But we already skipped whitespace at the start, so we shouldn't have whitespace here.

**Wait, let me reconsider**: 
- Start: position 0, char `{`
- `skip_whitespace()`: no whitespace, still at `{`
- `skip_comments()`: processes `{ Outer { Inner } }`, stops at first `}`, advances past it
- After comment: position after first `}`, which is the space before second `}`
- Check `string_continuation`: Should be `None`
- Continue: We're at a space, but we don't skip whitespace again
- `current_char()`: Returns space
- Tokenization: Space is not a valid token start, so we'd get an error or skip it

**Actually, the real issue might be**: After `skip_comment_curly()`, we're at the space. But then we check `string_continuation`, and if it's incorrectly set, we might process the `}` that comes after the space.

**The fix**: We need to ensure that:
1. `string_continuation` is ONLY set when in a string
2. After comment skipping, we should skip whitespace again (or ensure we're at a valid token)
3. The brace depth check should only happen when `string_continuation` is actually set

## Next Steps

1. **Fix Comment Interference**: 
   - Ensure `string_continuation` is only checked when actually in a string context
   - Add assertion/logging to verify `string_continuation` is `None` when not in string
   - After comment skipping, skip whitespace again to ensure we're at a valid token
   
2. **State Management**: 
   - Add debug logging to track when `string_continuation` is set/cleared
   - Verify it's cleared when string ends
   - Verify it's never set outside string context
   
3. **Testing**: 
   - Add test to verify `string_continuation` is `None` after comment
   - Add test for comment followed by program keyword
   - Add comprehensive tests for all scenarios
   
4. **Semantic Analysis**: Type-check expressions in interpolations
5. **IR Generation**: Generate code to format strings at runtime
6. **Runtime**: Implement string formatting functions

