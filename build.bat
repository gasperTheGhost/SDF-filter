@echo off
set arg1=%1
mkdir build
IF "%arg1%" == "" (
    FOR %%f IN (bin\*.dart) DO (
        dart2native %%f -o build\%%~nf.exe
    )
) ELSE (
    IF "%arg1%" == "all" (
        FOR %%f IN (bin\*.dart) DO (    
            dart2native %%f -o build\%%~nf.exe
        )
    ) ELSE (
        dart2native bin\%arg1%.dart -o build\%arg1%
    )
)
echo Done!
pause
