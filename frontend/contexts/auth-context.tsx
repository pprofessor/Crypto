'use client';

import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { useRouter, usePathname } from 'next/navigation';
import { apiClient, authApi, getCurrentUserId, isAuthenticated as checkIsAuthenticated } from '@/lib/api';

interface User {
  id: string;
  email: string;
  isVerified: boolean;
  balance: number;
  createdAt: string;
}

interface AuthContextType {
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  login: (email: string, password: string) => Promise<void>;
  register: (email: string, password: string, passwordConfirmation: string) => Promise<void>;
  logout: () => Promise<void>;
  refreshUser: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
}

interface AuthProviderProps {
  children: ReactNode;
}

export function AuthProvider({ children }: AuthProviderProps) {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const router = useRouter();
  const pathname = usePathname();

  const isAuthenticated = !!user;

  // بررسی وضعیت احراز هویت در اولین رندر
  useEffect(() => {
    checkAuthStatus();
  }, []);

  // ریدایرکت اگر کاربر لاگین نباشد و در صفحه protected باشد
  useEffect(() => {
    const protectedRoutes = ['/dashboard', '/wallet', '/trade'];
    const authRoutes = ['/auth/login', '/auth/register'];
    
    if (!isLoading) {
      const isProtectedRoute = protectedRoutes.some(route => pathname?.startsWith(route));
      const isAuthRoute = authRoutes.some(route => pathname?.startsWith(route));
      
      if (isProtectedRoute && !isAuthenticated) {
        router.push('/auth/login');
      } else if (isAuthRoute && isAuthenticated) {
        router.push('/dashboard');
      }
    }
  }, [pathname, isAuthenticated, isLoading, router]);

  const checkAuthStatus = async () => {
    setIsLoading(true);
    
    if (checkIsAuthenticated()) {
      try {
        await refreshUser();
      } catch (error) {
        console.error('Failed to refresh user:', error);
        apiClient.clearAuth();
      }
    }
    
    setIsLoading(false);
  };

  const login = async (email: string, password: string) => {
    setIsLoading(true);
    
    try {
      const response = await authApi.login({ email, password });
      
      apiClient.setAuthToken(response.accessToken);
      apiClient.setRefreshToken(response.refreshToken);
      
      setUser(response.user);
      router.push('/dashboard');
    } catch (error: any) {
      throw new Error(error.response?.data?.message || 'Login failed');
    } finally {
      setIsLoading(false);
    }
  };

  const register = async (email: string, password: string, passwordConfirmation: string) => {
    setIsLoading(true);
    
    try {
      const response = await authApi.register({ email, password, passwordConfirmation });
      
      apiClient.setAuthToken(response.accessToken);
      apiClient.setRefreshToken(response.refreshToken);
      
      setUser(response.user);
      router.push('/dashboard');
    } catch (error: any) {
      throw new Error(error.response?.data?.message || 'Registration failed');
    } finally {
      setIsLoading(false);
    }
  };

  const logout = async () => {
    setIsLoading(true);
    
    try {
      const refreshToken = localStorage.getItem('refresh_token');
      await authApi.logout(refreshToken || undefined);
    } catch (error) {
      console.error('Logout error:', error);
    } finally {
      apiClient.clearAuth();
      setUser(null);
      setIsLoading(false);
      router.push('/auth/login');
    }
  };

  const refreshUser = async () => {
    try {
      const userData = await authApi.getProfile();
      setUser(userData);
    } catch (error) {
      throw error;
    }
  };

  const value = {
    user,
    isAuthenticated,
    isLoading,
    login,
    register,
    logout,
    refreshUser,
  };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}
