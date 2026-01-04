import { Injectable, ExecutionContext, Logger, UnauthorizedException } from '@nestjs/common';
import { AuthGuard } from '@nestjs/passport';

@Injectable()
export class JwtAuthGuard extends AuthGuard('jwt') {
    private readonly logger = new Logger(JwtAuthGuard.name);

    canActivate(context: ExecutionContext) {
        console.log('🛡️ JwtAuthGuard.canActivate() called');
        this.logger.log('JwtAuthGuard activated for request');

        const request = context.switchToHttp().getRequest();
        const authHeader = request.headers.authorization;
        console.log('📨 Authorization header:', authHeader ? 'Present' : 'Missing');

        if (authHeader) {
            console.log('🔑 Token:', authHeader.substring(0, 50) + '...');
        }

        return super.canActivate(context);
    }

    handleRequest(err, user, info, context) {
        console.log('🔄 JwtAuthGuard.handleRequest()');
        console.log('❌ Error:', err);
        console.log('👤 User:', user);
        console.log('ℹ️ Info:', info);

        if (err || !user) {
            console.log('🚫 Authentication failed');
            throw err || new UnauthorizedException('Authentication failed');
        }

        console.log('✅ Authentication successful for user:', user.email);
        return user;
    }
}