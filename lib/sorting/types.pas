unit Sorting_Types;

interface

// Core types for sorting algorithms
// Source: algorithms/03_SortingAlgorithms.md

// Comparison function type
// Returns: < 0 if a < b, 0 if a = b, > 0 if a > b
type
  TCompareFunction = function(a, b: Integer): Integer;

// Default integer comparison function
function CompareInt(a, b: Integer): Integer;

implementation

// Default integer comparison
function CompareInt(a, b: Integer): Integer;
begin
  if a < b then
    Result := -1
  else if a > b then
    Result := 1
  else
    Result := 0;
end;

end.

