import { 
  Injectable, 
  UnauthorizedException, 
  ConflictException, 
  BadRequestException,
  Logger 
} from '@nestjs/common';
import { JwtService } from '@nestjs/jwt';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository, MoreThan } from 'typeorm';
import * as bcrypt from 'bcrypt';
import { v4 as uuidv4 } from 'uuid';

import { User } from '../entities/user.entity';
import { RefreshToken } from './entities/refresh-token.entity';
import { RegisterDto } from './dto/register.dto';
import { LoginDto } from './dto/login.dto';
import { ChangePasswordDto } from './dto/change-password.dto';
import { RefreshTokenDto } from './dto/refresh-token.dto';

@Injectable()
export class AuthService {
  private readonly logger = new Logger(AuthService.name);
  private readonly SALT_ROUNDS = parseInt(process.env.BCRYPT_SALT_ROUNDS || '10', 10);

  constructor(
    @InjectRepository(User)
    private usersRepository: Repository<User>,
    @InjectRepository(RefreshToken)
    private refreshTokensRepository: Repository<RefreshToken>,
    private readonly jwtService: JwtService,
  ) {}

  /**
   * ثبت‌نام کاربر جدید
   */
  async register(registerDto: RegisterDto): Promise<{ 
    user: Partial<User>; 
    accessToken: string; 
    refreshToken: string; 
  }> {
    this.logger.log(`Register attempt for email: ${registerDto.email}`);
    
    // بررسی وجود کاربر با ایمیل یا نام کاربری مشابه
    const existingUser = await this.usersRepository.findOne({
      where: [
        { email: registerDto.email },
        { username: registerDto.username }
      ],
      withDeleted: true, // حتی کاربران حذف‌شده را هم بررسی کن
    });

    if (existingUser) {
      if (existingUser.email === registerDto.email) {
        throw new ConflictException('User with this email already exists');
      }
      if (existingUser.username === registerDto.username) {
        throw new ConflictException('Username already taken');
      }
    }

    // هش کردن پسورد
    const passwordHash = await bcrypt.hash(registerDto.password, this.SALT_ROUNDS);

    // ایجاد کاربر جدید
    const user = this.usersRepository.create({
      email: registerDto.email,
      username: registerDto.username,
      passwordHash,
      firstName: registerDto.firstName,
      lastName: registerDto.lastName,
      phoneNumber: registerDto.phoneNumber,
      isVerified: false, // نیاز به تأیید ایمیل
      isActive: true,
      userType: 'regular',
    });

    await this.usersRepository.save(user);
    this.logger.log(`User registered successfully: ${user.id}`);

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
  async login(loginDto: LoginDto): Promise<{ 
    user: Partial<User>; 
    accessToken: string; 
    refreshToken: string; 
  }> {
    this.logger.log(`Login attempt for: ${loginDto.email || loginDto.username}`);
    
    const user = await this.usersRepository.findOne({
      where: [
        { email: loginDto.email || '' },
        { username: loginDto.username || '' }
      ],
    });

    if (!user) {
      this.logger.warn(`Login failed: User not found`);
      throw new UnauthorizedException('Invalid credentials');
    }

    // بررسی اگر حساب غیرفعال باشد
    if (!user.isActive) {
      throw new UnauthorizedException('Account is disabled. Please contact support.');
    }

    // بررسی اگر حساب قفل شده باشد (5 بار ورود ناموفق)
    if (user.failedLoginAttempts >= 5) {
      throw new UnauthorizedException('Account is locked due to too many failed attempts. Please reset your password.');
    }

    // بررسی پسورد
    const isPasswordValid = await bcrypt.compare(loginDto.password, user.passwordHash);
    
    if (!isPasswordValid) {
      // افزایش شمارنده ورود ناموفق
      user.failedLoginAttempts += 1;
      await this.usersRepository.save(user);
      
      this.logger.warn(`Failed login attempt for user: ${user.id}. Attempts: ${user.failedLoginAttempts}`);
      throw new UnauthorizedException('Invalid credentials');
    }

    // ریست شمارنده ورود ناموفق
    user.failedLoginAttempts = 0;
    user.lastLoginAt = new Date();
    await this.usersRepository.save(user);

    // تولید توکن‌ها
    const tokens = await this.generateTokens(user);

    this.logger.log(`User logged in successfully: ${user.id}`);

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

    // بررسی نکردن مجدد آخرین پسورد
    if (user.lastPasswordHash) {
      const isSameAsLast = await bcrypt.compare(changePasswordDto.newPassword, user.lastPasswordHash);
      if (isSameAsLast) {
        throw new BadRequestException('Cannot reuse your last password');
      }
    }

    // ذخیره پسورد فعلی به عنوان آخرین پسورد
    user.lastPasswordHash = user.passwordHash;
    
    // هش کردن پسورد جدید
    user.passwordHash = await bcrypt.hash(changePasswordDto.newPassword, this.SALT_ROUNDS);
    user.passwordChangedAt = new Date();

    // باطل کردن همه refresh tokenهای قبلی
    await this.refreshTokensRepository.delete({ userId: user.id });

    await this.usersRepository.save(user);
    this.logger.log(`Password changed for user: ${userId}`);
  }

  /**
   * ریفرش توکن
   */
  async refreshToken(refreshTokenDto: RefreshTokenDto): Promise<{ 
    accessToken: string; 
    refreshToken: string; 
  }> {
    const tokenRecord = await this.refreshTokensRepository.findOne({
      where: { 
        token: refreshTokenDto.refreshToken, 
        isRevoked: false 
      },
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

    // بررسی کاربر هنوز فعال است
    if (!tokenRecord.user.isActive) {
      throw new UnauthorizedException('User account is no longer active');
    }

    // تولید توکن‌های جدید
    const tokens = await this.generateTokens(tokenRecord.user);

    // غیرفعال کردن توکن قدیمی
    await this.refreshTokensRepository.update(tokenRecord.id, { isRevoked: true });

    this.logger.log(`Token refreshed for user: ${tokenRecord.userId}`);
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
      this.logger.log(`Specific token revoked for user: ${userId}`);
    } else {
      // غیرفعال کردن همه توکن‌های کاربر
      await this.refreshTokensRepository.update(
        { userId, isRevoked: false },
        { isRevoked: true },
      );
      this.logger.log(`All tokens revoked for user: ${userId}`);
    }
  }

  /**
   * تولید access و refresh token
   */
  private async generateTokens(user: User): Promise<{ 
    accessToken: string; 
    refreshToken: string; 
  }> {
    const payload = {
      sub: user.id,
      email: user.email,
      username: user.username,
      userType: user.userType,
    };

    const accessToken = await this.jwtService.signAsync(payload, {
      secret: process.env.JWT_SECRET,
      expiresIn: process.env.JWT_EXPIRATION || '1d',
    });

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
  private sanitizeUser(user: User): Partial<User> {
    const { 
      passwordHash, 
      twoFactorSecret, 
      recoveryCodes,
      lastPasswordHash,
      passwordChangedAt,
      ...sanitizedUser 
    } = user;
    return sanitizedUser;
  }

  /**
   * تأیید ایمیل
   */
  async verifyEmail(userId: string): Promise<void> {
    const result = await this.usersRepository.update(userId, { 
      isVerified: true,
      kycStatus: 'PENDING' // بعد از تأیید ایمیل، KYC در حالت pending قرار می‌گیرد
    });

    if (result.affected === 0) {
      throw new BadRequestException('User not found');
    }
    
    this.logger.log(`Email verified for user: ${userId}`);
  }

  /**
   * درخواست ریست پسورد
   */
  async requestPasswordReset(email: string): Promise<{ resetToken: string }> {
    const user = await this.usersRepository.findOne({ 
      where: { email },
      withDeleted: true, // حتی اگر کاربر حذف منطقی شده باشد
    });
    
    if (!user) {
      // برای امنیت، حتی اگر کاربر وجود نداشته باشد پیام موفقیت بده
      this.logger.log(`Password reset requested for non-existent email: ${email}`);
      return { resetToken: 'dummy-token-for-security' };
    }

    // اگر حساب غیرفعال باشد، اجازه نده
    if (!user.isActive && user.deletedAt) {
      throw new BadRequestException('Account is deactivated. Please contact support.');
    }

    const resetToken = uuidv4();
    const resetTokenExpiry = new Date();
    resetTokenExpiry.setHours(resetTokenExpiry.getHours() + 1); // 1 ساعت اعتبار

    // در entity فعلی فیلد reset token نداریم، باید اضافه کنیم یا جایگزین کنیم
    // فعلاً لاگ می‌کنیم
    this.logger.log(`Password reset token generated for ${email}: ${resetToken}`);
    
    // TODO: ارسال ایمیل با لینک ریست
    
    return { resetToken };
  }

  /**
   * اعتبارسنجی توکن ریست پسورد
   */
  async validateResetToken(resetToken: string): Promise<boolean> {
    // TODO: پیاده‌سازی منطق اعتبارسنجی توکن
    // فعلاً فرض می‌کنیم معتبر است
    this.logger.log(`Validating reset token: ${resetToken}`);
    return true;
  }

  /**
   * ریست پسورد
   */
  async resetPassword(resetToken: string, newPassword: string): Promise<void> {
    // TODO: پیاده‌سازی منطق کامل ریست پسورد
    // فعلاً فقط لاگ می‌کنیم
    this.logger.log(`Password reset with token: ${resetToken}`);
    
    // در production باید:
    // 1. اعتبارسنجی توکن
    // 2. پیدا کردن کاربر
    // 3. تغییر پسورد
    // 4. باطل کردن توکن‌های قدیمی
    // 5. ارسال تأییدیه
    
    throw new BadRequestException('Password reset not fully implemented yet');
  }

  /**
   * دریافت پروفایل کاربر
   */
  async getProfile(userId: string): Promise<Partial<User>> {
    const user = await this.usersRepository.findOne({
      where: { id: userId },
    });

    if (!user) {
      throw new UnauthorizedException('User not found');
    }

    return this.sanitizeUser(user);
  }

  /**
   * به‌روزرسانی پروفایل
   */
  async updateProfile(userId: string, updateData: Partial<User>): Promise<Partial<User>> {
    // حذف فیلدهای غیرقابل ویرایش
    const { id, email, passwordHash, isActive, isVerified, userType, ...allowedUpdates } = updateData;
    
    const result = await this.usersRepository.update(userId, allowedUpdates);
    
    if (result.affected === 0) {
      throw new BadRequestException('User not found or no changes made');
    }

    const updatedUser = await this.usersRepository.findOne({
      where: { id: userId },
    });

    return this.sanitizeUser(updatedUser);
  }
}
