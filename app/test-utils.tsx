/**
 * Test utilities for React Testing Library
 * Provides custom render functions and common test utilities
 */

import { render as rtlRender, RenderOptions } from '@testing-library/react'
import { ReactElement, ReactNode } from 'react'
import { IntlProvider } from 'react-intl'

// Default locale for tests
const defaultLocale = 'en'

// Mock messages for testing
const mockMessages: Record<string, string> = {
  'app.title': 'Stellar Chain Explorer',
  'nav.home': 'Home',
  'nav.ledgers': 'Ledgers',
  'nav.transactions': 'Transactions',
  'nav.operations': 'Operations',
  'nav.assets': 'Assets',
  'nav.accounts': 'Accounts',
}

interface AllTheProvidersProps {
  children: ReactNode
  locale?: string
  messages?: Record<string, string>
}

/**
 * Wrapper component that provides all necessary context providers
 */
function AllTheProviders({
  children,
  locale = defaultLocale,
  messages = mockMessages,
}: AllTheProvidersProps) {
  return (
    <IntlProvider locale={locale} messages={messages}>
      {children}
    </IntlProvider>
  )
}

interface CustomRenderOptions extends Omit<RenderOptions, 'wrapper'> {
  locale?: string
  messages?: Record<string, string>
}

/**
 * Custom render function that wraps components with necessary providers
 * @param ui - Component to render
 * @param options - Render options including locale and messages
 */
function customRender(
  ui: ReactElement,
  {
    locale = defaultLocale,
    messages = mockMessages,
    ...renderOptions
  }: CustomRenderOptions = {}
) {
  return rtlRender(ui, {
    wrapper: ({ children }) => (
      <AllTheProviders locale={locale} messages={messages}>
        {children}
      </AllTheProviders>
    ),
    ...renderOptions,
  })
}

/**
 * Creates a mock Stellar account object for testing
 */
export function createMockAccount(overrides = {}) {
  return {
    id: 'GAREELUB43IRHWEASCFBLKHURCGMHE5IF6XSE7EXDLACYHGRHM43RFOX',
    account_id: 'GAREELUB43IRHWEASCFBLKHURCGMHE5IF6XSE7EXDLACYHGRHM43RFOX',
    sequence: '123456789',
    subentry_count: 0,
    home_domain: 'example.com',
    last_modified_ledger: 1000000,
    thresholds: {
      low_threshold: 0,
      med_threshold: 0,
      high_threshold: 0,
    },
    flags: {
      auth_required: false,
      auth_revocable: false,
      auth_immutable: false,
    },
    balances: [
      {
        balance: '10000.0000000',
        asset_type: 'native',
      },
    ],
    signers: [
      {
        weight: 1,
        key: 'GAREELUB43IRHWEASCFBLKHURCGMHE5IF6XSE7EXDLACYHGRHM43RFOX',
        type: 'ed25519_public_key',
      },
    ],
    data: {},
    ...overrides,
  }
}

/**
 * Creates a mock Stellar transaction object for testing
 */
export function createMockTransaction(overrides = {}) {
  return {
    id: '26a568681712a44a515b2c90dcfaadb3ed2c40dc60254638407937bee4767071',
    hash: '26a568681712a44a515b2c90dcfaadb3ed2c40dc60254638407937bee4767071',
    ledger: 1000000,
    created_at: '2024-01-01T00:00:00Z',
    source_account: 'GAREELUB43IRHWEASCFBLKHURCGMHE5IF6XSE7EXDLACYHGRHM43RFOX',
    source_account_sequence: '123456789',
    fee_account: 'GAREELUB43IRHWEASCFBLKHURCGMHE5IF6XSE7EXDLACYHGRHM43RFOX',
    fee_charged: '100',
    max_fee: '100',
    operation_count: 1,
    successful: true,
    ...overrides,
  }
}

/**
 * Creates a mock Stellar ledger object for testing
 */
export function createMockLedger(overrides = {}) {
  return {
    id: '1000000',
    sequence: 1000000,
    hash: 'abc123',
    prev_hash: 'def456',
    transaction_count: 10,
    successful_transaction_count: 10,
    failed_transaction_count: 0,
    operation_count: 20,
    closed_at: '2024-01-01T00:00:00Z',
    total_coins: '100000000000.0000000',
    fee_pool: '10000.0000000',
    base_fee_in_stroops: 100,
    base_reserve_in_stroops: 5000000,
    max_tx_set_size: 1000,
    protocol_version: 20,
    ...overrides,
  }
}

/**
 * Creates a mock Stellar operation object for testing
 */
export function createMockOperation(overrides = {}) {
  return {
    id: '1000000-1',
    type: 'payment',
    type_i: 1,
    created_at: '2024-01-01T00:00:00Z',
    transaction_hash:
      '26a568681712a44a515b2c90dcfaadb3ed2c40dc60254638407937bee4767071',
    source_account: 'GAREELUB43IRHWEASCFBLKHURCGMHE5IF6XSE7EXDLACYHGRHM43RFOX',
    ...overrides,
  }
}

/**
 * Waits for a specific amount of time
 * Useful for testing async operations
 */
export function wait(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

/**
 * Mock fetch response helper
 */
export function createMockFetchResponse<T>(data: T, ok = true) {
  return {
    ok,
    status: ok ? 200 : 400,
    statusText: ok ? 'OK' : 'Bad Request',
    json: async () => data,
    text: async () => JSON.stringify(data),
  } as Response
}

// Re-export everything from React Testing Library
export * from '@testing-library/react'
export { customRender as render }
