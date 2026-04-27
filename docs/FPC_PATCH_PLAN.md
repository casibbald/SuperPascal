# FreePascal Patch Plan for Token Dumping

## Goal

Patch FreePascal compiler to add `-dt` (dump tokens) option that outputs tokens in a parseable format for comparison with SuperPascal tokenizer.

## Changes Required

### 1. Add Global Flag for Token Dumping

**File:** `freePascal/compiler/globals.pas`

Add a new global variable:
```pascal
var
  para_dump_tokens: boolean = false;
```

### 2. Add Command-Line Option Handler

**File:** `freePascal/compiler/options.pas`

In `Interpret_D_l` procedure, add:
```pascal
'dt' :
  begin
    para_dump_tokens:=true;
  end;
```

Or add a new `Interpret_D_U` handler for `-DT` or `--dump-tokens`.

### 3. Modify Scanner to Output Tokens

**File:** `freePascal/compiler/scanner.pas`

In `tscannerfile.readtoken` procedure, after token is determined (at `exit_label:`), add:
```pascal
exit_label:
  lasttoken:=token;
  
  { Dump token if requested }
  if para_dump_tokens then
    begin
      writeln(TokenKindToString(token), ' [', pattern, '] (', line_no, ':', 
              inputpointer - inputstart, ')');
    end;
end;
```

We'll need a helper function to convert `ttoken` to string.

### 4. Add Token Name Helper Function

**File:** `freePascal/compiler/scanner.pas` or `tokens.pas`

Add function to convert token enum to string:
```pascal
function TokenKindToString(t: ttoken): string;
begin
  case t of
    NOTOKEN: Result := 'NOTOKEN';
    _ID: Result := 'ID';
    _INTCONST: Result := 'INTCONST';
    _REALNUMBER: Result := 'REALNUMBER';
    _CSTRING: Result := 'CSTRING';
    // ... etc
  end;
end;
```

## Implementation Steps

1. **Uninstall brew FPC** ✓
2. **Add global flag** in `globals.pas`
3. **Add option handler** in `options.pas`
4. **Add token dump logic** in `scanner.pas`
5. **Add token name function** in `tokens.pas` or `scanner.pas`
6. **Build FPC from source**
7. **Test token dumping**
8. **Update scripts to use patched FPC**

## Build Instructions

```bash
cd freePascal
make build
sudo make install INSTALL_PREFIX=/usr/local
```

## Usage

```bash
fpc -dt input.pas
```

Output format:
```
PROGRAM [] (1:1)
ID [HelloWorld] (1:9)
SEMICOLON [] (1:19)
BEGIN [] (3:1)
...
EOF [] (5:5)
```

