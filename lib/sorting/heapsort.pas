unit Sorting_Heapsort;

interface

// Heapsort algorithm
// Source: algorithms/03_SortingAlgorithms.md
//
// Build heap, repeatedly extract maximum
// Average: O(n log n), Worst: O(n log n), Space: O(1), In-Place: Yes

// Sort array of integers using heapsort
procedure Heapsort(n: integer; var arr: array of integer);

// Sort array with custom comparison function
procedure HeapsortCustom(n: integer; var arr: array of integer; compare: function(a, b: integer): integer);

implementation

uses
  Sorting_Types;

// Fix heap property
procedure FixHeap(lower, element, upper: integer; var arr: array of integer; compare: function(a, b: integer): integer);
var
  child: integer;
begin
  // Reestablish the heap property in the tree
  // The element to insert at position 'lower' is 'element'
  // Last index in heap is 'upper'
  // The heap property means that the element at index 'lower' should
  // be the largest in the subtree it represents
  
  while lower * 2 + 1 <= upper do
  begin
    child := lower * 2 + 1;  // Left child
    
    // Find larger child
    if (child < upper) and (compare(arr[child], arr[child + 1]) < 0) then
      child := child + 1;
    
    // If element is larger than largest child, heap property satisfied
    if compare(element, arr[child]) >= 0 then
      break;
    
    // Move larger child up
    arr[lower] := arr[child];
    lower := child;
  end;
  
  arr[lower] := element;
end;

// Heapsort with default integer comparison
procedure Heapsort(n: integer; var arr: array of integer);
begin
  HeapsortCustom(n, arr, @Sorting_Types.CompareInt);
end;

// Heapsort with custom comparison function
procedure HeapsortCustom(n: integer; var arr: array of integer; compare: function(a, b: integer): integer);
var
  i, heapsize, max: integer;
begin
  // Heap construction: build max-heap from bottom up
  for i := (n div 2) - 1 downto 0 do
    FixHeap(i, arr[i], n - 1, arr, compare);
  
  // Rearrange: repeatedly extract maximum
  for heapsize := n - 1 downto 1 do
  begin
    max := arr[0];
    FixHeap(0, arr[heapsize], heapsize - 1, arr, compare);
    arr[heapsize] := max;
  end;
end;

end.

