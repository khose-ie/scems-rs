param (
    [string]$directory = "C:\path\to\your\directory"
)

# Check whether the directory is existerence
if (-Not (Test-Path $directory))
{
    Write-Host "Directory can not found: $directory"
    exit
}

# Select all files in this directory and sub directories
$files = Get-ChildItem -Path $directory -Recurse -File

# Search every files.
foreach ($file in $files)
{
    # Convert file via dos2unix
    dos2unix $file.FullName

    # Check and delete temp file
    $tmpFile = "$($file.FullName).tmp"
    if (Test-Path $tmpFile) {
        Remove-Item $tmpFile
        Write-Host "Delete the temp file: $tmpFile"
    }
}

Write-Host "Done."
