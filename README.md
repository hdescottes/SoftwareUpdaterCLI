# Software Updater CLI
![build workflow](https://github.com/hdescottes/SoftwareUpdaterCLI/actions/workflows/build.yaml/badge.svg)

## Description
<p>This project is a software update CLI that helps tracking the local software version.<br>
</p>

## Example
### Json file
```json
[
  {
    "name": "Intellij idea ICC",
    "update_command": "winget upgrade JetBrains.IntelliJIDEA.Community",
    "current_version_command": "findstr \"\\\"version\\\": \" \"C:\\Program Files\\JetBrains\\IntelliJ IDEA Community Edition 2024.3.3\\product-info.json\"",
    "latest_version_command": "curl -s \"https://data.services.jetbrains.com/products/releases?code=IIC&latest=true&type=release\" | awk -F'\"' '/\"version\"'"
  }
]