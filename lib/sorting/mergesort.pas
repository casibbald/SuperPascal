unit Sorting_Mergesort;

interface

// Mergesort algorithm
// Source: algorithms/03_SortingAlgorithms.md
//
// Divide array in half, sort each half, merge results
// Average: O(n log n), Worst: O(n log n), Space: O(n), In-Place: No, Stable: Yes

// Sort array of integers using mergesort
procedure Mergesort(left, right: integer; var arr: array of integer);

// Sort array with custom comparison function
procedure MergesortCustom(left, right: integer; var arr: array of integer; compare: function(a, b: integer): integer);

implementation

uses
  Sorting_Types;

// Merge two sorted halves
procedure Merge(left1, right1, left2, right2: integer; var arr: array of integer; compare: function(a, b: integer): integer);
var
  temp: array of integer;
  i, j, k: integer;
begin
  // Allocate temporary array
  SetLength(temp, right2 - left1 + 1);
  
  i := left1;
  j := left2;
  k := 0;
  
  // Merge both halves
  while (i <= right1) and (j <= right2) do
  begin
    if compare(arr[i], arr[j]) <= 0 then
    begin
      temp[k] := arr[i];
      i := i + 1;
    end
    else
    begin
      temp[k] := arr[j];
      j := j + 1;
    end;
    k := k + 1;
  end;
  
  // Copy remaining elements from first half
  while i <= right1 do
  begin
    temp[k] := arr[i];
    i := i + 1;
    k := k + 1;
  end;
  
  // Copy remaining elements from second half
  while j <= right2 do
  begin
    temp[k] := arr[j];
    j := j + 1;
    k := k + 1;
  end;
  
  // Copy back to original array
  for i := 0 to k - 1 do
    arr[left1 + i] := temp[i];
end;

// Mergesort with default integer comparison
procedure Mergesort(left, right: integer; var arr: array of integer);
begin
  MergesortCustom(left, right, arr, @Sorting_Types.CompareInt);
end;

// Mergesort with custom comparison function
procedure MergesortCustom(left, right: integer; var arr: array of integer; compare: function(a, b: integer): integer);
var
  middle: integer;
begin
  if left < right then
  begin
    middle := (left + right) div 2;
    MergesortCustom(left, middle, arr, compare);
    MergesortCustom(middle + 1, right, arr, compare);
    Merge(left, middle, middle + 1, right, arr, compare);
  end;
end;

end.

