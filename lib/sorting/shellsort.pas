unit Sorting_Shellsort;

interface

// Shellsort algorithm
// Source: algorithms/03_SortingAlgorithms.md
//
// Improved insertion sort with decreasing gap sizes
// Average: O(n^1.5), Worst: O(n^1.5), Space: O(1), In-Place: Yes

// Sort array of integers using shellsort
procedure Shellsort(n: integer; var arr: array of integer);

// Sort array with custom comparison function
procedure ShellsortCustom(n: integer; var arr: array of integer; compare: function(a, b: integer): integer);

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

// Shellsort with default integer comparison
procedure Shellsort(n: integer; var arr: array of integer);
begin
  ShellsortCustom(n, arr, @Sorting_Types.CompareInt);
end;

// Shellsort with custom comparison function
procedure ShellsortCustom(n: integer; var arr: array of integer; compare: function(a, b: integer): integer);
var
  d, i, j: integer;
  flag: boolean;
begin
  d := n div 2;
  
  while d >= 1 do
  begin
    for i := 0 to n - d - 1 do
    begin
      j := i;
      flag := true;
      
      while flag do
      begin
        flag := false;
        if j >= 0 then
        begin
          if compare(arr[j], arr[j + d]) > 0 then
          begin
            Swap(arr[j], arr[j + d]);
            flag := true;
            j := j - d;
          end;
        end;
      end;
    end;
    
    d := d div 2;
  end;
end;

end.

