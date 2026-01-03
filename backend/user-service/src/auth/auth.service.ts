import { Injectable, UnauthorizedException, ConflictException, BadRequestException } from '@nestjs/common';
import { JwtService } from '@nestjs/jwt';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import * as bcrypt from 'bcrypt';
import { v4 as uuidv4 } from 'uuid';

import { User } from '../users/entities/user.entity';
import { UsersService } from '../users/users.service';
import { RefreshToken } from './entities/refresh-token.entity';
import { RegisterDto } from './dto/register.dto';
import { LoginDto } from './dto/login.dto';
import { ChangePasswordDto } from './dto/change-password.dto';
import { WalletServiceClient } from '../clients/wallet-service.client';

@Injectable()
export class AuthService {
  private readonly SALT_ROUNDS = 12;

  constructor(
    @InjectRepository(User)
    private usersRepository: Repository<User>,
    @InjectRepository(RefreshToken)
    private refreshTokensRepository: Repository<RefreshToken>,
    private readonly usersService: UsersService,
    private readonly jwtService: JwtService,
    private readonly walletServiceClient: WalletServiceClient,
  ) {}

  /**
   * ثبت‌نام کاربر جدید
   */
  async register(registerDto: RegisterDto): Promise<{ user: User; accessToken: string; refreshToken: string }> {
    // بررسی وجود کاربر با ایمیل مشابه
    const existingUser = await this.usersRepository.findOne({
      where: { email: registerDto.email },
    });

    if (existingUser) {
      throw new ConflictException('User with this email already exists');
    }

    // هش کردن پسورد
    const passwordHash = await bcrypt.hash(registerDto.password, this.SALT_ROUNDS);

    // ایجاد کاربر جدید
    const user = this.usersRepository.create({
      email: registerDto.email,
      passwordHash,
      isVerified: false,
      balance: 0,
    });

    await this.usersRepository.save(user);

    // ایجاد کیف پول برای کاربر
    try {
      await this.walletServiceClient.createWallet(user.id);
    } catch (error) {
      // اگر ایجاد کیف پول شکست خورد، کاربر را حذف کن
      await this.usersRepository.remove(user);
      throw new BadRequestException('Failed to create wallet. Please try again.');
    }

    // تولید توکن‌ها
    const tokens = await this.generateTokens(user);

    return {
      user: this.sanitizeUser(user),
      ...tokens,
    };
  }

  /**
   * ورود کاربر
   */
  async login(loginDto: LoginDto): Promise<{ user: User; accessToken: string; refreshToken: string }> {
    const user = await this.usersRepository.findOne({
      where: { email: loginDto.email },
    });

    if (!user) {
      throw new UnauthorizedException('Invalid credentials');
    }

    // بررسی پسورد
    const isPasswordValid = await bcrypt.compare(loginDto.password, user.passwordHash);
    if (!isPasswordValid) {
      throw new UnauthorizedException('Invalid credentials');
    }

    // بررسی اگر کاربر تأیید نشده باشد
    if (!user.isVerified) {
      // در اینجا می‌توان ایمیل تأیید ارسال کرد
      throw new UnauthorizedException('Please verify your email address');
    }

    // تولید توکن‌ها
    const tokens = await this.generateTokens(user);

    // بروزرسانی زمان آخرین ورود
    user.lastLoginAt = new Date();
    await this.usersRepository.save(user);

    return {
      user: this.sanitizeUser(user),
      ...tokens,
    };
  }

  /**
   * تغییر پسورد
   */
  async changePassword(userId: string, changePasswordDto: ChangePasswordDto): Promise<void> {
    const user = await this.usersRepository.findOne({
      where: { id: userId },
    });

    if (!user) {
      throw new UnauthorizedException('User not found');
    }

    // بررسی پسورد قدیمی
    const isOldPasswordValid = await bcrypt.compare(
      changePasswordDto.oldPassword,
      user.passwordHash,
    );

    if (!isOldPasswordValid) {
      throw new UnauthorizedException('Old password is incorrect');
    }

    // هش کردن پسورد جدید
    const newPasswordHash = await bcrypt.hash(changePasswordDto.newPassword, this.SALT_ROUNDS);
    user.passwordHash = newPasswordHash;

    // باطل کردن همه refresh tokenهای قبلی
    await this.refreshTokensRepository.delete({ userId });

    await this.usersRepository.save(user);
  }

  /**
   * ریفرش توکن
   */
  async refreshToken(refreshToken: string): Promise<{ accessToken: string; refreshToken: string }> {
    const tokenRecord = await this.refreshTokensRepository.findOne({
      where: { token: refreshToken, isRevoked: false },
      relations: ['user'],
    });

    if (!tokenRecord) {
      throw new UnauthorizedException('Invalid refresh token');
    }

    // بررسی انقضا
    if (tokenRecord.expiresAt < new Date()) {
      await this.refreshTokensRepository.update(tokenRecord.id, { isRevoked: true });
      throw new UnauthorizedException('Refresh token has expired');
    }

    // تولید توکن‌های جدید
    const tokens = await this.generateTokens(tokenRecord.user);

    // غیرفعال کردن توکن قدیمی
    await this.refreshTokensRepository.update(tokenRecord.id, { isRevoked: true });

    return tokens;
  }

  /**
   * خروج کاربر
   */
  async logout(userId: string, refreshToken?: string): Promise<void> {
    if (refreshToken) {
      // غیرفعال کردن توکن خاص
      await this.refreshTokensRepository.update(
        { token: refreshToken, userId },
        { isRevoked: true },
      );
    } else {
      // غیرفعال کردن همه توکن‌های کاربر
      await this.refreshTokensRepository.update(
        { userId, isRevoked: false },
        { isRevoked: true },
      );
    }
  }

  /**
   * تولید access و refresh token
   */
  private async generateTokens(user: User): Promise<{ accessToken: string; refreshToken: string }> {
    const payload = {
      sub: user.id,
      email: user.email,
    };

    const accessToken = this.jwtService.sign(payload);

    // ایجاد refresh token
    const refreshToken = uuidv4();
    const refreshTokenExpiry = new Date();
    refreshTokenExpiry.setDate(refreshTokenExpiry.getDate() + 7); // 7 روز

    const refreshTokenRecord = this.refreshTokensRepository.create({
      token: refreshToken,
      userId: user.id,
      expiresAt: refreshTokenExpiry,
      isRevoked: false,
    });

    await this.refreshTokensRepository.save(refreshTokenRecord);

    return {
      accessToken,
      refreshToken,
    };
  }

  /**
   * پاکسازی اطلاعات حساس کاربر
   */
  private sanitizeUser(user: User): User {
    const { passwordHash, twoFactorSecret, ...sanitizedUser } = user;
    return sanitizedUser as User;
  }

  /**
   * تأیید ایمیل
   */
  async verifyEmail(userId: string): Promise<void> {
    await this.usersRepository.update(userId, { isVerified: true });
  }

  /**
   * درخواست ریست پسورد
   */
  async requestPasswordReset(email: string): Promise<{ resetToken: string }> {
    const user = await this.usersRepository.findOne({ where: { email } });
    
    if (!user) {
      // برای امنیت، حتی اگر کاربر وجود نداشته باشد پیام موفقیت بده
      return { resetToken: 'dummy-token' };
    }

    const resetToken = uuidv4();
    const resetTokenExpiry = new Date();
    resetTokenExpiry.setHours(resetTokenExpiry.getHours() + 1); // 1 ساعت اعتبار

    user.resetPasswordToken = resetToken;
    user.resetPasswordExpires = resetTokenExpiry;

    await this.usersRepository.save(user);

    // در اینجا باید ایمیل حاوی resetToken ارسال شود

    return { resetToken };
  }

  /**
   * ریست پسورد با توکن
   */
  async resetPassword(resetToken: string, newPassword: string): Promise<void> {
    const user = await this.usersRepository.findOne({
      where: {
        resetPasswordToken: resetToken,
        resetPasswordExpires: MoreThan(new Date()),
      },
    });

    if (!user) {
      throw new BadRequestException('Invalid or expired reset token');
    }

    const passwordHash = await bcrypt.hash(newPassword, this.SALT_ROUNDS);
    user.passwordHash = passwordHash;
    user.resetPasswordToken = null;
    user.resetPasswordExpires = null;

    // باطل کردن همه refresh tokenهای قبلی
    await this.refreshTokensRepository.delete({ userId: user.id });

    await this.usersRepository.save(user);
  }
}
