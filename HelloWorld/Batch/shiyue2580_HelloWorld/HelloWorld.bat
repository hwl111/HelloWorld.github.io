@echo off
REM ==================================================
REM 文件名: HelloWorld.bat
REM 描述: 标准化的 Windows BAT 示例 - 输出 Hello World
REM 作者: Other
REM 日期: 0000-00-00
REM ==================================================

echo Hello World from BAT!

REM 检查错误（示例）
if %errorlevel% neq 0 (
    echo [ERROR] 脚本执行失败，错误码: %errorlevel%
    exit /b %errorlevel%
)

pause