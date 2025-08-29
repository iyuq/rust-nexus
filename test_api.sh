# API Testing Script
# Make sure the server is running before executing these commands

# Health check
curl -X GET http://localhost:3000/health

# Create a user
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "name": "John Doe",
    "email": "john.doe@example.com"
  }'

# Get all users
curl -X GET http://localhost:3000/api/users

# Get user by ID (replace with actual UUID from create response)
curl -X GET http://localhost:3000/api/users/YOUR_USER_ID_HERE

# Update user (replace with actual UUID)
curl -X PUT http://localhost:3000/api/users/YOUR_USER_ID_HERE \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Jane Doe",
    "email": "jane.doe@example.com"
  }'

# Delete user (replace with actual UUID)
curl -X DELETE http://localhost:3000/api/users/YOUR_USER_ID_HERE
