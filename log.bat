set count=0
:loop
set /a count=%count%+1
echo -%count%- >> test.log
if %count% neq 10000 goto loop