param (
  [Parameter(Mandatory = $true)]
  [string]$raw
)

# Kebab case problem name
case kebab $raw
$kebab = Get-Clipboard

# Snake case problem name
case snake $raw
$snake = Get-Clipboard 

# Makde the problem directory
mkdir $kebab

# Enter the problem directory
cd $kebab

# Make the language dirctories
mkdir python
mkdir csharp
mkdir rust

# Enter csharp create the problem.cs file and leave 
cd csharp
"" > "$snake.cs"
cd ..

# Enter python create the problem.py file and leave 
cd python
"" > "$snake.py"
cd ..

# Exit the problem directory
cd..

# cd into the problem directory and list its items
c $kebab
