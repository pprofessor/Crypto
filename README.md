# 🚀 Crypto Options Exchange

**Professional Cryptocurrency Binary Options Trading Platform**

## 📋 Project Overview
A microservices-based trading platform for cryptocurrency binary options, featuring real-time trading, wallet management, and blockchain integration.

## 🏗️ Architecture

\\\
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Frontend      │    │   API Gateway   │    │   User Service  │
│   (Next.js 15)  │◄──►│   (NestJS)      │◄──►│   (NestJS)      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         │                       │              ┌────────┴────────┐
         │                       │              │                 │
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Wallet Service│    │ Deposit Service │    │ Tron Listener   │
│   (Rust)        │    │ (Rust)          │    │ (Rust)          │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────┬───────────┘                       │
                     │                                   │
              ┌─────────────┐                    ┌─────────────┐
              │ PostgreSQL  │                    │  TRON Node  │
              │  & Redis    │                    │  Blockchain │
              └─────────────┘                    └─────────────┘
\\\

## 🚀 Quick Start

### Prerequisites
- Docker & Docker Compose
- Node.js 18+
- Rust 1.70+
- PostgreSQL 15

### Installation

1. **Clone the repository**
   \\\ash
   git clone https://github.com/pprofessor/crypto-options-exchange.git
   cd crypto-options-exchange
   \\\

2. **Start infrastructure**
   \\\ash
   cd docker
   docker-compose up -d
   \\\

3. **Setup environment variables**
   \\\ash
   cp .env.example .env
   # Edit .env with your configuration
   \\\

4. **Install dependencies**
   \\\ash
   # User Service
   cd backend/user-service
   npm install

   # Frontend
   cd ../../frontend
   npm install

   # Rust Services
   cd ../backend/wallet-service
   cargo build

   cd ../deposit-service
   cargo build

   cd ../tron-listener
   cargo build
   \\\

5. **Run database migrations**
   \\\ash
   cd backend/user-service
   npm run migration:run
   \\\

6. **Start services**
   \\\ash
   # Terminal 1: User Service
   cd backend/user-service
   npm run start:dev

   # Terminal 2: Wallet Service
   cd backend/wallet-service
   cargo run

   # Terminal 3: Deposit Service
   cd backend/deposit-service
   cargo run

   # Terminal 4: Frontend
   cd frontend
   npm run dev
   \\\

## 📡 API Documentation

### Authentication Endpoints
- \POST /auth/register\ - Register new user
- \POST /auth/login\ - Login with credentials
- \POST /auth/logout\ - Logout user
- \GET /auth/profile\ - Get user profile

### Wallet Endpoints
- \POST /wallet/create\ - Create new wallet
- \GET /wallet/:user_id\ - Get wallet details
- \GET /wallet/balance/:user_id\ - Get wallet balance

### Deposit Endpoints
- \POST /deposit/create\ - Create deposit request
- \GET /deposits/:user_id\ - Get user deposits
- \GET /deposit/status/:deposit_id\ - Check deposit status

## 🗄️ Database Schema

### Users Table
- \id\ UUID PRIMARY KEY
- \email\ VARCHAR(255) UNIQUE
- \password_hash\ VARCHAR(255)
- \is_verified\ BOOLEAN DEFAULT false
- \created_at\ TIMESTAMP
- \updated_at\ TIMESTAMP

### Wallets Table
- \id\ UUID PRIMARY KEY
- \user_id\ UUID REFERENCES users(id)
- \	ron_address\ VARCHAR(255)
- \usdt_balance\ DECIMAL(20, 8)
- \created_at\ TIMESTAMP

### Deposits Table
- \id\ UUID PRIMARY KEY
- \user_id\ UUID REFERENCES users(id)
- \mount\ DECIMAL(20, 8)
- \status\ VARCHAR(50)
- \	ransaction_hash\ VARCHAR(255)
- \created_at\ TIMESTAMP
- \expires_at\ TIMESTAMP

## 🔧 Technology Stack

### Backend Services
- **User Service**: NestJS, TypeORM, PostgreSQL, JWT
- **Wallet Service**: Rust, Actix-web, SQLx, PostgreSQL
- **Deposit Service**: Rust, Actix-web, SQLx, TRON integration
- **Tron Listener**: Rust, TRON-RS, PostgreSQL

### Frontend
- **Framework**: Next.js 15 (App Router)
- **Styling**: Tailwind CSS, shadcn/ui
- **State Management**: React Query, Jotai
- **Charts**: Recharts
- **Animations**: Framer Motion

### Infrastructure
- **Database**: PostgreSQL 15
- **Cache**: Redis
- **Containerization**: Docker, Docker Compose
- **Blockchain**: TRON Network

## 📊 Development Status

| Service | Status | Progress | Notes |
|---------|--------|----------|-------|
| User Service | 🟡 In Progress | 85% | JWT auth, user management |
| Wallet Service | 🟡 In Progress | 70% | Basic wallet operations |
| Deposit Service | 🟡 In Progress | 60% | TRON deposit processing |
| Tron Listener | 🔴 Not Started | 0% | Blockchain monitoring |
| Frontend | 🟡 In Progress | 50% | Dashboard, trading UI |
| Database | ✅ Complete | 100% | Schema designed |
| Docker Setup | ✅ Complete | 100% | Multi-container |

## 🛡️ Security Features

- JWT-based authentication with refresh tokens
- Password hashing with bcrypt
- SQL injection prevention (parameterized queries)
- CORS configuration
- Rate limiting
- Input validation
- Environment-based configuration
- HTTPS enforcement

## 🚧 Roadmap

### Phase 1: Core Infrastructure (Current)
- [x] Project structure setup
- [x] Database design
- [x] Docker configuration
- [ ] Basic API endpoints
- [ ] User authentication

### Phase 2: Wallet & Deposits
- [ ] Wallet creation and management
- [ ] TRON deposit processing
- [ ] Transaction monitoring
- [ ] Balance updates

### Phase 3: Trading Engine
- [ ] Price feeds integration
- [ ] Order matching system
- [ ] P&L calculation
- [ ] Real-time updates

### Phase 4: Advanced Features
- [ ] Withdrawal system
- [ ] Admin dashboard
- [ ] Analytics and reporting
- [ ] Multi-chain support

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (\git checkout -b feature/amazing-feature\)
3. Commit changes (\git commit -m 'Add amazing feature'\)
4. Push to branch (\git push origin feature/amazing-feature\)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🆘 Support

- **Documentation**: [API Reference](./API_REFERENCE.md)
- **Issues**: [GitHub Issues](https://github.com/pprofessor/crypto-options-exchange/issues)
- **Discussion**: [GitHub Discussions](https://github.com/pprofessor/crypto-options-exchange/discussions)

---
**Built with ❤️ for the crypto trading community**
