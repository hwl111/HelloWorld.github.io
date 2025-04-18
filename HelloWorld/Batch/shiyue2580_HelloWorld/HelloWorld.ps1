<#
.SYNOPSIS
标准化的 PowerShell Hello World 脚本
.DESCRIPTION
作者: Other
日期: 0000-00-00
#>

try {
    Write-Output "Hello World from PowerShell!"
    # 模拟错误（测试用）
    # Throw "模拟错误"
} catch {
    Write-Host "[ERROR] $_" -ForegroundColor Red
    exit 1
}

exit 0