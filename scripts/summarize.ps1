# assert we are in the parent of the scripts directory
if ((Get-Location).Path.EndsWith("scripts")) {
    Push-Location (Get-Location).Parent
} else {
    Push-Location $(Get-Location)
}
try {
    $choices = git ls-files --exclude-standard --others --cached `
        <# exclude images #> `
        | Where-Object { $_.EndsWith(".rs") -or $_.EndsWith(".toml") -or $_.EndsWith("md") }

    $files = @()
    while ($true) {
        $chosen = $choices | fzf --multi --bind "ctrl-a:select-all,ctrl-d:deselect-all,ctrl-t:toggle-all"
        if ($null -eq $chosen) {
            break
        }
        $files += $chosen
    }
    $content = $files | ForEach-Object { 
        $content = Get-Content $_ -Raw
        return "
#REGION $($_)
$content
#ENDREGION
"
    }
    | Out-String
    $content | Set-Clipboard
    Write-Host '$content | Set-Clipboard'
} finally {
    Pop-Location
}
