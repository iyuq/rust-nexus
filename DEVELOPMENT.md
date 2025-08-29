# Development Guide

## Git Workflow & Commit Instructions

### Initial Setup

1. **Initialize Git repository** (if not already done):
```bash
git init
git add .
git commit -m "feat: initial project setup with Axum web server

- Add CRUD API for user management
- Implement PostgreSQL integration with SQLx
- Add database migrations and connection pooling
- Configure CORS and request tracing
- Add comprehensive error handling
- Include API testing files and documentation"
```

2. **Add remote repository**:
```bash
git remote add origin https://github.com/YOUR_USERNAME/YOUR_REPO_NAME.git
git branch -M main
git push -u origin main
```

### Commit Message Convention

Follow [Conventional Commits](https://www.conventionalcommits.org/) format:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

#### Types:
- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring without feature changes
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks, dependency updates
- `perf:` - Performance improvements
- `ci:` - CI/CD configuration changes

#### Examples:
```bash
git commit -m "feat(api): add user CRUD endpoints"
git commit -m "fix(db): resolve connection pool timeout issue"
git commit -m "docs: update API documentation with examples"
git commit -m "refactor(database): consolidate setup functions into single API"
git commit -m "test: add integration tests for user endpoints"
git commit -m "chore: update dependencies to latest versions"
```

### GitHub Copilot Integration

#### 1. Commit Message Generation
Use Copilot to generate commit messages:
```bash
# Stage your changes
git add .

# Use Copilot in terminal or VS Code to suggest commit message
# Type: git commit -m "
# Let Copilot suggest the rest based on your staged changes
```

#### 2. VS Code Integration
- Install "GitHub Copilot" extension
- Install "GitHub Copilot Chat" extension
- Use `Ctrl+I` (or `Cmd+I`) for inline suggestions
- Use `Ctrl+Shift+I` for Copilot Chat

#### 3. Copilot Commands for Development
```bash
# Ask Copilot to explain code
# In VS Code: Select code + Ctrl+Shift+I, then ask "Explain this code"

# Ask for code improvements
# Select code + Ctrl+Shift+I: "How can I improve this code?"

# Generate tests
# Ctrl+Shift+I: "Generate tests for this function"

# Debug help
# Ctrl+Shift+I: "Why is this code not working?"
```

### Branch Strategy

#### Feature Development:
```bash
# Create feature branch
git checkout -b feat/user-authentication
git push -u origin feat/user-authentication

# Make changes and commit
git add .
git commit -m "feat(auth): add JWT authentication middleware"

# Push changes
git push origin feat/user-authentication

# Create Pull Request on GitHub
```

#### Hotfix:
```bash
# Create hotfix branch from main
git checkout main
git pull origin main
git checkout -b hotfix/critical-bug-fix

# Make changes and commit
git add .
git commit -m "fix: resolve critical database connection issue"

# Push and create PR
git push -u origin hotfix/critical-bug-fix
```

### Pre-commit Hooks (Recommended)

Install pre-commit hooks to ensure code quality:

```bash
# Install pre-commit (if using Python)
pip install pre-commit

# Or using Cargo (Rust)
cargo install pre-commit
```

Create `.pre-commit-config.yaml`:
```yaml
repos:
  - repo: local
    hooks:
      - id: cargo-fmt
        name: cargo fmt
        entry: cargo fmt --
        language: system
        types: [rust]
        
      - id: cargo-clippy
        name: cargo clippy
        entry: cargo clippy -- -D warnings
        language: system
        types: [rust]
        
      - id: cargo-test
        name: cargo test
        entry: cargo test
        language: system
        types: [rust]
```

Install hooks:
```bash
pre-commit install
```

### Useful Git Commands

```bash
# Check status
git status

# View changes
git diff
git diff --staged

# Interactive staging
git add -p

# Amend last commit
git commit --amend

# View commit history
git log --oneline --graph

# Stash changes
git stash
git stash pop

# Reset changes
git reset --hard HEAD~1  # Careful! This removes commits

# Clean up branches
git branch -d feature-branch-name
git push origin --delete feature-branch-name
```

### Release Process

1. **Version Bump**:
```bash
# Update version in Cargo.toml
# Then commit
git add Cargo.toml
git commit -m "chore: bump version to 0.2.0"
```

2. **Create Release Tag**:
```bash
git tag -a v0.2.0 -m "Release version 0.2.0

- Add user authentication
- Improve error handling
- Update dependencies"

git push origin v0.2.0
```

3. **GitHub Release**:
- Go to GitHub repository
- Click "Releases" → "Create a new release"
- Select the tag
- Add release notes

### CI/CD with GitHub Actions

Create `.github/workflows/ci.yml`:
```yaml
name: CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_PASSWORD: password
          POSTGRES_DB: test_db
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
      
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
        
    - name: Run tests
      env:
        DATABASE_URL: postgresql://postgres:password@localhost/test_db
      run: |
        cargo test
        
    - name: Check formatting
      run: cargo fmt --check
      
    - name: Run clippy
      run: cargo clippy -- -D warnings
```

### Issue Templates

Create `.github/ISSUE_TEMPLATE/bug_report.md`:
```markdown
---
name: Bug report
about: Create a report to help us improve
title: ''
labels: bug
assignees: ''
---

**Describe the bug**
A clear description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '....'
3. See error

**Expected behavior**
What you expected to happen.

**Environment:**
- OS: [e.g. Windows, macOS, Linux]
- Rust version: [e.g. 1.89.0]
- Database: [e.g. PostgreSQL 15]
```

### Pull Request Template

Create `.github/pull_request_template.md`:
```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Testing
- [ ] Tests added/updated
- [ ] All tests passing
- [ ] Manual testing completed

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No new warnings introduced
```

This comprehensive guide covers all aspects of Git workflow, GitHub Copilot integration, and development best practices for your Rust web server project!
