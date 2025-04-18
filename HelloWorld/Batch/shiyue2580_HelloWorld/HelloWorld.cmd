@echo off
SETLOCAL
REM ==================================================
REM 文件名: HelloWorld.cmd
REM 描述: 标准化的 Windows CMD 示例 - 输出 Hello World
REM 支持环境: Windows NT 及以上
REM ==================================================

set MESSAGE=Hello World from CMD!
echo %MESSAGE%

REM 强制退出代码为 0（除非明确失败）
exit /b 0