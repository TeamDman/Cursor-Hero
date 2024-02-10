param(
    [Parameter(Mandatory=$true)]
    [string]$template_name,

    [Parameter(Mandatory=$true)]
    [string]$crate_name
)

function Convert-ToPascalCase {
    param(
        [string]$name
    )
    # Simple conversion: split by '_', '-', or ' ', and capitalize each part
    $pascalCase = ($name -split '[-_ ]' | ForEach-Object { $_.Substring(0,1).ToUpper() + $_.Substring(1).ToLower() }) -join ''
    return $pascalCase
}

$crate_name_pascal = Convert-ToPascalCase -name $crate_name

$template_dir = "./patterns/$template_name"
$parent_dir = (Get-Item $template_dir).Parent.FullName

# Function to process and apply template to a file
function Apply-Template {
    param(
        [string]$templatePath,
        [string]$destinationPath,
        [string]$crateName,
        [string]$crateNamePascal
    )
    
    # Read the template content
    $templateContent = Get-Content $templatePath -Raw

    # Replace template variables with actual values
    $processedContent = $templateContent -replace '\{\{crate_name\}\}', $crateName `
                                        -replace '\{\{crate_name_pascal\}\}', $crateNamePascal

    # Check if destination file exists, then apply template logic
    if (Test-Path $destinationPath) {
        Write-Host "Applying template to $destinationPath"
        # Additional logic here for applying the template
        # For example, you might want to insert the processed content into the file
    }
    else {
        Write-Host "Creating new file with template at $destinationPath"
        # Create new file or append content as needed
    }

    # Write or append processed content to the destination file
    Set-Content -Path $destinationPath -Value $processedContent
}

# Recursively find all files in the template directory and apply them
Get-ChildItem -Path $template_dir -Recurse -File | ForEach-Object {
    $relativePath = $_.FullName.Substring((Get-Item $template_dir).FullName.Length)
    $destinationPath = Join-Path $parent_dir $relativePath.TrimStart('\')

    Apply-Template -templatePath $_.FullName -destinationPath $destinationPath `
                   -crateName $crate_name -crateNamePascal $crate_name_pascal
}
