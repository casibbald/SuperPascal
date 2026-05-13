#!/usr/bin/env python3
"""
SuperPascal compiler iteration — runs tests and reports pass/fail counts
with a summary of what was last committed. Useful as data-collection context
for the LLM agent loop.
"""
import subprocess, datetime, os, re

CR = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                  '..', '..')
os.chdir(CR)

# 1) Test results
r = subprocess.run(['cargo', 'test', '--workspace'],
                   capture_output=True, text=True, timeout=600,
                   cwd=CR)
total = failed = passed = 0
for line in r.stdout.splitlines():
    m = re.search(r'test result:\s+(\w+)\.\s+(\d+)\s+passed;\s+(\d+)\s+failed', line)
    if m:
        status = m.group(1)
        passed += int(m.group(2))
        failed += int(m.group(3))

total = passed + failed

# 2) Last commit
gc = subprocess.run(['git', 'log', '-1', '--oneline'],
                    capture_output=True, text=True, cwd=CR)

now = datetime.datetime.now().strftime('%Y-%m-%d %H:%M')
print(f'### SuperPascal Compiler Iteration — {now}')
print(f'Last commit: {gc.stdout.strip()}')
print(f'Tests: {passed} passed, {failed} failed (total {total})')
if failed > 0:
    print('**FAILURES DETECTED — must fix before proceeding.**')
else:
    print('All tests passing. Ready to proceed.')
