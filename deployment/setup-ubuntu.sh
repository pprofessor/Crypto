#!/bin/bash
# Crypto Options Exchange - Ubuntu Deployment Script
# واقعی و قابل اجرا

set -e  # Exit on error

echo "========================================"
echo "  Crypto Options Exchange - Ubuntu Setup"
echo "========================================"

# مرحله 1: آپدیت سیستم
echo "[1/7] آپدیت سیستم..."
apt-get update
apt-get upgrade -y

# مرحله 2: نصب Docker
echo "[2/7] نصب Docker..."
if ! command -v docker &> /dev/null; then
    curl -fsSL https://get.docker.com -o get-docker.sh
    sh get-docker.sh
    usermod -aG docker \
fi

# مرحله 3: نصب Docker Compose
echo "[3/7] نصب Docker Compose..."
apt-get install docker-compose -y

# مرحله 4: نصب Node.js
echo "[4/7] نصب Node.js..."
if ! command -v node &> /dev/null; then
    curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
    apt-get install -y nodejs
fi

# مرحله 5: نصب Nginx
echo "[5/7] نصب Nginx..."
apt-get install nginx certbot python3-certbot-nginx -y

# مرحله 6: تنظیم فایروال
echo "[6/7] تنظیم فایروال..."
ufw allow 22/tcp
ufw allow 80/tcp
ufw allow 443/tcp
ufw --force enable

# مرحله 7: نمایش راهنمای نهایی
echo "[7/7] راه‌اندازی کامل شد!"
echo ""
echo "========================================"
echo "  قدم‌های بعدی:"
echo "========================================"
echo "1. دامین خود را تنظیم کنید (DNS Records)"
echo "2. فایل .env را ویرایش کنید:"
echo "   nano .env"
echo "3. SSL certificate بگیرید:"
echo "   certbot --nginx -d api.yourdomain.com -d app.yourdomain.com"
echo "4. پروژه را اجرا کنید:"
echo "   docker-compose -f deployment/docker-compose.prod.yml up -d"
echo "5. لاگ‌ها را بررسی کنید:"
echo "   docker-compose logs -f"
echo "========================================"
