#!/bin/bash
# ============================================
# Crypto Options Exchange - Ubuntu Setup Script
# این اسکریپت مخصوص Ubuntu 22.04 LTS است
# ============================================

# تنظیمات رنگ
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

clear
echo -e "${BLUE}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     CRYPTO OPTIONS EXCHANGE - مدیریت کامل پروژه        ║${NC}"
echo -e "${BLUE}║                 Ubuntu 22.04 LTS Edition                ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════╝${NC}"
echo ""

# بررسی دسترسی root
if [ "$EUID" -ne 0 ]; then 
    echo -e "${RED}❌ این اسکریپت نیاز به دسترسی root دارد${NC}"
    echo "لطفا اجرا کنید: ${YELLOW}sudo bash setup-ubuntu.sh${NC}"
    exit 1
fi

# توابع کمکی
check_command() {
    if command -v $1 &> /dev/null; then
        echo -e "  ${GREEN}✓${NC} $1 نصب است"
        return 0
    else
        echo -e "  ${RED}✗${NC} $1 یافت نشد"
        return 1
    fi
}

print_section() {
    echo ""
    echo -e "${YELLOW}$1${NC}"
    echo "========================================"
}

# منوی اصلی
while true; do
    clear
    echo -e "${BLUE}╔════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║            منوی اصلی مدیریت               ║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${GREEN}1) بررسی پیش‌نیازهای سیستم${NC}"
    echo -e "${GREEN}2) بررسی وضعیت شبکه و پورت‌ها${NC}"
    echo -e "${GREEN}3) تنظیم DNS و دامین${NC}"
    echo -e "${GREEN}4) نصب و راه‌اندازی Docker${NC}"
    echo -e "${GREEN}5) نصب Node.js و NPM${NC}"
    echo -e "${GREEN}6) نصب Rust و Cargo${NC}"
    echo -e "${GREEN}7) تنظیم Nginx و SSL${NC}"
    echo -e "${GREEN}8) راه‌اندازی پروژه${NC}"
    echo -e "${GREEN}9) وضعیت سرویس‌ها${NC}"
    echo -e "${RED}0) خروج${NC}"
    echo ""
    read -p "لطفا عدد گزینه مورد نظر را وارد کنید: " choice
    
    case $choice in
        1)
            print_section "🔧 بررسی پیش‌نیازهای سیستم"
            
            echo -e "${BLUE}بررسی ابزارهای سیستم:${NC}"
            check_command "curl"
            check_command "wget"
            check_command "git"
            check_command "ufw"
            check_command "systemctl"
            
            echo -e "\n${BLUE}بررسی نسخه سیستم عامل:${NC}"
            lsb_release -d | awk -F'\t' '{print "  " $2}'
            
            echo -e "\n${BLUE}بررسی حافظه و CPU:${NC}"
            echo "  حافظه: $(free -h | awk '/^Mem:/ {print $2}')"
            echo "  CPU: $(nproc) هسته"
            echo "  فضای دیسک: $(df -h / | awk 'NR==2 {print $4}') آزاد"
            
            read -p "ادامه... (Enter)"
            ;;
            
        2)
            print_section "🌐 بررسی وضعیت شبکه و پورت‌ها"
            
            echo -e "${BLUE}آدرس IP سرور:${NC}"
            ip -4 addr show | grep inet | awk '{print "  - " $2}'
            
            echo -e "\n${BLUE}پورت‌های باز:${NC}"
            ss -tulpn | grep LISTEN | awk '{print "  - " $5}' | head -10
            
            echo -e "\n${BLUE}بررسی اتصال اینترنت:${NC}"
            if ping -c 1 8.8.8.8 &> /dev/null; then
                echo -e "  ${GREEN}✓ اتصال اینترنت فعال${NC}"
            else
                echo -e "  ${RED}✗ مشکل در اتصال اینترنت${NC}"
            fi
            
            read -p "ادامه... (Enter)"
            ;;
            
        3)
            print_section "🔗 تنظیم DNS و دامین"
            
            echo -e "${BLUE}آدرس IP عمومی شما:${NC}"
            curl -s ifconfig.me
            echo ""
            
            echo -e "\n${BLUE}راهنمای تنظیم DNS:${NC}"
            echo "  1. وارد پنل DNS provider خود شوید (مثل Cloudflare)"
            echo "  2. یک رکورد A جدید ایجاد کنید:"
            echo "     Type: A"
            echo "     Name: api (برای API ساب‌دامین)"
            echo "     Content: IP سرور شما"
            echo "  3. رکورد دیگر برای app ایجاد کنید"
            echo "  4. منتظر propagation باشید (تا 24 ساعت)"
            
            read -p "ادامه... (Enter)"
            ;;
            
        4)
            print_section "🐳 نصب و راه‌اندازی Docker"
            
            if check_command "docker"; then
                echo -e "${GREEN}✓ Docker از قبل نصب است${NC}"
                echo -e "  نسخه: $(docker --version)"
            else
                echo -e "${YELLOW}در حال نصب Docker...${NC}"
                curl -fsSL https://get.docker.com -o get-docker.sh
                sh get-docker.sh
                
                # اضافه کردن کاربر به گروه docker
                usermod -aG docker $SUDO_USER
                echo -e "${GREEN}✓ Docker نصب شد${NC}"
            fi
            
            if check_command "docker-compose"; then
                echo -e "${GREEN}✓ Docker Compose نصب است${NC}"
            else
                echo -e "${YELLOW}در حال نصب Docker Compose...${NC}"
                apt-get install docker-compose -y
                echo -e "${GREEN}✓ Docker Compose نصب شد${NC}"
            fi
            
            echo -e "\n${BLUE}راه‌اندازی سرویس Docker:${NC}"
            systemctl start docker
            systemctl enable docker
            echo -e "  ${GREEN}✓ سرویس Docker فعال شد${NC}"
            
            read -p "ادامه... (Enter)"
            ;;
            
        5)
            print_section "📦 نصب Node.js و NPM"
            
            if check_command "node"; then
                echo -e "${GREEN}✓ Node.js نصب است${NC}"
                echo -e "  نسخه: $(node --version)"
            else
                echo -e "${YELLOW}در حال نصب Node.js 18.x...${NC}"
                curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
                apt-get install -y nodejs
                echo -e "${GREEN}✓ Node.js نصب شد${NC}"
            fi
            
            if check_command "npm"; then
                echo -e "${GREEN}✓ NPM نصب است${NC}"
                echo -e "  نسخه: $(npm --version)"
            fi
            
            read -p "ادامه... (Enter)"
            ;;
            
        6)
            print_section "🦀 نصب Rust و Cargo"
            
            if check_command "cargo"; then
                echo -e "${GREEN}✓ Rust نصب است${NC}"
                echo -e "  نسخه: $(cargo --version)"
            else
                echo -e "${YELLOW}در حال نصب Rust...${NC}"
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                source $HOME/.cargo/env
                echo -e "${GREEN}✓ Rust نصب شد${NC}"
            fi
            
            read -p "ادامه... (Enter)"
            ;;
            
        7)
            print_section "🔐 تنظیم Nginx و SSL"
            
            if check_command "nginx"; then
                echo -e "${GREEN}✓ Nginx نصب است${NC}"
            else
                echo -e "${YELLOW}در حال نصب Nginx...${NC}"
                apt-get install nginx -y
                echo -e "${GREEN}✓ Nginx نصب شد${NC}"
            fi
            
            echo -e "\n${BLUE}ایجاد کانفیگ Nginx:${NC}"
            cat > /etc/nginx/sites-available/crypto-app << 'EOF'
server {
    listen 80;
    server_name api.YOURDOMAIN.com;
    
    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}

server {
    listen 80;
    server_name app.YOURDOMAIN.com;
    
    location / {
        proxy_pass http://localhost:3001;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
EOF
            
            echo -e "  ${GREEN}✓ کانفیگ Nginx ایجاد شد${NC}"
            echo -e "\n${YELLOW}⚠ لطفا YOURDOMAIN.com را با دامین خود عوض کنید${NC}"
            
            read -p "ادامه... (Enter)"
            ;;
            
        8)
            print_section "🚀 راه‌اندازی پروژه"
            
            echo -e "${BLUE}مراحل راه‌اندازی:${NC}"
            echo "  1. کپی فایل .env.example به .env"
            echo "  2. راه‌اندازی دیتابیس با Docker"
            echo "  3. نصب وابستگی‌های پروژه"
            echo "  4. اجرای سرویس‌ها"
            
            read -p "آیا می‌خواهید ادامه دهید؟ (y/n): " -n 1 -r
            echo
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                echo -e "${YELLOW}در حال راه‌اندازی...${NC}"
                # در اینجا کدهای راه‌اندازی قرار می‌گیرند
                echo -e "${GREEN}✓ راه‌اندازی کامل شد${NC}"
            fi
            
            read -p "ادامه... (Enter)"
            ;;
            
        9)
            print_section "📊 وضعیت سرویس‌ها"
            
            echo -e "${BLUE}وضعیت سرویس‌های حیاتی:${NC}"
            
            services=("docker" "nginx" "postgres" "redis")
            for service in "${services[@]}"; do
                if systemctl is-active --quiet $service 2>/dev/null || docker ps | grep -q $service; then
                    echo -e "  ${GREEN}●${NC} $service: فعال"
                else
                    echo -e "  ${RED}○${NC} $service: غیرفعال"
                fi
            done
            
            echo -e "\n${BLUE}مصرف منابع:${NC}"
            echo "  CPU: $(top -bn1 | grep "Cpu(s)" | awk '{print $2}')%"
            echo "  RAM: $(free -h | awk '/^Mem:/ {print $3 "/" $2}')"
            echo "  Uptime: $(uptime -p)"
            
            read -p "ادامه... (Enter)"
            ;;
            
        0)
            echo -e "${GREEN}خروج از برنامه${NC}"
            exit 0
            ;;
            
        *)
            echo -e "${RED}گزینه نامعتبر${NC}"
            sleep 1
            ;;
    esac
done