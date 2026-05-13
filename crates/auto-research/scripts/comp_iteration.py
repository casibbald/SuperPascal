#!/usr/bin/env python3
"""
SuperPascal compiler iteration — runs tests and reports pass/fail counts
with a summary of what was last committed. Useful as data-collection context
for the LLM agent loop.
"""
import subprocess, datetime, os

CR = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                  '..', '..')
os.chdir(CR)

# 1) Test results
r = subprocess.run(['cargo', 'test', '--workspace'],
                   capture_output=True, text=True, timeout=600,
                   cwd=CR)
total = failed = passed = 0
for line in r.stdout.splitlines():
    line = line.strip()
    if line.startswith('test result:'):
        # e.g. "test result: ok. 24 passed; 0 failed; ..."
        parts = line.split()
        for i, p in enumerate(parts):
            if p == 'passed' and i + 1 < len(parts):
                try:
                    passed += int(parts[i - 1])
                except ValueError:
                    pass
            if p == 'failed' and i + 1 < len(parts):
                try:
                    failed += int(parts[i - 1])
                except ValueError:
                    pass
    if line.startswith('test result:'):
        total += passed + failed
        passed = failed = 0  # reset for each crate

# 2) Last commit
gc = subprocess.run(['git', 'log', '-1', '--oneline'],
                    capture_output=True, text=True, cwd=CR)

now = datetime.datetime.now().strftime('%Y-%m-%d %H:%M')
print(f'### SuperPascal Compiler Iteration — {now}')
print(f'Last commit: {gc.stdout.strip()}')
print(f'Tests: {passed} passed, {failed} failed (total {total})')
if failed > 0:
    print('**FAILURES DETECTED — must fix before proceeding.**')
    for line in r.stderr.splitlines():
        if 'error[' in line.lower() or 'failed' in line.lower():
            print(f'  {line}')
else:
    print('All tests passing. Ready to proceed.')
