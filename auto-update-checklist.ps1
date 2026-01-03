# اسکریپت بررسی خودکار برخی موارد
param([switch]\)

function Test-FileExists {
    param([string]\, [string]\)
    if (Test-Path \) {
        Write-Host "  ✅ \" -ForegroundColor Green
        return \True
    } else {
        Write-Host "  ❌ \" -ForegroundColor Red
        return \False
    }
}

function Test-CommandWorks {
    param([string]\, [string]\)
    try {
        Invoke-Expression "\ 2>$null" | Out-Null
        Write-Host "  ✅ \" -ForegroundColor Green
        return \True
    } catch {
        Write-Host "  ❌ \" -ForegroundColor Red
        return \False
    }
}

Write-Host "🔍 بررسی خودکار پروژه..." -ForegroundColor Cyan

# ۱. بررسی فایل‌های ساختاری
\ = @(
    @{Path=".\backend\user-service\src\"; Desc="User Service Source Code"},
    @{Path=".\backend\wallet-service\src\main.rs"; Desc="Wallet Service Main File"},
    @{Path=".\frontend\app\"; Desc="Next.js App Directory"},
    @{Path=".\docker\docker-compose.yml"; Desc="Docker Compose"},
    @{Path=".\README.md"; Desc="Documentation"}
)

\ = 0
foreach (\ in \) {
    if (Test-FileExists \.Path \.Desc) { \++ }
}

# ۲. بررسی پیش‌نیازها (اگر FullCheck باشد)
if (\) {
    Write-Host "
🔧 بررسی پیش‌نیازهای سیستم:" -ForegroundColor Yellow
    Test-CommandWorks "docker --version" "Docker Installation"
    Test-CommandWorks "node --version" "Node.js Installation"
    Test-CommandWorks "cargo --version" "Rust Installation"
}

# ۳. محاسبه درصد تخمینی
\ = \.Count
\ = [math]::Round((\ / \) * 100)

# ۴. آپدیت چک‌لیست
\ = Get-Content ".\FOR-RUN-CHECKLIST.md" -Raw
\ = \ -replace "Overall Progress: \d+%", "Overall Progress: \%"
\ = \ -replace "Last Updated: .*", "Last Updated: \2026-01-03 19:30:27"
Set-Content -Path ".\FOR-RUN-CHECKLIST.md" -Value \ -Encoding UTF8

Write-Host "
📊 نتیجه بررسی:" -ForegroundColor Cyan
Write-Host "  \ از \ مورد ساختاری وجود دارند" -ForegroundColor White
Write-Host "  پیشرفت تخمینی: \%" -ForegroundColor Yellow
Write-Host "  چک‌لیست آپدیت شد" -ForegroundColor Green

Write-Host "
⚠️ توجه: این بررسی فقط وجود فایل‌ها را چک می‌کند" -ForegroundColor Red
Write-Host "  کیفیت کد، عملکرد، و تست‌ها نیاز به بررسی دستی دارند" -ForegroundColor Red
