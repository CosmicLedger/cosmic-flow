/**
 * Base Soroban Contract Client
 * Provides common utilities for interacting with Soroban smart contracts
 */

import {
  Contract,
  SorobanRpc,
  TransactionBuilder,
  Networks,
  BASE_FEE,
  xdr,
  scValToNative,
  nativeToScVal,
  Address,
} from '@stellar/stellar-sdk';

export interface ContractClientConfig {
  contractId: string;
  rpcUrl: string;
  networkPassphrase: string;
}

export class SorobanContractClient {
  protected contract: Contract;
  protected server: SorobanRpc.Server;
  protected networkPassphrase: string;

  constructor(config: ContractClientConfig) {
    this.contract = new Contract(config.contractId);
    this.server = new SorobanRpc.Server(config.rpcUrl);
    this.networkPassphrase = config.networkPassphrase;
  }

  /**
   * Call a read-only contract function
   */
  async read(method: string, ...params: any[]): Promise<any> {
    try {
      const account = await this.server.getAccount(
        'GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF'
      );

      const scParams = params.map((p) => this.toScVal(p));

      const transaction = new TransactionBuilder(account, {
        fee: BASE_FEE,
        networkPassphrase: this.networkPassphrase,
      })
        .addOperation(this.contract.call(method, ...scParams))
        .setTimeout(30)
        .build();

      const response = await this.server.simulateTransaction(transaction);

      if (SorobanRpc.Api.isSimulationSuccess(response)) {
        return scValToNative(response.result!.retval);
      } else {
        throw new Error(`Simulation failed: ${JSON.stringify(response)}`);
      }
    } catch (error) {
      console.error(`Error reading ${method}:`, error);
      throw error;
    }
  }

  /**
   * Call a state-changing contract function
   */
  async write(
    method: string,
    sourceKeypair: any,
    ...params: any[]
  ): Promise<any> {
    try {
      const sourceAccount = await this.server.getAccount(
        sourceKeypair.publicKey()
      );

      const scParams = params.map((p) => this.toScVal(p));

      let transaction = new TransactionBuilder(sourceAccount, {
        fee: BASE_FEE,
        networkPassphrase: this.networkPassphrase,
      })
        .addOperation(this.contract.call(method, ...scParams))
        .setTimeout(30)
        .build();

      // Simulate to get the transaction data
      const simulated = await this.server.simulateTransaction(transaction);

      if (!SorobanRpc.Api.isSimulationSuccess(simulated)) {
        throw new Error(`Simulation failed: ${JSON.stringify(simulated)}`);
      }

      // Assemble the transaction with simulation results
      const assembled = SorobanRpc.assembleTransaction(
        transaction,
        simulated
      ).build();

      // Sign the transaction
      assembled.sign(sourceKeypair);

      // Submit the transaction
      const sendResponse = await this.server.sendTransaction(assembled);

      if (sendResponse.status === 'PENDING') {
        let getResponse = await this.server.getTransaction(sendResponse.hash);

        // Poll for transaction result
        const retries = 10;
        let attempts = 0;

        while (
          getResponse.status === SorobanRpc.Api.GetTransactionStatus.NOT_FOUND &&
          attempts < retries
        ) {
          await new Promise((resolve) => setTimeout(resolve, 1000));
          getResponse = await this.server.getTransaction(sendResponse.hash);
          attempts++;
        }

        if (getResponse.status === SorobanRpc.Api.GetTransactionStatus.SUCCESS) {
          const result = getResponse.returnValue;
          return result ? scValToNative(result) : null;
        } else {
          throw new Error(`Transaction failed: ${JSON.stringify(getResponse)}`);
        }
      } else {
        throw new Error(`Send failed: ${JSON.stringify(sendResponse)}`);
      }
    } catch (error) {
      console.error(`Error writing ${method}:`, error);
      throw error;
    }
  }

  /**
   * Convert JavaScript values to Soroban ScVal
   */
  protected toScVal(value: any): xdr.ScVal {
    if (typeof value === 'string' && value.startsWith('G')) {
      // Stellar address
      return nativeToScVal(new Address(value), { type: 'address' });
    } else if (typeof value === 'number' || typeof value === 'bigint') {
      return nativeToScVal(value, { type: 'i128' });
    } else if (typeof value === 'string') {
      return nativeToScVal(value, { type: 'string' });
    } else if (typeof value === 'boolean') {
      return nativeToScVal(value, { type: 'bool' });
    } else if (Array.isArray(value)) {
      return nativeToScVal(value, { type: 'bytes' });
    }
    return nativeToScVal(value);
  }

  /**
   * Get contract events
   */
  async getEvents(
    startLedger?: number,
    filters?: any[]
  ): Promise<SorobanRpc.Api.EventResponse> {
    const request: SorobanRpc.Api.GetEventsRequest = {
      filters: filters || [
        {
          type: 'contract',
          contractIds: [this.contract.contractId()],
        },
      ],
      startLedger: startLedger,
    };

    return await this.server.getEvents(request);
  }
}

/**
 * Get network configuration
 */
export function getNetworkConfig(network: 'testnet' | 'mainnet' | 'futurenet') {
  switch (network) {
    case 'testnet':
      return {
        rpcUrl: 'https://soroban-testnet.stellar.org',
        networkPassphrase: Networks.TESTNET,
        horizonUrl: 'https://horizon-testnet.stellar.org',
      };
    case 'futurenet':
      return {
        rpcUrl: 'https://rpc-futurenet.stellar.org',
        networkPassphrase: Networks.FUTURENET,
        horizonUrl: 'https://horizon-futurenet.stellar.org',
      };
    case 'mainnet':
      return {
        rpcUrl: 'https://soroban-mainnet.stellar.org',
        networkPassphrase: Networks.PUBLIC,
        horizonUrl: 'https://horizon.stellar.org',
      };
    default:
      throw new Error(`Unknown network: ${network}`);
  }
}

/**
 * Freighter wallet integration
 */
export async function connectFreighter(): Promise<string | null> {
  if (typeof window === 'undefined') return null;

  const { isConnected, getPublicKey } = (window as any).freighter || {};

  if (!isConnected) {
    throw new Error('Freighter wallet not installed');
  }

  const connected = await isConnected();
  if (!connected) {
    throw new Error('Freighter wallet not connected');
  }

  return await getPublicKey();
}

/**
 * Sign transaction with Freighter
 */
export async function signWithFreighter(
  xdr: string,
  network: string
): Promise<string> {
  if (typeof window === 'undefined') {
    throw new Error('Window is undefined');
  }

  const { signTransaction } = (window as any).freighter || {};

  if (!signTransaction) {
    throw new Error('Freighter wallet not installed');
  }

  return await signTransaction(xdr, network);
}
