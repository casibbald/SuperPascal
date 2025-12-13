unit Sorting_Quicksort;

interface

// Quicksort algorithm
// Source: algorithms/03_SortingAlgorithms.md
//
// Divide and conquer sorting algorithm using a pivot element
// Average: O(n log n), Worst: O(n²), Space: O(log n), In-Place: Yes

// Sort array of integers using quicksort
procedure Quicksort(left, right: integer; var arr: array of integer);

// Sort array with custom comparison function
procedure QuicksortCustom(left, right: integer; var arr: array of integer; compare: function(a, b: integer): integer);

implementation

uses
  Sorting_Types;

// Helper: Swap two integers
procedure Swap(var a, b: integer);
var
  temp: integer;
begin
  temp := a;
  a := b;
  b := temp;
end;

// Quicksort with default integer comparison
procedure Quicksort(left, right: integer; var arr: array of integer);
begin
  QuicksortCustom(left, right, arr, @Sorting_Types.CompareInt);
end;

// Quicksort with custom comparison function
procedure QuicksortCustom(left, right: integer; var arr: array of integer; compare: function(a, b: integer): integer);
var
  m, i, j: integer;
begin
  i := left;
  j := right;
  m := arr[(left + right) div 2];  // Pivot: middle element
  
  repeat
    while compare(arr[i], m) < 0 do
      i := i + 1;
    while compare(arr[j], m) > 0 do
      j := j - 1;
    
    if i <= j then
    begin
      Swap(arr[i], arr[j]);
      i := i + 1;
      j := j - 1;
    end;
  until i > j;
  
  if left < j then
    QuicksortCustom(left, j, arr, compare);
  if i < right then
    QuicksortCustom(i, right, arr, compare);
end;

end.

