import { Injectable, UnauthorizedException, Logger } from '@nestjs/common';
import { PassportStrategy } from '@nestjs/passport';
import { ExtractJwt, Strategy } from 'passport-jwt';
import { ConfigService } from '@nestjs/config';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';

import { User } from '../../entities/user.entity';

@Injectable()
export class JwtStrategy extends PassportStrategy(Strategy, 'jwt') {
  private readonly logger = new Logger(JwtStrategy.name);

  constructor(
    private configService: ConfigService,
    @InjectRepository(User)
    private usersRepository: Repository<User>,
  ) {
    const secret = configService.get<string>('JWT_SECRET');
    console.log('🔐 JWT_SECRET from config:', secret ? 'SET' : 'NOT SET');
    
    super({
      jwtFromRequest: ExtractJwt.fromAuthHeaderAsBearerToken(),
      ignoreExpiration: false,
      secretOrKey: secret,
      // issuer و audience را حذف کردیم تا با توکن‌های موجود سازگار باشد
    });
    
    this.logger.log('JWT Strategy initialized successfully');
  }

  async validate(payload: any): Promise<any> {
    console.log('🔍 JwtStrategy.validate() called with payload:', payload);
    
    const userId = payload.sub;
    
    const user = await this.usersRepository.findOne({
      where: { id: userId, isActive: true },
      select: ['id', 'email', 'username', 'isActive', 'isVerified']
    });

    if (!user) {
      throw new UnauthorizedException('User not found or account is inactive');
    }

    console.log('✅ User validated:', user.email);
    
    return {
      sub: user.id,
      email: user.email,
      username: user.username,
      isVerified: user.isVerified,
      ...payload
    };
  }
}