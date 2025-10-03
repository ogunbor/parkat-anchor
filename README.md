# Parkat Anchor - Decentralized Parking Management System

A programme for automated parking lot management with multi-tenant support. Built with Anchor framework for schools, universities, corporate campuses, and residential complexes.

## Overview

Parkat Anchor enables organizations to run independent parking operations on a shared blockchain infrastructure. Each tenant (parking operator) manages their own users, rates, and funds while benefiting from transparent, automated fee collection.

**Perfect for:**
- **Schools & Universities** - Multiple departments or campuses managing separate parking zones
- **Corporate Campuses** - Different buildings or tenant companies with independent parking
- **Residential Complexes** - Multiple property managers sharing infrastructure
- **Shopping Centers** - Individual stores or zones with dedicated parking

## Multi-Tenancy Model

Each tenant operates independently with:
- Separate user registrations and vehicle tracking
- Independent vault accounts for user deposits
- Custom admin wallets for fee collection
- Isolated parking session management

Users register under a specific tenant and can only use that tenant's parking facilities, ensuring clear separation between operators.

## Key Features

- **Deposit-Based System** - Users pre-fund their parking vault
- **Automated Fee Calculation** - 100 lamports per minute (configurable)
- **Real-Time Tracking** - On-chain parking session records
- **Transparent Accounting** - All transactions verifiable on Solana

## Program Structure

**State Accounts:**
- `Tenant` - Parking operator (school, building, etc.)
- `User` - Registered parker with vehicle info and deposit vault

**Instructions:**
1. `init_tenant` - Create new parking operator
2. `init_user` - Register user with vehicle number plate
3. `deposit` - Add funds to parking vault
4. `record_parking_start` - Begin parking session
5. `process_exit` - Calculate fees and process payment

## Quick Start
```bash
# Build
anchor build

# Test
cargo test

# Deploy
anchor deploy

# ProgramID: FDKqFqZ8MnAfwVCGAR8FJfbSjHyfqs14Vx9c1hBZSjGU