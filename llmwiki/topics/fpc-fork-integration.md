# FreePascal Fork Integration

- Status: partially-verified
- Source docs: `CONTRIBUTING.md`, `docs/FPC_PATCH_SUMMARY.md`, `docs/FPC_INTEGRATION_SUMMARY.md`
- Code anchors: `../freePascal/`, `tests/compliance/`

## Why the fork exists

SuperPascal needs token-level comparison with FreePascal for compliance testing. Upstream FPC doesn't provide a parseable token dump API.

## The jerryrig

A forked FreePascal compiler with minimal patches:
1. `globals.pas` — added `para_dump_tokens: boolean;`
2. `options.pas` — added `-dt` command-line flag
3. `tokens.pas` — added `TokenKindToString(t: ttoken): string;`
4. `scanner.pas` — modified `readtoken` to output tokens when flag is set

## Output format

```
TOKEN_KIND [pattern] (line:column)
PROGRAM [] (1:1)
identifier [HELLOWORLD] (1:9)
; [] (1:19)
...
```

## Usage

```bash
# Generate FPC tokens for a test file
cd crates/compiler-rs/tests/compliance
fpc -dt test_cases/simple_program.pas > fpc_tokens.txt

# Compare with SuperPascal (when tokenizer is complete)
cargo run --bin tokenizer test_cases/simple_program.pas > sp_tokens.txt
diff fpc_tokens.txt sp_tokens.txt
```

## CI integration

- Patched FPC binaries are built and released at `https://github.com/casibbald/freePascal/releases`
- Linux x86_64 built on GitHub Actions
- macOS built locally
- CI uses setup-fpc action to download and install

## FPC as semantic oracle

Beyond tokenizer comparison, FPC serves as:
- Semantic reference (compiler directives, type compatibility)
- ABI reference (calling conventions, structure layout)
- Feature compatibility baseline

## Maintenance

When modifying FPC patches:
1. Edit `../freePascal/compiler/`
2. Build: `cd ../freePascal && make build`
3. Test: `./ppca64 -dt test.pas`
4. Commit to `freePascal` repo separately
5. Push to trigger CI
6. Update SuperPascal to use new release
