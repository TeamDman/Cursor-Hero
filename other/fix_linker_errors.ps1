Write-Host -ForegroundColor Red "This script doesn't fix the problem FYI."

Write-Host "Performing cargo build..."
$cargo_output = cargo build 2>&1

# Check if build was successful
if ($? -eq $true) {
    Write-Host "Cargo build successful"
    exit 0
} else {
    Write-Host "Build failed, examining errors..."
}

$broken = $cargo_output | rg --only-matching "\bcursor_hero_\w*\b" | Sort-Object -Unique

if ($broken.Length -eq 0) {
    Write-Host "No specific broken target artifacts identified."
    exit 1
}

Write-Host "Broken target artifacts: $($broken -join ', ')"
Write-Host "Proceed with removal? (y/n)"
$proceed = Read-Host

if ($proceed -ne "y") {
    Write-Host "Exiting..."
    exit 1
}


foreach ($pattern in $broken) {
    Write-Host "Removing target files with pattern: $pattern"
    Get-ChildItem .\target\debug\deps\ | % { $_.FullName } | rg $pattern | Remove-Item
}