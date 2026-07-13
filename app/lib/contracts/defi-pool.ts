/**
 * DeFi Pool Contract Client
 * Wrapper for interacting with the Soroban DeFi liquidity pool contract
 */

import { SorobanContractClient, ContractClientConfig } from './client';

export interface PoolInfo {
  token_a: string;
  token_b: string;
  reserve_a: bigint;
  reserve_b: bigint;
  total_shares: bigint;
  fee_rate: number;
}

export class DeFiPoolContract extends SorobanContractClient {
  constructor(config: ContractClientConfig) {
    super(config);
  }

  /**
   * Get pool information
   */
  async getPoolInfo(): Promise<PoolInfo> {
    return await this.read('get_pool_info');
  }

  /**
   * Get quote for swapping token A to B
   */
  async getQuoteAToB(amountIn: bigint): Promise<bigint> {
    return await this.read('get_quote_a_to_b', amountIn);
  }

  /**
   * Get quote for swapping token B to A
   */
  async getQuoteBToA(amountIn: bigint): Promise<bigint> {
    return await this.read('get_quote_b_to_a', amountIn);
  }

  /**
   * Add liquidity to the pool
   */
  async addLiquidity(
    sourceKeypair: any,
    sender: string,
    amountA: bigint,
    amountB: bigint,
    minShares: bigint
  ): Promise<bigint> {
    return await this.write(
      'add_liquidity',
      sourceKeypair,
      sender,
      amountA,
      amountB,
      minShares
    );
  }

  /**
   * Remove liquidity from the pool
   */
  async removeLiquidity(
    sourceKeypair: any,
    sender: string,
    shares: bigint,
    minAmountA: bigint,
    minAmountB: bigint
  ): Promise<[bigint, bigint]> {
    return await this.write(
      'remove_liquidity',
      sourceKeypair,
      sender,
      shares,
      minAmountA,
      minAmountB
    );
  }

  /**
   * Swap token A for token B
   */
  async swapAToB(
    sourceKeypair: any,
    sender: string,
    amountIn: bigint,
    minAmountOut: bigint
  ): Promise<bigint> {
    return await this.write(
      'swap_a_to_b',
      sourceKeypair,
      sender,
      amountIn,
      minAmountOut
    );
  }

  /**
   * Swap token B for token A
   */
  async swapBToA(
    sourceKeypair: any,
    sender: string,
    amountIn: bigint,
    minAmountOut: bigint
  ): Promise<bigint> {
    return await this.write(
      'swap_b_to_a',
      sourceKeypair,
      sender,
      amountIn,
      minAmountOut
    );
  }

  /**
   * Calculate price impact of a swap
   */
  calculatePriceImpact(
    amountIn: bigint,
    reserveIn: bigint,
    reserveOut: bigint,
    feeRate: number
  ): number {
    const amountInWithFee = amountIn * BigInt(10000 - feeRate) / BigInt(10000);
    const numerator = amountInWithFee * reserveOut;
    const denominator = reserveIn + amountInWithFee;
    const amountOut = numerator / denominator;

    const idealOut = (amountIn * reserveOut) / reserveIn;
    const impact = Number((idealOut - amountOut) * BigInt(10000) / idealOut) / 100;

    return impact;
  }

  /**
   * Calculate APY for liquidity providers (simplified)
   */
  calculateAPY(
    dailyVolume: bigint,
    totalLiquidity: bigint,
    feeRate: number
  ): number {
    const dailyFees = dailyVolume * BigInt(feeRate) / BigInt(10000);
    const dailyReturn = Number(dailyFees) / Number(totalLiquidity);
    const annualReturn = dailyReturn * 365;
    return annualReturn * 100; // Return as percentage
  }
}

/**
 * Calculate optimal liquidity amounts maintaining pool ratio
 */
export function calculateOptimalLiquidity(
  desiredAmountA: bigint,
  desiredAmountB: bigint,
  reserveA: bigint,
  reserveB: bigint
): { amountA: bigint; amountB: bigint } {
  if (reserveA === BigInt(0) || reserveB === BigInt(0)) {
    return { amountA: desiredAmountA, amountB: desiredAmountB };
  }

  const optimalAmountB = (desiredAmountA * reserveB) / reserveA;

  if (optimalAmountB <= desiredAmountB) {
    return { amountA: desiredAmountA, amountB: optimalAmountB };
  } else {
    const optimalAmountA = (desiredAmountB * reserveA) / reserveB;
    return { amountA: optimalAmountA, amountB: desiredAmountB };
  }
}

/**
 * Calculate slippage tolerance
 */
export function calculateMinOutput(
  expectedOutput: bigint,
  slippageBps: number
): bigint {
  return expectedOutput * BigInt(10000 - slippageBps) / BigInt(10000);
}
