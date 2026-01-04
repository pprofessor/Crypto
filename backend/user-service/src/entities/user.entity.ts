import { 
  Entity, 
  PrimaryGeneratedColumn, 
  Column, 
  CreateDateColumn, 
  UpdateDateColumn, 
  DeleteDateColumn,
  Index 
} from 'typeorm';
import { Exclude } from 'class-transformer';

@Entity({ name: 'users', schema: 'crypto' })
@Index('idx_users_email', ['email'], { where: 'deleted_at IS NULL' })
@Index('idx_users_status', ['isActive', 'isVerified'], { where: 'deleted_at IS NULL' })
export class User {
  @PrimaryGeneratedColumn('uuid')
  id: string;

  @Column({ type: 'varchar', length: 255, unique: true })
  email: string;

  @Column({ type: 'varchar', length: 50, unique: true })
  username: string;

  @Column({ type: 'varchar', length: 20, nullable: true })
  phoneNumber: string;

  @Exclude()
  @Column({ name: 'password_hash', type: 'varchar', length: 255 })
  passwordHash: string;

  @Exclude()
  @Column({ name: 'two_factor_secret', type: 'varchar', length: 255, nullable: true })
  twoFactorSecret: string | null;

  @Exclude()
  @Column({ name: 'recovery_codes', type: 'simple-array', nullable: true })
  recoveryCodes: string[];

  @Column({ name: 'is_active', type: 'boolean', default: true })
  isActive: boolean;

  @Column({ name: 'is_verified', type: 'boolean', default: false })
  isVerified: boolean;

  @Column({ name: 'is_2fa_enabled', type: 'boolean', default: false })
  is2faEnabled: boolean;

  @Column({ 
    name: 'user_type', 
    type: 'varchar', 
    length: 20, 
    default: 'regular',
    enum: ['regular', 'vip', 'admin']
  })
  userType: string;

  @Column({ name: 'failed_login_attempts', type: 'int', default: 0 })
  failedLoginAttempts: number;

  @Column({ name: 'last_login_at', type: 'timestamptz', nullable: true })
  lastLoginAt: Date;

  @Exclude()
  @Column({ name: 'password_changed_at', type: 'timestamptz', nullable: true })
  passwordChangedAt: Date;

  @Exclude()
  @Column({ name: 'last_password_hash', type: 'varchar', length: 255, nullable: true })
  lastPasswordHash: string;

  @Column({ 
    name: 'daily_withdrawal_limit', 
    type: 'decimal', 
    precision: 30, 
    scale: 18, 
    default: 1000 
  })
  dailyWithdrawalLimit: number;

  @Column({ 
    name: 'monthly_trade_limit', 
    type: 'decimal', 
    precision: 30, 
    scale: 18, 
    default: 10000 
  })
  monthlyTradeLimit: number;

  @Column({ name: 'withdrawal_whitelist', type: 'simple-array', nullable: true })
  withdrawalWhitelist: string[];

  @Column({ 
    name: 'kyc_status', 
    type: 'varchar', 
    length: 20, 
    default: 'PENDING',
    enum: ['PENDING', 'VERIFIED', 'REJECTED']
  })
  kycStatus: string;

  @Column({ name: 'kyc_verified_at', type: 'timestamptz', nullable: true })
  kycVerifiedAt: Date;

  @Column({ name: 'kyc_document_type', type: 'varchar', length: 50, nullable: true })
  kycDocumentType: string;

  @Column({ name: 'first_name', type: 'varchar', length: 100, nullable: true })
  firstName: string;

  @Column({ name: 'last_name', type: 'varchar', length: 100, nullable: true })
  lastName: string;

  @CreateDateColumn({ name: 'created_at' })
  createdAt: Date;

  @UpdateDateColumn({ name: 'updated_at' })
  updatedAt: Date;

  @DeleteDateColumn({ name: 'deleted_at' })
  deletedAt: Date;

  constructor(partial: Partial<User>) {
    Object.assign(this, partial);
  }

  // Helper methods
  isAdmin(): boolean {
    return this.userType === 'admin';
  }

  isVIP(): boolean {
    return this.userType === 'vip';
  }

  isAccountLocked(): boolean {
    return this.failedLoginAttempts >= 5;
  }

  getFullName(): string {
    return `${this.firstName || ''} ${this.lastName || ''}`.trim();
  }
}
