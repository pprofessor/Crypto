import axios, { AxiosInstance, AxiosRequestConfig } from 'axios';

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000';

class ApiClient {
  private client: AxiosInstance;
  private authToken: string | null = null;

  constructor() {
    this.client = axios.create({
      baseURL: API_BASE_URL,
      timeout: 30000,
      headers: {
        'Content-Type': 'application/json',
      },
    });

    // Load token from localStorage on initialization
    if (typeof window !== 'undefined') {
      this.authToken = localStorage.getItem('auth_token');
      if (this.authToken) {
        this.setAuthToken(this.authToken);
      }
    }

    // Request interceptor
    this.client.interceptors.request.use(
      (config) => {
        if (this.authToken) {
          config.headers.Authorization = Bearer ;
        }
        return config;
      },
      (error) => Promise.reject(error)
    );

    // Response interceptor
    this.client.interceptors.response.use(
      (response) => response,
      async (error) => {
        const originalRequest = error.config;

        // Handle 401 Unauthorized
        if (error.response?.status === 401 && !originalRequest._retry) {
          originalRequest._retry = true;

          try {
            const refreshToken = localStorage.getItem('refresh_token');
            if (refreshToken) {
              const { data } = await this.post('/auth/refresh', {
                refreshToken,
              });

              this.setAuthToken(data.accessToken);
              localStorage.setItem('refresh_token', data.refreshToken);

              originalRequest.headers.Authorization = Bearer ;
              return this.client(originalRequest);
            }
          } catch (refreshError) {
            // Clear tokens and redirect to login
            this.clearAuth();
            window.location.href = '/auth/login';
          }
        }

        return Promise.reject(error);
      }
    );
  }

  setAuthToken(token: string) {
    this.authToken = token;
    if (typeof window !== 'undefined') {
      localStorage.setItem('auth_token', token);
    }
  }

  setRefreshToken(token: string) {
    if (typeof window !== 'undefined') {
      localStorage.setItem('refresh_token', token);
    }
  }

  clearAuth() {
    this.authToken = null;
    if (typeof window !== 'undefined') {
      localStorage.removeItem('auth_token');
      localStorage.removeItem('refresh_token');
    }
  }

  getAuthToken(): string | null {
    return this.authToken;
  }

  async get<T>(url: string, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.client.get(url, config);
    return response.data;
  }

  async post<T>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.client.post(url, data, config);
    return response.data;
  }

  async put<T>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.client.put(url, data, config);
    return response.data;
  }

  async patch<T>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.client.patch(url, data, config);
    return response.data;
  }

  async delete<T>(url: string, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.client.delete(url, config);
    return response.data;
  }
}

// Authentication API
export const authApi = {
  register: (data: { email: string; password: string; passwordConfirmation: string }) =>
    apiClient.post<{ user: any; accessToken: string; refreshToken: string }>('/auth/register', data),

  login: (data: { email: string; password: string }) =>
    apiClient.post<{ user: any; accessToken: string; refreshToken: string }>('/auth/login', data),

  logout: (refreshToken?: string) =>
    apiClient.post('/auth/logout', { refreshToken }),

  changePassword: (data: { oldPassword: string; newPassword: string; newPasswordConfirmation: string }) =>
    apiClient.post('/auth/change-password', data),

  getProfile: () =>
    apiClient.get<any>('/auth/profile'),

  refreshToken: (refreshToken: string) =>
    apiClient.post<{ accessToken: string; refreshToken: string }>('/auth/refresh', { refreshToken }),
};

// Wallet API
export const walletApi = {
  getWallet: (userId: string) =>
    apiClient.get<any>(/api/v1/wallets/),

  getBalance: (userId: string) =>
    apiClient.get<{ user_id: string; usdt_balance: string; last_updated: string }>(
      /api/v1/wallets//balance
    ),

  createDeposit: (data: { user_id: string; amount: string; payment_method: string }) =>
    apiClient.post<any>('/api/v1/deposits', data),

  getUserDeposits: (userId: string, params?: { limit?: number; offset?: number }) =>
    apiClient.get<any[]>(/api/v1/deposits/user/, { params }),

  getDepositStatus: (depositId: string) =>
    apiClient.get<any>(/api/v1/deposits//status),
};

// Export singleton instance
export const apiClient = new ApiClient();

// Helper function to check if user is authenticated
export const isAuthenticated = (): boolean => {
  if (typeof window === 'undefined') return false;
  return !!localStorage.getItem('auth_token');
};

// Helper function to get current user ID from token
export const getCurrentUserId = (): string | null => {
  if (typeof window === 'undefined') return null;
  
  const token = localStorage.getItem('auth_token');
  if (!token) return null;

  try {
    const payload = JSON.parse(atob(token.split('.')[1]));
    return payload.sub || null;
  } catch {
    return null;
  }
};
