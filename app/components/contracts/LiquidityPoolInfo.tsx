/**
 * Liquidity Pool Info Component
 * Displays pool information and allows adding/removing liquidity
 */

import { useState, useEffect } from 'react';
import { Card, Row, Col, Button, Form, InputGroup, Spinner, Alert } from 'react-bootstrap';
import { DeFiPoolContract, calculateOptimalLiquidity } from '~/lib/contracts/defi-pool';
import { getNetworkConfig } from '~/lib/contracts/client';

interface LiquidityPoolInfoProps {
  contractId: string;
  userAddress?: string;
  network?: 'testnet' | 'mainnet' | 'futurenet';
}

export default function LiquidityPoolInfo({
  contractId,
  userAddress,
  network = 'testnet',
}: LiquidityPoolInfoProps) {
  const [poolInfo, setPoolInfo] = useState<any>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [amountA, setAmountA] = useState('');
  const [amountB, setAmountB] = useState('');
  const [quote, setQuote] = useState<bigint | null>(null);

  useEffect(() => {
    fetchPoolInfo();
  }, [contractId, network]);

  const fetchPoolInfo = async () => {
    setLoading(true);
    setError(null);

    try {
      const config = getNetworkConfig(network);
      const poolContract = new DeFiPoolContract({
        contractId,
        rpcUrl: config.rpcUrl,
        networkPassphrase: config.networkPassphrase,
      });

      const info = await poolContract.getPoolInfo();
      setPoolInfo(info);
    } catch (err: any) {
      setError(err.message || 'Failed to fetch pool info');
      console.error('Error fetching pool info:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleGetQuote = async () => {
    if (!amountA || !poolInfo) return;

    try {
      const config = getNetworkConfig(network);
      const poolContract = new DeFiPoolContract({
        contractId,
        rpcUrl: config.rpcUrl,
        networkPassphrase: config.networkPassphrase,
      });

      const amountInBigInt = BigInt(parseFloat(amountA) * 1e7);
      const quoteResult = await poolContract.getQuoteAToB(amountInBigInt);
      setQuote(quoteResult);
    } catch (err: any) {
      console.error('Error getting quote:', err);
      setError(err.message || 'Failed to get quote');
    }
  };

  const handleOptimalLiquidity = () => {
    if (!poolInfo || !amountA) return;

    const amountABigInt = BigInt(parseFloat(amountA) * 1e7);
    const amountBBigInt = BigInt(parseFloat(amountB || '0') * 1e7);

    const optimal = calculateOptimalLiquidity(
      amountABigInt,
      amountBBigInt,
      poolInfo.reserve_a,
      poolInfo.reserve_b
    );

    setAmountA((Number(optimal.amountA) / 1e7).toString());
    setAmountB((Number(optimal.amountB) / 1e7).toString());
  };

  if (loading) {
    return (
      <Card>
        <Card.Body className="text-center">
          <Spinner animation="border" />
          <div className="mt-2">Loading pool info...</div>
        </Card.Body>
      </Card>
    );
  }

  if (error && !poolInfo) {
    return (
      <Card border="danger">
        <Card.Body>
          <Alert variant="danger">{error}</Alert>
        </Card.Body>
      </Card>
    );
  }

  if (!poolInfo) {
    return null;
  }

  const price = poolInfo.reserve_a > BigInt(0)
    ? Number(poolInfo.reserve_b) / Number(poolInfo.reserve_a)
    : 0;

  return (
    <Card>
      <Card.Header>
        <h5 className="mb-0">Liquidity Pool</h5>
      </Card.Header>
      <Card.Body>
        <Row className="mb-4">
          <Col md={6}>
            <div className="mb-3">
              <small className="text-muted">Token A Reserve</small>
              <div className="h4">{(Number(poolInfo.reserve_a) / 1e7).toLocaleString()}</div>
              <code className="small">{poolInfo.token_a.slice(0, 8)}...</code>
            </div>
          </Col>
          <Col md={6}>
            <div className="mb-3">
              <small className="text-muted">Token B Reserve</small>
              <div className="h4">{(Number(poolInfo.reserve_b) / 1e7).toLocaleString()}</div>
              <code className="small">{poolInfo.token_b.slice(0, 8)}...</code>
            </div>
          </Col>
        </Row>

        <Row className="mb-4">
          <Col md={4}>
            <div>
              <small className="text-muted">Price (B/A)</small>
              <div className="h5">{price.toFixed(6)}</div>
            </div>
          </Col>
          <Col md={4}>
            <div>
              <small className="text-muted">Total Shares</small>
              <div className="h5">{(Number(poolInfo.total_shares) / 1e7).toLocaleString()}</div>
            </div>
          </Col>
          <Col md={4}>
            <div>
              <small className="text-muted">Fee Rate</small>
              <div className="h5">{(poolInfo.fee_rate / 100).toFixed(2)}%</div>
            </div>
          </Col>
        </Row>

        <hr />

        <h6 className="mb-3">Swap Quote</h6>
        <Row>
          <Col md={6}>
            <Form.Group className="mb-3">
              <Form.Label>Amount In (Token A)</Form.Label>
              <InputGroup>
                <Form.Control
                  type="number"
                  placeholder="0.0"
                  value={amountA}
                  onChange={(e) => setAmountA(e.target.value)}
                />
                <Button variant="outline-secondary" onClick={handleGetQuote}>
                  Get Quote
                </Button>
              </InputGroup>
            </Form.Group>
          </Col>
          <Col md={6}>
            <Form.Group className="mb-3">
              <Form.Label>Amount Out (Token B)</Form.Label>
              <Form.Control
                type="text"
                placeholder="0.0"
                value={quote ? (Number(quote) / 1e7).toFixed(6) : ''}
                readOnly
              />
            </Form.Group>
          </Col>
        </Row>

        {error && (
          <Alert variant="warning" className="mt-3">
            {error}
          </Alert>
        )}

        {!userAddress && (
          <Alert variant="info">
            Connect your wallet to interact with the pool
          </Alert>
        )}
      </Card.Body>
    </Card>
  );
}
