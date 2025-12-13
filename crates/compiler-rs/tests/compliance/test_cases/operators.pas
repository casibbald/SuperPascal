program TestOperators;

var
  a, b, c: integer;
  flag: boolean;

begin
  a := 5 + 3;
  b := 10 - 2;
  c := 4 * 2;
  a := 16 div 2;
  b := 15 mod 4;
  
  flag := a = b;
  flag := a <> b;
  flag := a < b;
  flag := a <= b;
  flag := a > b;
  flag := a >= b;
  
  flag := (a > 0) and (b > 0);
  flag := (a = 0) or (b = 0);
  flag := not (a = 0);
end.

