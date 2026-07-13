/**
 * Wallet Connect Component
 * Handles connection to Freighter wallet and displays wallet status
 */

import { useState, useEffect } from 'react';
import { Button, Badge } from 'react-bootstrap';

interface WalletConnectProps {
  onConnect?: (publicKey: string) => void;
  onDisconnect?: () => void;
}

export default function WalletConnect({ onConnect, onDisconnect }: WalletConnectProps) {
  const [publicKey, setPublicKey] = useState<string | null>(null);
  const [isConnecting, setIsConnecting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    // Check if wallet is already connected on mount
    checkConnection();
  }, []);

  const checkConnection = async () => {
    if (typeof window === 'undefined') return;

    const freighter = (window as any).freighter;
    if (!freighter) return;

    try {
      const isConnected = await freighter.isConnected();
      if (isConnected) {
        const pubKey = await freighter.getPublicKey();
        setPublicKey(pubKey);
        onConnect?.(pubKey);
      }
    } catch (err) {
      console.error('Error checking wallet connection:', err);
    }
  };

  const handleConnect = async () => {
    setIsConnecting(true);
    setError(null);

    try {
      if (typeof window === 'undefined') {
        throw new Error('Window is undefined');
      }

      const freighter = (window as any).freighter;

      if (!freighter) {
        throw new Error('Freighter wallet not installed. Please install from freighter.app');
      }

      const isAllowed = await freighter.isAllowed();
      if (!isAllowed) {
        await freighter.setAllowed();
      }

      const pubKey = await freighter.getPublicKey();
      setPublicKey(pubKey);
      onConnect?.(pubKey);
    } catch (err: any) {
      setError(err.message || 'Failed to connect wallet');
      console.error('Error connecting wallet:', err);
    } finally {
      setIsConnecting(false);
    }
  };

  const handleDisconnect = () => {
    setPublicKey(null);
    onDisconnect?.();
  };

  const formatAddress = (address: string) => {
    return `${address.slice(0, 6)}...${address.slice(-4)}`;
  };

  if (publicKey) {
    return (
      <div className="d-flex align-items-center gap-2">
        <Badge bg="success">Connected</Badge>
        <code className="text-muted">{formatAddress(publicKey)}</code>
        <Button variant="outline-secondary" size="sm" onClick={handleDisconnect}>
          Disconnect
        </Button>
      </div>
    );
  }

  return (
    <div>
      <Button
        variant="primary"
        onClick={handleConnect}
        disabled={isConnecting}
      >
        {isConnecting ? 'Connecting...' : 'Connect Wallet'}
      </Button>
      {error && (
        <div className="text-danger small mt-2">
          {error}
        </div>
      )}
    </div>
  );
}
