import { IsString, MinLength, IsOptional, ValidateIf } from 'class-validator';

export class LoginDto {
  @ValidateIf(o => !o.username)
  @IsString()
  email?: string;

  @ValidateIf(o => !o.email)
  @IsString()
  @MinLength(3)
  username?: string;

  @IsString()
  @MinLength(8)
  password: string;
}