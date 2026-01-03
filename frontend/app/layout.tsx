import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import './globals.css';
import { Providers } from './providers';

const inter = Inter({ subsets: ['latin'] });

export const metadata: Metadata = {
  title: 'Crypto Options Exchange | Professional Trading Platform',
  description: 'Trade cryptocurrency binary options with professional tools and security',
  keywords: ['crypto', 'options', 'trading', 'bitcoin', 'ethereum', 'tron'],
  authors: [{ name: 'Crypto Options Exchange Team' }],
  openGraph: {
    type: 'website',
    locale: 'en_US',
    url: 'https://trading.yourdomain.com',
    title: 'Crypto Options Exchange',
    description: 'Professional Cryptocurrency Binary Options Trading Platform',
    siteName: 'Crypto Options Exchange',
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body className={inter.className}>
        <Providers>
          <div className="min-h-screen bg-gradient-to-br from-gray-900 to-gray-950">
            {children}
          </div>
        </Providers>
      </body>
    </html>
  );
}
