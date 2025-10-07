# Verifying Partial Signature Shares in FROST

According to the FROST library documentation:

1. **Automatic verification**: The `frost::aggregate()` function (used at line 149 in the current implementation)
   automatically verifies each signature share during aggregation
2. **Optional pre-verification**: The library provides `frost_core::verify_signature_share()` for explicitly verifying
   individual shares before calling `aggregate()`

The documentation states: "This is not required for regular FROST usage but might be useful in certain
situations where it is desired to verify each individual signature share before aggregating the signature."

## Benefits of pre-verification:
- **Early detection**: Identify invalid shares immediately instead of waiting for aggregation to fail
- **Accountability**: Pinpoint exactly which signer produced an invalid share
- **DoS prevention**: Reject bad shares early before spending computation on aggregation

## Current implementation
The PoC only uses the automatic verification in `aggregate()`, which will fail if any
share is invalid but won't tell you which signer caused the problem. Adding explicit `verify_signature_share()`
calls would enable identifying malicious signers.

## Recommended Production Strategy

**Important constraint**: If you collect shares from a specific participant set (e.g., signers {1,2,3,4}),
ALL shares in that set must be valid. You cannot selectively use only the valid shares because each share
is computed with Lagrange coefficients for that exact participant set.

### Strategy 1: Pre-verify individual shares (RECOMMENDED)

```
1. Collect > T shares (e.g., T+2 for safety margin)
2. Verify each share individually with verify_signature_share()
3. Keep first T valid shares
4. Aggregate the T valid shares (guaranteed to succeed)
```

**Advantages:**
- ✅ **Predictable performance**: O(n) verifications where n = shares collected
- ✅ **One aggregation**: Only aggregate once with known-valid shares
- ✅ **Identify ALL malicious signers** at once (for accountability/punishment)
- ✅ **Fail-fast**: Know immediately if you don't have enough valid shares

**Example implementation:**
```rust
// Collect T+2 shares for redundancy
let shares = collect_shares(threshold + 2);

// Verify individually
let mut valid_shares = BTreeMap::new();
for (id, share) in shares {
    if verify_signature_share(id, &share, &signing_package, &pubkey_package).is_ok() {
        valid_shares.insert(id, share);
        if valid_shares.len() == threshold {
            break; // Got enough valid shares
        }
    } else {
        report_malicious_signer(id); // Log for accountability
    }
}

// Check if we have enough valid shares
if valid_shares.len() < threshold {
    return Err("Insufficient valid shares");
}

// Aggregate (guaranteed success with pre-verified shares)
let signature = frost::aggregate(&signing_package, &valid_shares, &pubkey_package).unwrap();
```

### Strategy 2: Trial-and-error aggregation (NOT recommended)

```
1. Collect > T shares
2. Try aggregating T shares
3. If fails, note culprit, exclude them, try with different T
4. Repeat until success
```

**Disadvantages:**
- ❌ **Unpredictable retries**: With k invalid shares, may need many attempts
- ❌ **Each failure only reveals ONE culprit** - need multiple rounds for multiple bad actors
- ❌ **Aggregation likely more expensive** than individual verification
- ❌ Harder to identify all malicious signers for accountability

### Why Strategy 1 is better for production:

1. **Byzantine fault tolerance**: Quickly identify and exclude ALL malicious signers
2. **Predictable costs**: O(n) + O(1) vs potentially exponential retries
3. **Network efficiency**: In distributed systems, reject bad shares immediately
4. **Accountability**: Log all malicious signers at once for forensics/slashing
