# assert we are in the parent of the scripts directory
if ((Get-Location).Path.EndsWith("scripts")) {
    Push-Location (Get-Location).Parent
} else {
    Push-Location $(Get-Location)
}
try {
    $content = "
``````README.md
$(Get-Content README.md -Raw)
``````
===
``````scripts/summarize.ps1
$(Get-Content "scripts/summarize.ps1" -Raw)
``````
===
``````git log --pretty=format:`"%s`" -n 25
$(git log --pretty=format:"%s" -n 25)
``````
===
``````todo.md
$(Get-Content todo.md -Raw)
``````
===

Please let me know your thoughts.
"

    $content
} finally {
    Pop-Location
}
