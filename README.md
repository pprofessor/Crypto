# 🚀 Crypto Options Exchange Platform

یک پلتفرم کامل و امن برای معاملات آپشن ارزهای دیجیتال با معماری میکروسرویس.

## ✨ ویژگی‌های کلیدی

- 🔐 **احراز هویت امن**: JWT با refresh token rotation و 2FA
- 💼 **مدیریت چند ارزی**: پشتیبانی از BTC، ETH، TRX، USDT
- 🏦 **کیف‌پول امن**: تولید آدرس‌های منحصر به فرد برای هر کاربر
- 📊 **معاملات واقعی**: موتور تطبیق سفارش با قیمت‌گذاری زنده
- 🛡️ **امنیت بالا**: bcrypt، rate limiting، validation کامل
- 🐳 **کانتینری‌شده**: اجرا با Docker/Podman
- 📱 **فرانت‌اند مدرن**: Next.js 15 با Tailwind CSS

## 🏗️ معماری فنی

### میکروسرویس‌ها
crypto-exchange/
├── backend/
│   ├── user-service/        # NestJS - مدیریت کاربران و احراز هویت ✅
│   ├── wallet-service/      # Rust - مدیریت کیف‌پول‌ها
│   ├── deposit-service/     # Rust - پردازش واریزها
│   └── tron-listener/       # Rust - مانیتورینگ بلاکچین
├── frontend/                # Next.js 15 - رابط کاربری
├── docker/                  # کانتینرهای Docker/Podman
├── deployment/              # تنظیمات استقرار تولید
└── init-db/                 # اسکریپت‌های راه‌اندازی دیتابیس

### پایگاه داده
- **PostgreSQL 15**: داده‌های تراکنشی و کاربران
- **Redis 7**: کش، session، پیام‌رسانی real-time
- **Schema**: `crypto` با 6 جدول اصلی

## 🚀 شروع سریع

### پیش‌نیازها
- Node.js 18+
- Rust 1.70+ (برای سرویس‌های مالی)
- Podman 4.0+ یا Docker
- PostgreSQL 15 (با Podman خودکار نصب می‌شود)

### نصب و راه‌اندازی
1. کلون ریپازیتوری:
git clone https://github.com/pprofessor/Crypto.git
cd Crypto

2. راه‌اندازی دیتابیس (با Podman):
cd docker
podman-compose -f docker-compose-simple.yml up -d

3. راه‌اندازی سرویس کاربران:
cd backend/user-service
npm install
cp .env.example .env  # تنظیم متغیرهای محیطی
npx ts-node src/main.ts

4. تست API:
curl -X POST http://localhost:3001/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","username":"testuser","password":"Test@1234"}'

## 📚 مستندات کامل

- [📋 چک‌لیست پیشرفت پروژه](FOR-RUN-CHECKLIST.md) - وضعیت فعلی و نقشه راه
- [🔧 مرجع API](API_REFERENCE.md) - مستندات کامل endpoints
- [🗃️ طرح دیتابیس](DATABASE_SCHEMA.md) - جداول و روابط
- [⚙️ راهنمای استقرار](deployment/setup-ubuntu.sh) - استقرار روی سرور اوبونتو

## 🔐 APIهای فعلی (آماده)

### احراز هویت (`/api/auth/*`)
- `POST /register` - ثبت‌نام کاربر جدید
- `POST /login` - ورود با ایمیل یا نام کاربری
- `POST /refresh` - تمدید توکن دسترسی
- `POST /logout` - خروج و باطل‌سازی توکن
- `POST /change-password` - تغییر رمز عبور
- `GET /profile` - دریافت پروفایل کاربر
- `PUT /profile` - به‌روزرسانی پروفایل

### وضعیت سرویس
- `GET /health` - سلامت سرویس (به زودی)

## 🗃️ ساختار دیتابیس

### جداول اصلی:
1. **users** - اطلاعات کاربران (تأیید هویت، KYC، محدودیت‌ها)
2. **cryptocurrencies** - ارزهای پشتیبانی‌شده
3. **wallets** - کیف‌پول‌های کاربران
4. **transactions** - تراکنش‌های مالی
5. **orders** - سفارش‌های معاملاتی
6. **refresh_tokens** - توکن‌های بازنشانی

## 🛠️ توسعه

### تنظیمات محیط توسعه
1. کپی فایل env:
cd backend/user-service
cp .env.example .env

2. ویرایش فایل `.env` با مقادیر مناسب

3. نصب وابستگی‌ها:
npm install

4. راه‌اندازی سرویس:
npm run start:dev

### تست
# تست TypeScript
npx tsc --noEmit

# تست API با curl (پس از راه‌اندازی سرویس)
curl -X POST http://localhost:3001/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"Test@1234"}'

## 📊 وضعیت توسعه

### ✅ تکمیل شده
- [x] ساختار پروژه و ریپازیتوری GitHub
- [x] راه‌اندازی PostgreSQL و Redis با Podman
- [x] طراحی کامل schema دیتابیس
- [x] سرویس کاربران (NestJS) با احراز هویت کامل
- [x] مستندات API و طرح دیتابیس
- [x] تنظیمات Docker/Podman

### 🔄 در حال توسعه
- [ ] سرویس کیف‌پول (Rust)
- [ ] سرویس واریز (Rust)
- [ ] فرانت‌اند Next.js
- [ ] موتور معاملاتی

### ⏳ برنامه‌ریزی شده
- [ ] تست‌های واحد و یکپارچه‌سازی
- [ ] سیستم KYC/AML
- [ ] اعلان‌های real-time
- [ ] استقرار تولید با SSL

## 🤝 مشارکت

1. Fork ریپازیتوری
2. ایجاد Branch جدید (`git checkout -b feature/AmazingFeature`)
3. Commit تغییرات (`git commit -m 'Add some AmazingFeature'`)
4. Push به Branch (`git push origin feature/AmazingFeature`)
5. باز کردن Pull Request

## 📝 لایسنس

این پروژه تحت لایسنس MIT منتشر شده است. جزئیات بیشتر در فایل [LICENSE](LICENSE).

## 👥 حمایت

- گزارش باگ یا درخواست قابلیت جدید: [Issues](https://github.com/pprofessor/Crypto/issues)
- سؤالات فنی: Discussions
- ایمیل: [اطلاعات تماس]

## 📞 تماس با ما

- **توسعه‌دهنده اصلی**: [اطلاعات شما]
- **GitHub**: [@pprofessor](https://github.com/pprofessor)
- **ریپازیتوری**: https://github.com/pprofessor/Crypto

---

**آخرین بروزرسانی**: 2026-01-04  
**ورژن**: 2.0.0  
**وضعیت**: در حال توسعه فعال