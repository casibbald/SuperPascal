#!/bin/sh
DoExitAsm ()
{ echo "An error occurred while assembling $1"; exit 1; }
DoExitLink ()
{ echo "An error occurred while linking $1"; exit 1; }
echo Linking test_cases/simple_program
OFS=$IFS
IFS="
"
/Library/Developer/CommandLineTools/usr/bin/ld        -x   -multiply_defined suppress -L. -o test_cases/simple_program `cat link63224.res` -filelist linkfiles63224.res
if [ $? != 0 ]; then DoExitLink test_cases/simple_program; fi
IFS=$OFS
