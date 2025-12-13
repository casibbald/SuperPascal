unit Sorting;

interface

// Sorting algorithms library
// Source: algorithms/03_SortingAlgorithms.md
//
// Provides efficient sorting algorithms for organizing data:
// - Quicksort: Fast average case, divide and conquer
// - Shellsort: Simple, good for small arrays
// - Mergesort: Guaranteed O(n log n), stable
// - Heapsort: Guaranteed O(n log n), in-place

uses
  Sorting_Types,
  Sorting_Quicksort,
  Sorting_Shellsort,
  Sorting_Mergesort,
  Sorting_Heapsort;

// Re-export types
type
  TCompareFunction = Sorting_Types.TCompareFunction;

// Re-export comparison function
function CompareInt(a, b: Integer): Integer;

// Re-export sorting functions
// Quicksort
procedure Quicksort(left, right: integer; var arr: array of integer);
procedure QuicksortCustom(left, right: integer; var arr: array of integer; compare: TCompareFunction);

// Shellsort
procedure Shellsort(n: integer; var arr: array of integer);
procedure ShellsortCustom(n: integer; var arr: array of integer; compare: TCompareFunction);

// Mergesort
procedure Mergesort(left, right: integer; var arr: array of integer);
procedure MergesortCustom(left, right: integer; var arr: array of integer; compare: TCompareFunction);

// Heapsort
procedure Heapsort(n: integer; var arr: array of integer);
procedure HeapsortCustom(n: integer; var arr: array of integer; compare: TCompareFunction);

implementation

// Re-export implementations
function CompareInt(a, b: Integer): Integer; begin Result := Sorting_Types.CompareInt(a, b); end;

procedure Quicksort(left, right: integer; var arr: array of integer); begin Sorting_Quicksort.Quicksort(left, right, arr); end;
procedure QuicksortCustom(left, right: integer; var arr: array of integer; compare: TCompareFunction); begin Sorting_Quicksort.QuicksortCustom(left, right, arr, compare); end;

procedure Shellsort(n: integer; var arr: array of integer); begin Sorting_Shellsort.Shellsort(n, arr); end;
procedure ShellsortCustom(n: integer; var arr: array of integer; compare: TCompareFunction); begin Sorting_Shellsort.ShellsortCustom(n, arr, compare); end;

procedure Mergesort(left, right: integer; var arr: array of integer); begin Sorting_Mergesort.Mergesort(left, right, arr); end;
procedure MergesortCustom(left, right: integer; var arr: array of integer; compare: TCompareFunction); begin Sorting_Mergesort.MergesortCustom(left, right, arr, compare); end;

procedure Heapsort(n: integer; var arr: array of integer); begin Sorting_Heapsort.Heapsort(n, arr); end;
procedure HeapsortCustom(n: integer; var arr: array of integer; compare: TCompareFunction); begin Sorting_Heapsort.HeapsortCustom(n, arr, compare); end;

end.

