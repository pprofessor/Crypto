import {
    IsString,
    MaxLength,
    IsOptional,
    IsPhoneNumber,
    IsArray,
    IsEnum
} from 'class-validator';

export class UpdateProfileDto {
    @IsString()
    @MaxLength(100)
    @IsOptional()
    firstName?: string;

    @IsString()
    @MaxLength(100)
    @IsOptional()
    lastName?: string;

    @IsPhoneNumber()
    @IsOptional()
    phoneNumber?: string;

    @IsArray()
    @IsString({ each: true })
    @IsOptional()
    withdrawalWhitelist?: string[];

    @IsEnum(['regular', 'vip', 'admin'])
    @IsOptional()
    userType?: string;

    @IsOptional()
    isActive?: boolean;

    @IsOptional()
    is2faEnabled?: boolean;

    @IsOptional()
    dailyWithdrawalLimit?: number;

    @IsOptional()
    monthlyTradeLimit?: number;
}