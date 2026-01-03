'use client';

import { useState, useEffect } from 'react';
import { useAuth } from '@/contexts/auth-context';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Slider } from '@/components/ui/slider';
import { toast } from '@/components/ui/use-toast';
import { 
  TrendingUp, 
  TrendingDown, 
  Clock, 
  DollarSign,
  RefreshCw,
  Bitcoin,
  CircleDollarSign
} from 'lucide-react';

interface Asset {
  symbol: string;
  name: string;
  price: number;
  change24h: number;
  icon: React.ReactNode;
}

export default function TradePage() {
  const { user, isAuthenticated } = useAuth();
  const [selectedAsset, setSelectedAsset] = useState('BTC');
  const [amount, setAmount] = useState(10);
  const [duration, setDuration] = useState(60); // seconds
  const [direction, setDirection] = useState<'call' | 'put'>('call');
  const [isPlacingTrade, setIsPlacingTrade] = useState(false);
  const [assets, setAssets] = useState<Asset[]>([
    { 
      symbol: 'BTC', 
      name: 'Bitcoin', 
      price: 45032.50, 
      change24h: 2.34,
      icon: <Bitcoin className="h-5 w-5 text-orange-500" />
    },
    { 
      symbol: 'ETH', 
      name: 'Ethereum', 
      price: 2450.75, 
      change24h: 1.23,
      icon: <CircleDollarSign className="h-5 w-5 text-purple-500" />
    },
    { 
      symbol: 'TRX', 
      name: 'TRON', 
      price: 0.1056, 
      change24h: 0.45,
      icon: <CircleDollarSign className="h-5 w-5 text-red-500" />
    },
    { 
      symbol: 'SOL', 
      name: 'Solana', 
      price: 102.45, 
      change24h: 5.67,
      icon: <CircleDollarSign className="h-5 w-5 text-blue-500" />
    },
  ]);

  const [prices, setPrices] = useState<Record<string, number>>({
    BTC: 45032.50,
    ETH: 2450.75,
    TRX: 0.1056,
    SOL: 102.45,
  });

  const durationOptions = [
    { value: 30, label: '30s' },
    { value: 60, label: '1m' },
    { value: 300, label: '5m' },
    { value: 900, label: '15m' },
    { value: 1800, label: '30m' },
  ];

  const amountOptions = [10, 25, 50, 100, 250, 500];

  const selectedAssetData = assets.find(a => a.symbol === selectedAsset);

  // شبیه‌سازی قیمت‌های زنده
  useEffect(() => {
    const interval = setInterval(() => {
      setPrices(prev => {
        const newPrices = { ...prev };
        Object.keys(newPrices).forEach(symbol => {
          const change = (Math.random() - 0.5) * 0.1; // تغییرات کوچک
          newPrices[symbol] = parseFloat((newPrices[symbol] * (1 + change / 100)).toFixed(2));
        });
        return newPrices;
      });
    }, 2000);

    return () => clearInterval(interval);
  }, []);

  const handlePlaceTrade = async () => {
    if (!isAuthenticated || !user) {
      toast({
        title: 'Authentication required',
        description: 'Please login to place trades',
        variant: 'destructive',
      });
      return;
    }

    if (amount > (user.balance || 0)) {
      toast({
        title: 'Insufficient balance',
        description: You need Green{amount} USDT to place this trade,
        variant: 'destructive',
      });
      return;
    }

    setIsPlacingTrade(true);

    // شبیه‌سازی API call
    setTimeout(() => {
      const isWin = Math.random() > 0.5;
      const profit = isWin ? amount * 0.8 : -amount;
      
      toast({
        title: isWin ? 'Trade Won! 🎉' : 'Trade Lost',
        description: You  Green{Math.abs(profit).toFixed(2)},
        variant: isWin ? 'default' : 'destructive',
      });

      setIsPlacingTrade(false);
    }, 1500);
  };

  const potentialPayout = amount * 1.8; // 80% profit
  const riskAmount = amount;

  if (!isAuthenticated) {
    return (
      <div className="container mx-auto px-4 py-8 text-center">
        <h1 className="text-3xl font-bold mb-4">Trading Platform</h1>
        <p className="text-gray-400 mb-6">
          Please login to start trading
        </p>
        <Button asChild>
          <a href="/auth/login">Login to Trade</a>
        </Button>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Left Column - Asset Selection */}
        <div className="lg:col-span-2 space-y-6">
          <Card className="border-gray-700">
            <CardHeader>
              <CardTitle className="flex items-center justify-between">
                <span>Market Overview</span>
                <Button variant="ghost" size="sm">
                  <RefreshCw className="h-4 w-4" />
                </Button>
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                {assets.map((asset) => (
                  <Card 
                    key={asset.symbol}
                    className={order cursor-pointer transition-all hover:border-blue-500 }
                    onClick={() => setSelectedAsset(asset.symbol)}
                  >
                    <CardContent className="p-4">
                      <div className="flex items-center justify-between mb-2">
                        <div className="flex items-center space-x-2">
                          {asset.icon}
                          <div>
                            <div className="font-semibold">{asset.symbol}</div>
                            <div className="text-xs text-gray-400">{asset.name}</div>
                          </div>
                        </div>
                        <div className={	ext-sm }>
                          {asset.change24h >= 0 ? '+' : ''}{asset.change24h}%
                        </div>
                      </div>
                      <div className="text-xl font-bold">
                        
                      </div>
                    </CardContent>
                  </Card>
                ))}
              </div>
            </CardContent>
          </Card>

          {/* Trading Chart Area */}
          <Card className="border-gray-700 h-96">
            <CardHeader>
              <CardTitle>
                {selectedAssetData?.name} Chart
              </CardTitle>
              <CardDescription>
                Real-time price movements
              </CardDescription>
            </CardHeader>
            <CardContent className="flex items-center justify-center h-64">
              <div className="text-center">
                <div className="text-4xl font-bold gradient-text mb-2">
                  
                </div>
                <p className="text-gray-400">
                  Live {selectedAssetData?.name} Price
                </p>
                <div className="mt-4 text-sm text-gray-500">
                  Chart integration coming soon...
                </div>
              </div>
            </CardContent>
          </Card>
        </div>

        {/* Right Column - Trading Panel */}
        <div>
          <Card className="border-gray-700 sticky top-6">
            <CardHeader>
              <CardTitle>Place Trade</CardTitle>
              <CardDescription>
                Configure your binary options trade
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-6">
              {/* Asset Selection */}
              <div className="space-y-2">
                <Label>Asset</Label>
                <Select value={selectedAsset} onValueChange={setSelectedAsset}>
                  <SelectTrigger>
                    <SelectValue placeholder="Select asset" />
                  </SelectTrigger>
                  <SelectContent>
                    {assets.map((asset) => (
                      <SelectItem key={asset.symbol} value={asset.symbol}>
                        <div className="flex items-center">
                          {asset.icon}
                          <span className="ml-2">{asset.symbol} - {asset.name}</span>
                        </div>
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>

              {/* Amount Selection */}
              <div className="space-y-2">
                <Label>Amount (USDT)</Label>
                <div className="flex space-x-2 mb-2">
                  {amountOptions.map((option) => (
                    <Button
                      key={option}
                      type="button"
                      variant={amount === option ? "default" : "outline"}
                      size="sm"
                      onClick={() => setAmount(option)}
                    >
                      
                    </Button>
                  ))}
                </div>
                <div className="flex items-center space-x-2">
                  <DollarSign className="h-4 w-4 text-gray-400" />
                  <Slider
                    value={[amount]}
                    onValueChange={([value]) => setAmount(value)}
                    min={1}
                    max={1000}
                    step={1}
                    className="flex-1"
                  />
                  <Input
                    type="number"
                    value={amount}
                    onChange={(e) => setAmount(Number(e.target.value))}
                    className="w-24"
                    min={1}
                    max={10000}
                  />
                </div>
              </div>

              {/* Duration Selection */}
              <div className="space-y-2">
                <Label>Duration</Label>
                <div className="flex space-x-2">
                  {durationOptions.map((option) => (
                    <Button
                      key={option.value}
                      type="button"
                      variant={duration === option.value ? "default" : "outline"}
                      size="sm"
                      onClick={() => setDuration(option.value)}
                    >
                      {option.label}
                    </Button>
                  ))}
                </div>
                <div className="flex items-center space-x-2 text-sm text-gray-400">
                  <Clock className="h-4 w-4" />
                  <span>Trade will close in {duration} seconds</span>
                </div>
              </div>

              {/* Direction Selection */}
              <div className="space-y-2">
                <Label>Direction</Label>
                <div className="grid grid-cols-2 gap-2">
                  <Button
                    type="button"
                    variant={direction === 'call' ? 'default' : 'outline'}
                    className={h-16 }
                    onClick={() => setDirection('call')}
                  >
                    <div className="flex flex-col items-center">
                      <TrendingUp className="h-6 w-6 mb-1" />
                      <span>CALL</span>
                      <span className="text-xs">Price will rise</span>
                    </div>
                  </Button>
                  <Button
                    type="button"
                    variant={direction === 'put' ? 'default' : 'outline'}
                    className={h-16 }
                    onClick={() => setDirection('put')}
                  >
                    <div className="flex flex-col items-center">
                      <TrendingDown className="h-6 w-6 mb-1" />
                      <span>PUT</span>
                      <span className="text-xs">Price will fall</span>
                    </div>
                  </Button>
                </div>
              </div>

              {/* Trade Summary */}
              <Card className="bg-gray-900/50">
                <CardContent className="p-4 space-y-2">
                  <div className="flex justify-between">
                    <span className="text-gray-400">Payout:</span>
                    <span className="text-green-400 font-bold"></span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-400">Profit:</span>
                    <span className="text-green-400">+</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-400">Risk:</span>
                    <span className="text-red-400"></span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-400">Win Rate:</span>
                    <span>80%</span>
                  </div>
                </CardContent>
              </Card>

              {/* Place Trade Button */}
              <Button
                className="w-full h-12 text-lg"
                onClick={handlePlaceTrade}
                disabled={isPlacingTrade}
              >
                {isPlacingTrade ? (
                  <>
                    <RefreshCw className="mr-2 h-4 w-4 animate-spin" />
                    Placing Trade...
                  </>
                ) : (
                  Place  Trade for Green{amount}
                )}
              </Button>

              {/* Balance Info */}
              <div className="text-center text-sm text-gray-400">
                Available Balance: <span className="font-semibold"> USDT</span>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}
