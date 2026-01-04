import { Injectable, UnauthorizedException } from '@nestjs/common';
import { PassportStrategy } from '@nestjs/passport';
import { ExtractJwt, Strategy } from 'passport-jwt';
import { ConfigService } from '@nestjs/config';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';

import { User } from '../../entities/user.entity';

@Injectable()
export class JwtStrategy extends PassportStrategy(Strategy, 'jwt') {
  constructor(
    private configService: ConfigService,
    @InjectRepository(User)
    private usersRepository: Repository<User>,
  ) {
    super({
      jwtFromRequest: ExtractJwt.fromExtractors([
        ExtractJwt.fromAuthHeaderAsBearerToken(),
        ExtractJwt.fromUrlQueryParameter('token'),
      ]),
      ignoreExpiration: false,
      secretOrKey: configService.get<string>('JWT_SECRET'),
      issuer: 'crypto-exchange',
      audience: 'crypto-users',
    });
  }

  async validate(payload: any): Promise<any> {
    const user = await this.usersRepository.findOne({
      where: { id: payload.sub, isActive: true },
      select: [
        'id', 'email', 'username', 'firstName', 'lastName',
        'phoneNumber', 'isActive', 'isVerified', 'is2faEnabled',
        'userType', 'lastLoginAt', 'createdAt', 'updatedAt',
        'kycStatus', 'dailyWithdrawalLimit', 'monthlyTradeLimit'
      ]
    });

    if (!user) {
      throw new UnauthorizedException('User not found or account is inactive');
    }

    // اگر 2FA فعال است، بررسی کن که payload حاوی 2FA claim باشد
    if (user.is2faEnabled && !payload.is2faVerified) {
      throw new UnauthorizedException('2FA verification required');
    }

    // اگر حساب تأیید نشده باشد (در موارد خاص)
    if (!user.isVerified && payload.requireVerification !== false) {
      throw new UnauthorizedException('Email verification required');
    }

    // payload نهایی
    return {
      sub: user.id,
      email: user.email,
      username: user.username,
      userType: user.userType,
      isVerified: user.isVerified,
      is2faEnabled: user.is2faEnabled,
      kycStatus: user.kycStatus,
      ...payload // سایر claims از توکن اصلی
    };
  }
}