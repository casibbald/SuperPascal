# SuperPascal Sorting Library

**Status:** ✅ Complete  
**Priority:** ⭐⭐⭐ MEDIUM  
**Target:** All platforms (8-bit through 64-bit)

---

## Overview

Efficient sorting algorithms for organizing data. These algorithms are **generic** and work on **all platforms**.

**Key Features:**
- **Quicksort:** Fast average case, divide and conquer
- **Shellsort:** Simple, good for small arrays, no recursion
- **Mergesort:** Guaranteed O(n log n), stable
- **Heapsort:** Guaranteed O(n log n), in-place, no recursion
- **Custom Comparison:** All algorithms support custom comparison functions
- **Platform-Agnostic:** Works on all platforms without modification

---

## Module Structure

```
lib/sorting/
├── mod.pas          # Main entry point
├── types.pas        # Core types (TCompareFunction, CompareInt)
├── quicksort.pas    # Quicksort algorithm
├── shellsort.pas    # Shellsort algorithm
├── mergesort.pas    # Mergesort algorithm
├── heapsort.pas     # Heapsort algorithm
└── README.md        # This file
```

---

## Quick Start

### Basic Usage

```pascal
uses Sorting;

var
  data: array[0..99] of integer;
  i: integer;
begin
  // Fill array with random data
  for i := 0 to 99 do
    data[i] := Random(1000);
  
  // Sort using quicksort
  Quicksort(0, 99, data);
  
  // Array is now sorted
end.
```

### Custom Comparison

```pascal
uses Sorting;

// Custom comparison: sort in descending order
function CompareDescending(a, b: integer): integer;
begin
  if a > b then
    Result := -1
  else if a < b then
    Result := 1
  else
    Result := 0;
end;

var
  data: array[0..99] of integer;
begin
  // Fill array...
  
  // Sort in descending order
  QuicksortCustom(0, 99, data, @CompareDescending);
end.
```

---

## Algorithm Comparison

| Algorithm | Average Time | Worst Time | Space | In-Place | Stable | Best For |
|-----------|-------------|------------|-------|----------|--------|----------|
| **Quicksort** | O(n log n) | O(n²) | O(log n) | Yes | No | General-purpose, fast average |
| **Shellsort** | O(n^1.5) | O(n^1.5) | O(1) | Yes | No | Small arrays, no recursion |
| **Mergesort** | O(n log n) | O(n log n) | O(n) | No | Yes | Guaranteed performance, stability |
| **Heapsort** | O(n log n) | O(n log n) | O(1) | Yes | No | Memory-constrained, no recursion |

---

## API Reference

### Types

```pascal
type
  TCompareFunction = function(a, b: Integer): Integer;
```

**Returns:**
- `< 0` if `a < b`
- `0` if `a = b`
- `> 0` if `a > b`

### Functions

#### Quicksort

```pascal
procedure Quicksort(left, right: integer; var arr: array of integer);
procedure QuicksortCustom(left, right: integer; var arr: array of integer; compare: TCompareFunction);
```

**Parameters:**
- `left`: Starting index (inclusive)
- `right`: Ending index (inclusive)
- `arr`: Array to sort
- `compare`: Optional custom comparison function

**Complexity:** Average O(n log n), Worst O(n²)

#### Shellsort

```pascal
procedure Shellsort(n: integer; var arr: array of integer);
procedure ShellsortCustom(n: integer; var arr: array of integer; compare: TCompareFunction);
```

**Parameters:**
- `n`: Number of elements to sort
- `arr`: Array to sort (must have at least `n` elements)
- `compare`: Optional custom comparison function

**Complexity:** O(n^1.5)

#### Mergesort

```pascal
procedure Mergesort(left, right: integer; var arr: array of integer);
procedure MergesortCustom(left, right: integer; var arr: array of integer; compare: TCompareFunction);
```

**Parameters:**
- `left`: Starting index (inclusive)
- `right`: Ending index (inclusive)
- `arr`: Array to sort
- `compare`: Optional custom comparison function

**Complexity:** O(n log n) guaranteed, Stable

#### Heapsort

```pascal
procedure Heapsort(n: integer; var arr: array of integer);
procedure HeapsortCustom(n: integer; var arr: array of integer; compare: TCompareFunction);
```

**Parameters:**
- `n`: Number of elements to sort
- `arr`: Array to sort (must have at least `n` elements)
- `compare`: Optional custom comparison function

**Complexity:** O(n log n) guaranteed, In-place

---

## Usage Examples

### Example 1: Sort Integer Array

```pascal
uses Sorting;

var
  numbers: array[0..9] of integer = (5, 2, 8, 1, 9, 3, 7, 4, 6, 0);
begin
  Quicksort(0, 9, numbers);
  // numbers is now: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
end.
```

### Example 2: Sort with Custom Comparison

```pascal
uses Sorting;

type
  TPerson = record
    Name: string;
    Age: integer;
  end;

var
  people: array[0..4] of TPerson;
  
function CompareByAge(a, b: integer): integer;
begin
  // Compare ages (assuming people array indices)
  if people[a].Age < people[b].Age then
    Result := -1
  else if people[a].Age > people[b].Age then
    Result := 1
  else
    Result := 0;
end;

begin
  // Fill people array...
  
  // Sort by age (using indices)
  // Note: This is a simplified example - in practice, you'd sort the records directly
end.
```

### Example 3: Choose Algorithm Based on Array Size

```pascal
uses Sorting;

var
  data: array of integer;
  n: integer;
begin
  n := Length(data);
  
  if n < 10 then
    // Small array: use shellsort
    Shellsort(n, data)
  else if n < 1000 then
    // Medium array: use quicksort
    Quicksort(0, n - 1, data)
  else
    // Large array: use mergesort for guaranteed performance
    Mergesort(0, n - 1, data);
end.
```

---

## Platform Considerations

### 8-Bit Systems (Z80, 65C02)

- **Avoid Quicksort/Mergesort:** Recursion may cause stack overflow
- **Prefer Shellsort/Heapsort:** No recursion, constant space
- **Memory:** Mergesort requires O(n) extra memory (may be limited)

### 16-Bit Systems (65C816)

- **Quicksort:** Generally safe, but watch recursion depth
- **Mergesort:** Requires temporary array (may be limited)
- **Heapsort:** Good choice (no recursion, in-place)

### 32-Bit Systems (MC68LC060)

- **All algorithms:** Generally safe
- **Quicksort:** Fastest for most cases
- **Mergesort:** Use when stability is required

### 64-Bit Systems (ARM64)

- **All algorithms:** Fully supported
- **Quicksort:** Best average performance
- **Mergesort:** Best for large datasets with stability requirement

---

## Performance Notes

### Quicksort
- Very fast on average
- Worst case occurs when pivot is always smallest/largest
- Can be optimized with median-of-three pivot
- For small arrays (n < 10), insertion sort may be faster

### Shellsort
- Simple to implement
- Good for small to medium arrays
- Gap sequence affects performance (this implementation uses n/2, n/4, ...)
- Better gap sequences exist (e.g., Knuth's: 1, 4, 13, 40, 121, ...)

### Mergesort
- Guaranteed O(n log n) performance
- Stable (preserves order of equal elements)
- Requires O(n) extra memory
- Good for linked lists (can be done in-place)

### Heapsort
- Guaranteed O(n log n) performance
- In-place (no extra memory needed)
- Not stable (may change order of equal elements)
- Slower than quicksort in practice due to poor cache locality

---

## Choosing the Right Algorithm

**For Small Arrays (n < 10):**
- Use **Shellsort** (simple, fast for small n)

**For Medium Arrays (10 < n < 1000):**
- Use **Quicksort** (fast average case)
- Or **Shellsort** (simple, no recursion)

**For Large Arrays (n > 1000):**
- Use **Quicksort** with median-of-three pivot
- Or **Mergesort** if stability is required
- Or **Heapsort** if memory is constrained

**For Embedded Systems:**
- Prefer **Shellsort** or **Heapsort** (no recursion)
- Avoid **Mergesort** (requires extra memory)

**For Stable Sorting:**
- Use **Mergesort** (only stable O(n log n) algorithm)

---

## Source Material

**Source:** `algorithms/03_SortingAlgorithms.md`  
**Original:** `docs/mikro_docs_archive/Coding/1/SORT_ALG.TXT`

---

**Last Updated:** 2025-01-XX  
**Status:** ✅ Complete (5 modules)

