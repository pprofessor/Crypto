'use client';

import { useEffect, useState } from 'react';
import { useAuth } from '@/contexts/auth-context';
import { walletApi } from '@/lib/api';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Skeleton } from '@/components/ui/skeleton';
import { 
  TrendingUp, 
  Wallet, 
  ArrowDownCircle, 
  ArrowUpCircle,
  RefreshCw
} from 'lucide-react';
import { toast } from '@/components/ui/use-toast';
import Link from 'next/link';

interface WalletData {
  id: string;
  user_id: string;
  tron_address: string | null;
  usdt_balance: string;
  created_at: string;
  is_active: boolean;
}

export default function DashboardPage() {
  const { user, isAuthenticated, refreshUser } = useAuth();
  const [wallet, setWallet] = useState<WalletData | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [balance, setBalance] = useState('0.00');

  useEffect(() => {
    if (isAuthenticated && user) {
      loadWalletData();
    }
  }, [isAuthenticated, user]);

  const loadWalletData = async () => {
    if (!user) return;
    
    setIsLoading(true);
    try {
      const walletData = await walletApi.getWallet(user.id);
      setWallet(walletData);
      setBalance(parseFloat(walletData.usdt_balance).toFixed(2));
    } catch (error) {
      console.error('Failed to load wallet:', error);
      toast({
        title: 'Error',
        description: 'Failed to load wallet data',
        variant: 'destructive',
      });
    } finally {
      setIsLoading(false);
    }
  };

  const handleRefresh = () => {
    loadWalletData();
    refreshUser();
  };

  if (!isAuthenticated) {
    return null;
  }

  return (
    <div className="container mx-auto px-4 py-8">
      {/* Header */}
      <div className="flex flex-col md:flex-row justify-between items-start md:items-center mb-8">
        <div>
          <h1 className="text-3xl font-bold gradient-text">Dashboard</h1>
          <p className="text-gray-400 mt-2">
            Welcome back, {user?.email}
          </p>
        </div>
        <Button 
          onClick={handleRefresh} 
          variant="outline" 
          className="mt-4 md:mt-0"
          disabled={isLoading}
        >
          <RefreshCw className={mr-2 h-4 w-4 } />
          Refresh
        </Button>
      </div>

      {/* Balance Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        <Card className="bg-gradient-to-br from-gray-800 to-gray-900 border-gray-700">
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium text-gray-300">
              Total Balance
            </CardTitle>
            <Wallet className="h-4 w-4 text-blue-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {isLoading ? (
                <Skeleton className="h-8 w-32" />
              ) : (
                $ USDT
              )}
            </div>
            <p className="text-xs text-gray-400 mt-1">
              Available for trading
            </p>
          </CardContent>
        </Card>

        <Card className="bg-gradient-to-br from-gray-800 to-gray-900 border-gray-700">
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium text-gray-300">
              Today&apos;s Profit
            </CardTitle>
            <TrendingUp className="h-4 w-4 text-green-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-green-400">
              +.00
            </div>
            <p className="text-xs text-gray-400 mt-1">
              From 0 trades
            </p>
          </CardContent>
        </Card>

        <Card className="bg-gradient-to-br from-gray-800 to-gray-900 border-gray-700">
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium text-gray-300">
              Active Trades
            </CardTitle>
            <div className="h-4 w-4 rounded-full bg-yellow-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">0</div>
            <p className="text-xs text-gray-400 mt-1">
              No active positions
            </p>
          </CardContent>
        </Card>

        <Card className="bg-gradient-to-br from-gray-800 to-gray-900 border-gray-700">
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium text-gray-300">
              Win Rate
            </CardTitle>
            <div className="h-4 w-4 rounded-full bg-purple-400" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">0%</div>
            <p className="text-xs text-gray-400 mt-1">
              Based on 0 trades
            </p>
          </CardContent>
        </Card>
      </div>

      {/* Quick Actions */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
        <Card className="border-gray-700">
          <CardHeader>
            <CardTitle className="flex items-center">
              <ArrowDownCircle className="mr-2 h-5 w-5 text-green-400" />
              Deposit Funds
            </CardTitle>
            <CardDescription>
              Add funds to your trading account
            </CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-gray-400 mb-4">
              Deposit USDT via TRON network to start trading
            </p>
            <Link href="/wallet/deposit">
              <Button className="w-full bg-gradient-to-r from-green-500 to-emerald-600 hover:from-green-600 hover:to-emerald-700">
                Deposit Now
              </Button>
            </Link>
          </CardContent>
        </Card>

        <Card className="border-gray-700">
          <CardHeader>
            <CardTitle className="flex items-center">
              <ArrowUpCircle className="mr-2 h-5 w-5 text-red-400" />
              Withdraw Funds
            </CardTitle>
            <CardDescription>
              Withdraw your profits
            </CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-gray-400 mb-4">
              Withdraw USDT to your external wallet
            </p>
            <Link href="/wallet/withdraw">
              <Button variant="outline" className="w-full">
                Withdraw Now
              </Button>
            </Link>
          </CardContent>
        </Card>

        <Card className="border-gray-700">
          <CardHeader>
            <CardTitle className="flex items-center">
              <TrendingUp className="mr-2 h-5 w-5 text-blue-400" />
              Start Trading
            </CardTitle>
            <CardDescription>
              Begin trading binary options
            </CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-gray-400 mb-4">
              Trade Bitcoin, Ethereum, and other cryptocurrencies
            </p>
            <Link href="/trade">
              <Button className="w-full bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700">
                Go to Trading
              </Button>
            </Link>
          </CardContent>
        </Card>
      </div>

      {/* Wallet Information */}
      <Card className="border-gray-700">
        <CardHeader>
          <CardTitle>Wallet Information</CardTitle>
          <CardDescription>
            Your TRON wallet details
          </CardDescription>
        </CardHeader>
        <CardContent>
          {isLoading ? (
            <div className="space-y-4">
              <Skeleton className="h-4 w-full" />
              <Skeleton className="h-4 w-3/4" />
              <Skeleton className="h-4 w-1/2" />
            </div>
          ) : wallet ? (
            <div className="space-y-4">
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <p className="text-sm text-gray-400">Wallet Status</p>
                  <p className={ont-medium }>
                    {wallet.is_active ? 'Active' : 'Pending Activation'}
                  </p>
                </div>
                <div>
                  <p className="text-sm text-gray-400">Created Date</p>
                  <p className="font-medium">
                    {new Date(wallet.created_at).toLocaleDateString()}
                  </p>
                </div>
              </div>
              {wallet.tron_address && (
                <div>
                  <p className="text-sm text-gray-400 mb-1">TRON Address</p>
                  <div className="bg-gray-800 rounded-lg p-3 font-mono text-sm break-all">
                    {wallet.tron_address}
                  </div>
                  <p className="text-xs text-gray-400 mt-1">
                    Use this address for deposits
                  </p>
                </div>
              )}
            </div>
          ) : (
            <p className="text-gray-400">No wallet information available</p>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
