import { registerAs } from '@nestjs/config';

export default registerAs('database', () => ({
  host: process.env.DB_HOST || 'localhost',
  port: parseInt(process.env.DB_PORT || '5432', 10),
  username: process.env.DB_USERNAME || 'crypto_user',
  password: process.env.DB_PASSWORD || 'ChangeMe123!',
  database: process.env.DB_DATABASE || 'crypto_exchange',
  schema: process.env.DB_SCHEMA || 'crypto',
  
  // Connection pool settings
  maxConnections: parseInt(process.env.DB_MAX_CONNECTIONS || '10', 10),
  connectionTimeout: parseInt(process.env.DB_CONNECTION_TIMEOUT || '30000', 10),
  
  // SSL configuration
  ssl: process.env.DB_SSL === 'true' ? { rejectUnauthorized: false } : false,
  
  // TypeORM specific
  type: 'postgres' as const,
  synchronize: process.env.NODE_ENV === 'development',
  logging: process.env.NODE_ENV === 'development',
  
  // Construct connection URL
  url: process.env.DATABASE_URL || `postgresql://${process.env.DB_USERNAME || 'crypto_user'}:${process.env.DB_PASSWORD || 'ChangeMe123!'}@${process.env.DB_HOST || 'localhost'}:${process.env.DB_PORT || '5432'}/${process.env.DB_DATABASE || 'crypto_exchange'}`,
}));
