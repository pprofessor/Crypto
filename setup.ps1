# Crypto Options Exchange - Setup Script
# Version: 1.0.0
# Description: Automated setup script for development environment

Write-Host "=========================================" -ForegroundColor Cyan
Write-Host "  CRYPTO OPTIONS EXCHANGE SETUP" -ForegroundColor Cyan
Write-Host "=========================================" -ForegroundColor Cyan

function Test-Command {
    param([string])
    try {
        Get-Command  -ErrorAction Stop | Out-Null
        return True
    } catch {
        return False
    }
}

function Show-Step {
    param([string], [int])
    Write-Host "
[/7] " -ForegroundColor Yellow
}

# Step 1: Verify prerequisites
Show-Step "Checking prerequisites" 1

 = @{
    "Docker" = Test-Command "docker";
    "Docker Compose" = Test-Command "docker-compose";
    "Node.js" = Test-Command "node";
    "npm" = Test-Command "npm";
    "Rust" = Test-Command "cargo";
    "Git" = Test-Command "git";
}

foreach ( in .Keys) {
    if ([]) {
        Write-Host "  ✓ " -ForegroundColor Green
    } else {
        Write-Host "  ✗  (Not found)" -ForegroundColor Red
    }
}

# Step 2: Copy environment file
Show-Step "Setting up environment variables" 2
if (Test-Path ".\.env.example" -and -not (Test-Path ".\.env")) {
    Copy-Item ".\.env.example" ".\.env"
    Write-Host "  ✓ Created .env file from template" -ForegroundColor Green
} else {
    Write-Host "  ⚠ .env file already exists or template missing" -ForegroundColor Yellow
}

# Step 3: Start Docker services
Show-Step "Starting Docker containers" 3
if (["Docker"] -and ["Docker Compose"]) {
    try {
        Set-Location ".\docker"
        docker-compose up -d
        Write-Host "  ✓ Docker containers started" -ForegroundColor Green
        
        # Wait for PostgreSQL to be ready
        Write-Host "  ⏳ Waiting for database to be ready..." -ForegroundColor Yellow
        Start-Sleep -Seconds 10
    } catch {
        Write-Host "  ✗ Failed to start Docker: " -ForegroundColor Red
    }
    Set-Location ".."
} else {
    Write-Host "  ⚠ Docker not available, skipping container setup" -ForegroundColor Yellow
}

# Step 4: Install Node.js dependencies
Show-Step "Installing Node.js dependencies" 4
if (["npm"]) {
    # User Service
    if (Test-Path ".\backend\user-service") {
        Set-Location ".\backend\user-service"
        npm install
        Write-Host "  ✓ User service dependencies installed" -ForegroundColor Green
        Set-Location "..\.."
    }
    
    # Frontend
    if (Test-Path ".\frontend") {
        Set-Location ".\frontend"
        npm install
        Write-Host "  ✓ Frontend dependencies installed" -ForegroundColor Green
        Set-Location ".."
    }
} else {
    Write-Host "  ⚠ npm not available, skipping Node.js dependencies" -ForegroundColor Yellow
}

# Step 5: Build Rust services
Show-Step "Building Rust services" 5
if (["Rust"]) {
     = @("wallet-service", "deposit-service", "tron-listener")
    
    foreach ( in ) {
         = ".\backend\"
        if (Test-Path ) {
            Set-Location 
            Write-Host "  Building ..." -ForegroundColor Gray
            cargo build --quiet
            Write-Host "  ✓  built successfully" -ForegroundColor Green
            Set-Location "..\.."
        }
    }
} else {
    Write-Host "  ⚠ Rust not available, skipping Rust services" -ForegroundColor Yellow
}

# Step 6: Initialize Git repository
Show-Step "Initializing Git repository" 6
if (["Git"] -and -not (Test-Path ".\.git")) {
    git init
    git add .
    git commit -m "Initial commit: Project structure setup"
    Write-Host "  ✓ Git repository initialized" -ForegroundColor Green
} elseif (Test-Path ".\.git") {
    Write-Host "  ⚠ Git repository already exists" -ForegroundColor Yellow
} else {
    Write-Host "  ⚠ Git not available, skipping repository initialization" -ForegroundColor Yellow
}

# Step 7: Final summary
Show-Step "Setup complete!" 7
Write-Host "
=========================================" -ForegroundColor Cyan
Write-Host "  SETUP SUMMARY" -ForegroundColor Cyan
Write-Host "=========================================" -ForegroundColor Cyan

Write-Host "
Next steps:" -ForegroundColor Green
Write-Host "1. Review .env file configuration" -ForegroundColor White
Write-Host "2. Run database migrations:" -ForegroundColor White
Write-Host "   cd backend\user-service" -ForegroundColor Gray
Write-Host "   npm run migration:run" -ForegroundColor Gray
Write-Host "3. Start services:" -ForegroundColor White
Write-Host "   User Service:   cd backend\user-service && npm run start:dev" -ForegroundColor Gray
Write-Host "   Wallet Service: cd backend\wallet-service && cargo run" -ForegroundColor Gray
Write-Host "   Frontend:       cd frontend && npm run dev" -ForegroundColor Gray
Write-Host "4. Access the application:" -ForegroundColor White
Write-Host "   Frontend: http://localhost:3001" -ForegroundColor Gray
Write-Host "   API Docs: http://localhost:3000/api" -ForegroundColor Gray

Write-Host "
Troubleshooting:" -ForegroundColor Yellow
Write-Host "• Check Docker: docker ps" -ForegroundColor Gray
Write-Host "• View logs: docker-compose -f docker\docker-compose.yml logs" -ForegroundColor Gray
Write-Host "• Test database: psql -U crypto_user -d crypto_options" -ForegroundColor Gray

Write-Host "
=========================================" -ForegroundColor Cyan
Write-Host "  Happy coding! 🚀" -ForegroundColor Cyan
Write-Host "=========================================" -ForegroundColor Cyan
