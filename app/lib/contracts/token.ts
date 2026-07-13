/**
 * Token Contract Client
 * Wrapper for interacting with the Soroban Token contract
 */

import { SorobanContractClient, ContractClientConfig } from './client';

export class TokenContract extends SorobanContractClient {
  constructor(config: ContractClientConfig) {
    super(config);
  }

  /**
   * Get token balance for an account
   */
  async balance(account: string): Promise<bigint> {
    return await this.read('balance', account);
  }

  /**
   * Get total token supply
   */
  async totalSupply(): Promise<bigint> {
    return await this.read('total_supply');
  }

  /**
   * Get token name
   */
  async name(): Promise<string> {
    return await this.read('name');
  }

  /**
   * Get token symbol
   */
  async symbol(): Promise<string> {
    return await this.read('symbol');
  }

  /**
   * Get token decimals
   */
  async decimals(): Promise<number> {
    return await this.read('decimals');
  }

  /**
   * Get allowance
   */
  async allowance(owner: string, spender: string): Promise<bigint> {
    return await this.read('allowance', owner, spender);
  }

  /**
   * Transfer tokens
   */
  async transfer(
    sourceKeypair: any,
    from: string,
    to: string,
    amount: bigint
  ): Promise<void> {
    await this.write('transfer', sourceKeypair, from, to, amount);
  }

  /**
   * Approve spender
   */
  async approve(
    sourceKeypair: any,
    from: string,
    spender: string,
    amount: bigint
  ): Promise<void> {
    await this.write('approve', sourceKeypair, from, spender, amount);
  }

  /**
   * Transfer from (using allowance)
   */
  async transferFrom(
    sourceKeypair: any,
    spender: string,
    from: string,
    to: string,
    amount: bigint
  ): Promise<void> {
    await this.write('transfer_from', sourceKeypair, spender, from, to, amount);
  }

  /**
   * Mint new tokens (admin only)
   */
  async mint(
    sourceKeypair: any,
    to: string,
    amount: bigint
  ): Promise<void> {
    await this.write('mint', sourceKeypair, to, amount);
  }

  /**
   * Burn tokens
   */
  async burn(
    sourceKeypair: any,
    from: string,
    amount: bigint
  ): Promise<void> {
    await this.write('burn', sourceKeypair, from, amount);
  }
}

/**
 * Format token amount with decimals
 */
export function formatTokenAmount(
  amount: bigint,
  decimals: number
): string {
  const divisor = BigInt(10 ** decimals);
  const integerPart = amount / divisor;
  const fractionalPart = amount % divisor;
  
  if (fractionalPart === BigInt(0)) {
    return integerPart.toString();
  }
  
  const fractionalStr = fractionalPart.toString().padStart(decimals, '0');
  return `${integerPart}.${fractionalStr}`.replace(/\.?0+$/, '');
}

/**
 * Parse token amount from string
 */
export function parseTokenAmount(
  amountStr: string,
  decimals: number
): bigint {
  const [integerPart, fractionalPart = ''] = amountStr.split('.');
  const paddedFractional = fractionalPart.padEnd(decimals, '0').slice(0, decimals);
  const combined = integerPart + paddedFractional;
  return BigInt(combined);
}
