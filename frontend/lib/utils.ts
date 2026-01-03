import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function formatCurrency(amount: number | string, currency: string = "USD"): string {
  const numAmount = typeof amount === "string" ? parseFloat(amount) : amount
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency,
    minimumFractionDigits: 2,
    maximumFractionDigits: 8,
  }).format(numAmount)
}

export function formatDate(date: string | Date): string {
  return new Date(date).toLocaleDateString("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  })
}

export function truncateAddress(address: string, start: number = 6, end: number = 4): string {
  if (address.length <= start + end) return address
  return ${address.slice(0, start)}...
}

export function generateTradingId(): string {
  return TRD
}

export function calculateProfit(entryPrice: number, exitPrice: number, amount: number, direction: "call" | "put"): number {
  if (direction === "call") {
    return exitPrice > entryPrice ? amount * 0.8 : -amount // 80% profit on win, 100% loss on lose
  } else {
    return exitPrice < entryPrice ? amount * 0.8 : -amount
  }
}

export function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms))
}

export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeout: NodeJS.Timeout
  return (...args: Parameters<T>) => {
    clearTimeout(timeout)
    timeout = setTimeout(() => func(...args), wait)
  }
}
