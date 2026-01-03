import { Entity, PrimaryGeneratedColumn, Column, CreateDateColumn, UpdateDateColumn } from 'typeorm';
import { Exclude } from 'class-transformer';

@Entity('users')
export class User {
    @PrimaryGeneratedColumn('uuid')
    id: string;

    @Column({ unique: true, length: 255 })
    email: string;

    @Exclude() // مخفی کردن در responses
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

    @CreateDateColumn({ name: 'created_at' })
    createdAt: Date;

    @UpdateDateColumn({ name: 'updated_at' })
    updatedAt: Date;

    constructor(partial: Partial<User>) {
        Object.assign(this, partial);
    }
}
