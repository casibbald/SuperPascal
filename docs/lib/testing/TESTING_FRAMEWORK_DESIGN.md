# SuperPascal Unit Testing Framework Design

**Status:** 🎨 Design Phase  
**Priority:** ⭐⭐⭐⭐⭐ CRITICAL  
**Target:** All platforms (8-bit through 64-bit)

---

## Overview

A comprehensive unit testing framework for SuperPascal, designed from modern testing principles but adapted for retro computing platforms. While unit testing wasn't common in the retro computing era, we can leverage all modern knowledge to build an excellent testing framework.

**Key Principles:**
- **Platform-Agnostic:** Works on all platforms without modification
- **No Dependencies:** Standalone framework (no external dependencies)
- **Simple API:** Easy to use, follows modern testing patterns
- **Rich Assertions:** Comprehensive assertion library
- **Clear Reporting:** Human-readable test results
- **Memory Efficient:** Suitable for memory-constrained platforms

---

## Architecture

### Core Components

1. **Test Types** - Test cases, test suites, test results
2. **Assertions** - Assert functions for validation
3. **Test Runner** - Execution engine
4. **Fixtures** - Setup/teardown support
5. **Reporting** - Test result output

### Module Structure

```
lib/testing/
├── mod.pas          # Main entry point
├── types.pas        # Core types (TestCase, TestSuite, TestResult)
├── assertions.pas   # Assert functions
├── runner.pas       # Test runner and execution
├── fixtures.pas     # Setup/teardown support
├── reporting.pas    # Test result reporting
└── README.md        # Documentation
```

---

## Core Types

### TestResult

```pascal
type
  TTestStatus = (tsPassed, tsFailed, tsSkipped, tsError);
  
  TTestResult = record
    TestName: string;
    SuiteName: string;
    Status: TTestStatus;
    Message: string;
    Duration: integer;  // Milliseconds (if timing available)
  end;
  
  TTestResults = array of TTestResult;
```

### TestCase

```pascal
type
  TTestProcedure = procedure;
  
  TTestCase = record
    Name: string;
    SuiteName: string;
    TestProc: TTestProcedure;
    SetupProc: TTestProcedure;      // Optional
    TeardownProc: TTestProcedure;    // Optional
    Skip: boolean;                   // Skip this test
    ExpectedException: string;       // Expected exception type
  end;
```

### TestSuite

```pascal
type
  TTestSuite = record
    Name: string;
    Tests: array of TTestCase;
    Count: integer;
    SetupProc: TTestProcedure;      // Suite-level setup
    TeardownProc: TTestProcedure;   // Suite-level teardown
  end;
```

---

## Assertions

### Basic Assertions

```pascal
// Boolean assertions
procedure AssertTrue(condition: boolean; const message: string = '');
procedure AssertFalse(condition: boolean; const message: string = '');

// Equality assertions
procedure AssertEqual(expected, actual: integer; const message: string = '');
procedure AssertEqual(expected, actual: string; const message: string = '');
procedure AssertEqual(expected, actual: Fixed16; const message: string = '');
procedure AssertNotEqual(expected, actual: integer; const message: string = '');

// Nil assertions
procedure AssertNil(ptr: Pointer; const message: string = '');
procedure AssertNotNil(ptr: Pointer; const message: string = '');

// Comparison assertions
procedure AssertGreaterThan(value, threshold: integer; const message: string = '');
procedure AssertLessThan(value, threshold: integer; const message: string = '');
procedure AssertGreaterThanOrEqual(value, threshold: integer; const message: string = '');
procedure AssertLessThanOrEqual(value, threshold: integer; const message: string = '');
```

### Collection Assertions

```pascal
// Array assertions
procedure AssertArrayEqual(
  const expected, actual: array of integer;
  const message: string = ''
);

// String assertions
procedure AssertStringEqual(
  const expected, actual: string;
  const message: string = ''
);
procedure AssertStringContains(
  const haystack, needle: string;
  const message: string = ''
);
```

### Exception Assertions

```pascal
// Exception handling
procedure AssertRaises(
  testProc: TTestProcedure;
  const expectedException: string;
  const message: string = ''
);
```

### Floating-Point Assertions (Fixed-Point)

```pascal
// Fixed-point assertions (with tolerance)
procedure AssertEqual(
  expected, actual: Fixed16;
  tolerance: Fixed16;
  const message: string = ''
);
```

---

## Test Registration

### Manual Registration

```pascal
// Register a single test
procedure RegisterTest(
  const suiteName, testName: string;
  testProc: TTestProcedure
);

// Register test with setup/teardown
procedure RegisterTestWithFixtures(
  const suiteName, testName: string;
  testProc, setupProc, teardownProc: TTestProcedure
);

// Register a test suite
procedure RegisterSuite(const suiteName: string; tests: array of TTestCase);
```

### Automatic Discovery (Future)

```pascal
// Auto-discover tests with naming convention
// Tests must be named: Test_* or *_Test
procedure DiscoverTests;
```

---

## Test Execution

### Running Tests

```pascal
// Run a single test
function RunTest(const suiteName, testName: string): TTestResult;

// Run all tests in a suite
function RunSuite(const suiteName: string): TTestResults;

// Run all registered tests
function RunAllTests: TTestResults;

// Run tests matching pattern
function RunTestsMatching(const pattern: string): TTestResults;
```

### Test Runner Options

```pascal
type
  TTestRunnerOptions = record
    StopOnFirstFailure: boolean;
    Verbose: boolean;
    ShowTiming: boolean;
    FilterSuite: string;      // Run only this suite
    FilterTest: string;       // Run only this test
  end;

procedure SetRunnerOptions(const options: TTestRunnerOptions);
```

---

## Fixtures

### Setup/Teardown

```pascal
// Suite-level fixtures
procedure SetSuiteSetup(const suiteName: string; setupProc: TTestProcedure);
procedure SetSuiteTeardown(const suiteName: string; teardownProc: TTestProcedure);

// Test-level fixtures
procedure SetTestSetup(const suiteName, testName: string; setupProc: TTestProcedure);
procedure SetTestTeardown(const suiteName, testName: string; teardownProc: TTestProcedure);

// Global fixtures (run before/after all tests)
procedure SetGlobalSetup(setupProc: TTestProcedure);
procedure SetGlobalTeardown(teardownProc: TTestProcedure);
```

### BeforeEach/AfterEach

```pascal
// Run before/after each test in suite
procedure SetBeforeEach(const suiteName: string; proc: TTestProcedure);
procedure SetAfterEach(const suiteName: string; proc: TTestProcedure);
```

---

## Reporting

### Console Output

```pascal
// Print test results to console
procedure PrintTestResults(const results: TTestResults);

// Print summary statistics
procedure PrintTestSummary(const results: TTestResults);

// Print detailed failure information
procedure PrintFailureDetails(const result: TTestResult);
```

### Report Format

```
Running tests...
========================================
Suite: Math
  Test: AdditionTest ................. PASSED
  Test: SubtractionTest .............. PASSED
  Test: MultiplicationTest ........... FAILED: Expected 6, got 5
Suite: Graphics
  Test: LineDrawingTest .............. PASSED
  Test: CircleDrawingTest ............ PASSED

========================================
Summary:
  Total: 5
  Passed: 4
  Failed: 1
  Skipped: 0
  Duration: 42ms
========================================
```

---

## Usage Examples

### Basic Test

```pascal
uses Testing;

procedure TestAddition;
begin
  AssertEqual(2 + 2, 4, 'Basic addition');
  AssertEqual(10 + 20, 30, 'Larger numbers');
end;

begin
  RegisterTest('Math', 'Addition', @TestAddition);
  RunAllTests;
end.
```

### Test with Setup/Teardown

```pascal
uses Testing;

var
  TestData: integer;

procedure SetupMathTest;
begin
  TestData := 10;
end;

procedure TeardownMathTest;
begin
  TestData := 0;
end;

procedure TestWithData;
begin
  AssertEqual(TestData * 2, 20, 'TestData should be 10');
end;

begin
  RegisterTestWithFixtures(
    'Math',
    'TestWithData',
    @TestWithData,
    @SetupMathTest,
    @TeardownMathTest
  );
  RunAllTests;
end.
```

### Test Suite

```pascal
uses Testing;

procedure TestAdd;
begin
  AssertEqual(1 + 1, 2);
end;

procedure TestSubtract;
begin
  AssertEqual(5 - 3, 2);
end;

procedure TestMultiply;
begin
  AssertEqual(3 * 4, 12);
end;

var
  suite: TTestSuite;
begin
  suite.Name := 'Math';
  SetLength(suite.Tests, 3);
  suite.Tests[0] := (Name: 'Add'; SuiteName: 'Math'; TestProc: @TestAdd);
  suite.Tests[1] := (Name: 'Subtract'; SuiteName: 'Math'; TestProc: @TestSubtract);
  suite.Tests[2] := (Name: 'Multiply'; SuiteName: 'Math'; TestProc: @TestMultiply);
  suite.Count := 3;
  
  RegisterSuite('Math', suite.Tests);
  RunSuite('Math');
end.
```

### Fixed-Point Assertions

```pascal
uses Testing, Math;

procedure TestFixedPoint;
var
  a, b: Fixed16;
begin
  a := IntToFixed16(2);
  b := IntToFixed16(2);
  AssertEqual(a, b, 'Fixed16 equality');
  
  a := Fixed16Mul(IntToFixed16(2), IntToFixed16(3));
  b := IntToFixed16(6);
  AssertEqual(a, b, FIXED16_ONE div 256, 'Fixed16 multiplication');  // Tolerance
end;
```

---

## Platform Considerations

### Memory Constraints

**8-bit Platforms (ZealZ80, CommanderX16):**
- Use static arrays where possible
- Limit test result history
- Simple string handling (avoid dynamic strings if possible)

**16-bit+ Platforms:**
- Can use dynamic arrays
- More detailed reporting
- Timing information (if available)

### String Handling

**Challenge:** Pascal strings vary by platform
- **Solution:** Use `ShortString` for compatibility
- **Alternative:** Use integer IDs for test names (more memory-efficient)

### Timing

**Challenge:** Not all platforms have high-resolution timers
- **Solution:** Make timing optional
- **Fallback:** Count test executions only

---

## Implementation Phases

### Phase 1: Core Framework
- [ ] Types and structures
- [ ] Basic assertions (True, False, Equal)
- [ ] Test registration
- [ ] Simple test runner

### Phase 2: Advanced Features
- [ ] Setup/teardown support
- [ ] Test suites
- [ ] Exception handling
- [ ] Reporting system

### Phase 3: Enhancements
- [ ] Test filtering
- [ ] Test discovery
- [ ] Mocking support (basic)
- [ ] Performance profiling

---

## Design Decisions

### 1. Procedure Pointers vs. Strings

**Decision:** Use procedure pointers for test functions
- **Pros:** Type-safe, fast execution
- **Cons:** Requires explicit registration

**Alternative:** String-based test discovery (future enhancement)

### 2. String Types

**Decision:** Use `ShortString` for maximum compatibility
- **Pros:** Works on all platforms
- **Cons:** Limited length (255 characters)

**Alternative:** Platform-specific string types (conditional compilation)

### 3. Test Result Storage

**Decision:** Dynamic arrays for test results
- **Pros:** Flexible, can store all results
- **Cons:** May use more memory on 8-bit platforms

**Alternative:** Static arrays with fixed size (8-bit optimization)

### 4. Assertion Failure Handling

**Decision:** Continue execution, collect all failures
- **Pros:** See all test failures at once
- **Cons:** May take longer on failures

**Alternative:** Stop on first failure (configurable option)

---

## Future Enhancements

1. **Test Discovery:** Automatic test discovery via naming convention
2. **Mocking:** Basic mocking framework for dependencies
3. **Parametrized Tests:** Run same test with different parameters
4. **Test Coverage:** Basic code coverage tracking
5. **Parallel Execution:** Run tests in parallel (for multi-core platforms)
6. **XML/JSON Output:** Export test results in structured formats
7. **Continuous Integration:** CI-friendly output formats

---

## References

**Modern Testing Frameworks:**
- JUnit (Java)
- pytest (Python)
- Jest (JavaScript)
- Go testing package
- Rust testing framework

**Key Concepts:**
- Arrange-Act-Assert (AAA) pattern
- Test fixtures and setup/teardown
- Test isolation
- Assertion libraries
- Test organization (suites, groups)

---

**Last Updated:** 2025-01-XX  
**Status:** 🎨 Design Phase - Ready for implementation

