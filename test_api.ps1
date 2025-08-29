# API Testing Script for PowerShell
# Make sure the server is running before executing these commands

# Health check
Write-Host "Testing health endpoint..."
Invoke-RestMethod -Uri "http://localhost:3000/health" -Method Get

# Create a user
Write-Host "`nCreating a user..."
$createUser = @{
    name = "John Doe"
    email = "john.doe@example.com"
} | ConvertTo-Json

$newUser = Invoke-RestMethod -Uri "http://localhost:3000/api/users" -Method Post -Body $createUser -ContentType "application/json"
$userId = $newUser.data.id
Write-Host "Created user with ID: $userId"

# Get all users
Write-Host "`nGetting all users..."
Invoke-RestMethod -Uri "http://localhost:3000/api/users" -Method Get

# Get user by ID
Write-Host "`nGetting user by ID..."
Invoke-RestMethod -Uri "http://localhost:3000/api/users/$userId" -Method Get

# Update user
Write-Host "`nUpdating user..."
$updateUser = @{
    name = "Jane Doe"
    email = "jane.doe@example.com"
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://localhost:3000/api/users/$userId" -Method Put -Body $updateUser -ContentType "application/json"

# Get updated user
Write-Host "`nGetting updated user..."
Invoke-RestMethod -Uri "http://localhost:3000/api/users/$userId" -Method Get

# Delete user
Write-Host "`nDeleting user..."
Invoke-RestMethod -Uri "http://localhost:3000/api/users/$userId" -Method Delete

# Verify deletion
Write-Host "`nVerifying deletion (should return 404)..."
try {
    Invoke-RestMethod -Uri "http://localhost:3000/api/users/$userId" -Method Get
} catch {
    Write-Host "User successfully deleted (404 error expected)"
}
