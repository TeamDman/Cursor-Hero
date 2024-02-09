param (
    [Parameter(Mandatory=$true, Position=0)]
    [ValidateScript({Test-Path $_ -PathType 'Leaf'})]
    [string]$NotebookPath,

    [Parameter(Position=1)]
    [switch]$IncludeOutputs
)

# Load the notebook as JSON
$notebook = Get-Content -Raw -Path $NotebookPath | ConvertFrom-Json

# Function to format code cells as markdown entries, correctly handling the array of lines in $Content
function Format-CodeCell {
    param (
        [Parameter(Mandatory=$true)]
        [Object[]]$Content, # Changed to Object[] to handle both source and outputs correctly

        [Parameter(Mandatory=$false)]
        [Object[]]$Outputs
    )

    $formattedContent = $Content -join ""
    $markdown = "``````py`n$formattedContent`n``````"

    if ($IncludeOutputs -and $Outputs) {
        $formattedOutputs = $Outputs | ForEach-Object {
            if ($_.output_type -eq "stream") {
                $_.text -join "`n"
            }
            elseif ($_.output_type -eq "execute_result" -or $_.output_type -eq "display_data") {
                $_.data."text/plain" -join "`n"
            }
        } -join "`n"

        if ($formattedOutputs) {
            $markdown += "`n```````n$formattedOutputs`n``````"
        }
    }

    $markdown
}

# Map code cells to formatted markdown entries
$markdownEntries = $notebook.cells | Where-Object { $_.cell_type -eq 'code' } | ForEach-Object {
    $source = $_.source
    $outputs = if ($IncludeOutputs) { $_.outputs } else { $null }
    Format-CodeCell -Content $source -Outputs $outputs
}

# Copy the result to the clipboard
$markdownEntries -join "`n`n" | Set-Clipboard
