# Email Validation Service

This service helps you decide whether an email is safe and usable before you accept it in your product flow.

## What the service does

The service reviews each email and returns a final pass/fail decision so teams can reduce fake signups, low-quality leads, and delivery issues.

It is designed for:

- Signup and onboarding checks
- Lead form quality control
- CRM and contact list cleanup
- Fraud and abuse prevention

## What we check

Each email goes through multiple checks, including:

- **Temporary email detection**  
  Flags addresses from disposable or short-lived mailbox providers.

- **SMTP mailbox check**  
  Verifies whether the destination mail system can receive messages for that address.

- **DLL screening**  
  Screens the domain against known risky, disposable, or blocked-domain lists (DLL).

- **Basic quality signals**  
  Looks for common patterns that indicate low-trust or non-usable addresses.

## Checking flow

1. The email is received by the service.
2. The service runs all validation signals in sequence.
3. Results are combined into one overall trust decision.
4. A final status is returned for downstream action.

## How we conclude the result

The final result is based on all checks together, not a single signal.

- **Valid**: no critical issues found, email is safe to use.
- **Invalid**: one or more hard-fail checks triggered.
- **Risky**: usable but high-risk indicators are present.

## Why this helps

Using this service early in your flow improves data quality, protects sender reputation, and reduces wasted follow-up on unreachable or disposable addresses.
