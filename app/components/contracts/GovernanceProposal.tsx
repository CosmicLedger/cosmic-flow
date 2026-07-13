/**
 * Governance Proposal Component
 * Displays proposal information and voting interface
 */

import { useState, useEffect } from 'react';
import { Card, Badge, Button, ProgressBar, Spinner } from 'react-bootstrap';
import {
  GovernanceContract,
  Proposal,
  ProposalStatus,
  formatProposalStatus,
  getProposalStatusColor,
} from '~/lib/contracts/governance';
import { getNetworkConfig } from '~/lib/contracts/client';

interface GovernanceProposalProps {
  contractId: string;
  proposalId: bigint;
  userAddress?: string;
  network?: 'testnet' | 'mainnet' | 'futurenet';
  onVoteSuccess?: () => void;
}

export default function GovernanceProposal({
  contractId,
  proposalId,
  userAddress,
  network = 'testnet',
  onVoteSuccess,
}: GovernanceProposalProps) {
  const [proposal, setProposal] = useState<Proposal | null>(null);
  const [loading, setLoading] = useState(true);
  const [voting, setVoting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetchProposal();
  }, [contractId, proposalId, network]);

  const fetchProposal = async () => {
    setLoading(true);
    setError(null);

    try {
      const config = getNetworkConfig(network);
      const govContract = new GovernanceContract({
        contractId,
        rpcUrl: config.rpcUrl,
        networkPassphrase: config.networkPassphrase,
      });

      const prop = await govContract.getProposal(proposalId);
      setProposal(prop);
    } catch (err: any) {
      setError(err.message || 'Failed to fetch proposal');
      console.error('Error fetching proposal:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleVote = async (support: boolean) => {
    if (!userAddress) {
      alert('Please connect your wallet first');
      return;
    }

    setVoting(true);
    setError(null);

    try {
      const config = getNetworkConfig(network);
      const govContract = new GovernanceContract({
        contractId,
        rpcUrl: config.rpcUrl,
        networkPassphrase: config.networkPassphrase,
      });

      // Note: In real implementation, you would need to sign with Freighter
      // This is a simplified example
      alert('In production, this would sign with Freighter wallet');
      
      onVoteSuccess?.();
      await fetchProposal(); // Refresh proposal data
    } catch (err: any) {
      setError(err.message || 'Failed to cast vote');
      console.error('Error casting vote:', err);
    } finally {
      setVoting(false);
    }
  };

  if (loading) {
    return (
      <Card>
        <Card.Body className="text-center">
          <Spinner animation="border" />
          <div className="mt-2">Loading proposal...</div>
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

  if (!proposal) {
    return null;
  }

  const totalVotes = proposal.for_votes + proposal.against_votes;
  const forPercentage = totalVotes > BigInt(0)
    ? Number((proposal.for_votes * BigInt(100)) / totalVotes)
    : 0;
  const againstPercentage = totalVotes > BigInt(0)
    ? Number((proposal.against_votes * BigInt(100)) / totalVotes)
    : 0;

  const statusColor = getProposalStatusColor(proposal.status);

  return (
    <Card>
      <Card.Header>
        <div className="d-flex justify-content-between align-items-center">
          <h5 className="mb-0">Proposal #{proposal.id.toString()}</h5>
          <Badge bg={statusColor}>{formatProposalStatus(proposal.status)}</Badge>
        </div>
      </Card.Header>
      <Card.Body>
        <h4>{proposal.title}</h4>
        <p className="text-muted">{proposal.description}</p>

        <div className="mb-3">
          <small className="text-muted">Proposer</small>
          <div>
            <code>{proposal.proposer}</code>
          </div>
        </div>

        <div className="mb-3">
          <div className="d-flex justify-content-between mb-1">
            <span>For: {forPercentage.toFixed(2)}%</span>
            <span>{proposal.for_votes.toString()} votes</span>
          </div>
          <ProgressBar now={forPercentage} variant="success" />
        </div>

        <div className="mb-3">
          <div className="d-flex justify-content-between mb-1">
            <span>Against: {againstPercentage.toFixed(2)}%</span>
            <span>{proposal.against_votes.toString()} votes</span>
          </div>
          <ProgressBar now={againstPercentage} variant="danger" />
        </div>

        <div className="mb-3">
          <small className="text-muted">Voting Period</small>
          <div>
            Ledger {proposal.start_ledger} - {proposal.end_ledger}
          </div>
        </div>

        {proposal.status === ProposalStatus.Active && userAddress && (
          <div className="d-flex gap-2">
            <Button
              variant="success"
              onClick={() => handleVote(true)}
              disabled={voting}
            >
              {voting ? 'Voting...' : 'Vote For'}
            </Button>
            <Button
              variant="danger"
              onClick={() => handleVote(false)}
              disabled={voting}
            >
              {voting ? 'Voting...' : 'Vote Against'}
            </Button>
          </div>
        )}

        {error && (
          <div className="alert alert-danger mt-3 mb-0">
            {error}
          </div>
        )}
      </Card.Body>
    </Card>
  );
}
