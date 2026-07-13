/**
 * Token Balance Component
 * Displays token balance for a connected wallet
 */

import { useState, useEffect } from 'react';
import { Card, Spinner } from 'react-bootstrap';
import { TokenContract, formatTokenAmount } from '~/lib/contracts/token';
import { getNetworkConfig } from '~/lib/contracts/client';

interface TokenBalanceProps {
  contractId: string;
  accountAddress: string;
  network?: 'testnet' | 'mainnet' | 'futurenet';
}

export default function TokenBalance({
  contractId,
  accountAddress,
  network = 'testnet',
}: TokenBalanceProps) {
  const [balance, setBalance] = useState<bigint | null>(null);
  const [tokenInfo, setTokenInfo] = useState<{
    name: string;
    symbol: string;
    decimals: number;
  } | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetchTokenData();
  }, [contractId, accountAddress, network]);

  const fetchTokenData = async () => {
    setLoading(true);
    setError(null);

    try {
      const config = getNetworkConfig(network);
      const tokenContract = new TokenContract({
        contractId,
        rpcUrl: config.rpcUrl,
        networkPassphrase: config.networkPassphrase,
      });

      const [bal, name, symbol, decimals] = await Promise.all([
        tokenContract.balance(accountAddress),
        tokenContract.name(),
        tokenContract.symbol(),
        tokenContract.decimals(),
      ]);

      setBalance(bal);
      setTokenInfo({ name, symbol, decimals });
    } catch (err: any) {
      setError(err.message || 'Failed to fetch token data');
      console.error('Error fetching token data:', err);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <Card>
        <Card.Body className="text-center">
          <Spinner animation="border" size="sm" />
          <div className="mt-2">Loading token balance...</div>
        </Card.Body>
      </Card>
    );
  }

  if (error) {
    return (
      <Card border="danger">
        <Card.Body>
          <div className="text-danger">Error: {error}</div>
        </Card.Body>
      </Card>
    );
  }

  if (!tokenInfo || balance === null) {
    return null;
  }

  const formattedBalance = formatTokenAmount(balance, tokenInfo.decimals);

  return (
    <Card>
      <Card.Body>
        <h5>{tokenInfo.name}</h5>
        <div className="d-flex justify-content-between align-items-center">
          <div>
            <div className="h3 mb-0">{formattedBalance}</div>
            <div className="text-muted small">{tokenInfo.symbol}</div>
          </div>
          <div className="text-end">
            <div className="small text-muted">Contract</div>
            <code className="small">{contractId.slice(0, 8)}...</code>
          </div>
        </div>
      </Card.Body>
    </Card>
  );
}
