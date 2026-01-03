import { Entity, PrimaryGeneratedColumn, Column, CreateDateColumn, UpdateDateColumn, OneToMany } from 'typeorm';
import { Exclude } from 'class-transformer';
import { RefreshToken } from '../../auth/entities/refresh-token.entity';

@Entity('users')
export class User {
  @PrimaryGeneratedColumn('uuid')
  id: string;

  @Column({ unique: true, length: 255 })
  email: string;

  @Exclude()
  @Column({ name: 'password_hash', length: 255 })
  passwordHash: string;

  @Column({ name: 'is_verified', default: false })
  isVerified: boolean;

  @Exclude()
  @Column({ name: 'two_factor_secret', nullable: true, length: 255 })
  twoFactorSecret: string | null;

  @Column({ 
    type: 'decimal', 
    precision: 20, 
    scale: 8, 
    default: 0,
    transformer: {
      to: (value: number) => value,
      from: (value: string) => parseFloat(value)
    }
  })
  balance: number;

  @Column({ name: 'last_login_at', nullable: true })
  lastLoginAt: Date | null;

  @Column({ name: 'reset_password_token', nullable: true, length: 255 })
  resetPasswordToken: string | null;

  @Column({ name: 'reset_password_expires', nullable: true })
  resetPasswordExpires: Date | null;

  @CreateDateColumn({ name: 'created_at' })
  createdAt: Date;

  @UpdateDateColumn({ name: 'updated_at' })
  updatedAt: Date;

  @OneToMany(() => RefreshToken, refreshToken => refreshToken.user)
  refreshTokens: RefreshToken[];

  constructor(partial: Partial<User>) {
    Object.assign(this, partial);
  }
}
