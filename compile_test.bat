@echo off
c:\users\administrator\desktop\rust中文版\mingw64\bin\gcc.exe -o compile_test_manual.exe temp_output.c 2>error.log 1>output.log
echo Exit code: %errorlevel%
type error.log
type output.log