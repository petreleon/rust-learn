fn build_observed_event(
    log: Log,
    contract_address: Address,
    event_type: &str,
    from_address: Address,
    to_address: Address,
    chain_id: i64,
    token_decimals: u32,
) -> Result<ObservedWalletDepositEvent, String> {
    let transaction_hash = log
        .transaction_hash
        .ok_or_else(|| "log is missing transaction_hash".to_string())?;
    let log_index = log
        .log_index
        .ok_or_else(|| "log is missing log_index".to_string())?
        .as_u64() as i64;
    let amount = amount_from_log_data(log.data.0.as_ref(), token_decimals)?;

    Ok(ObservedWalletDepositEvent {
        chain_id,
        contract_address: format!("{:#x}", contract_address),
        transaction_hash: format!("{:#x}", transaction_hash),
        log_index,
        event_type: event_type.to_string(),
        from_address: format!("{:#x}", from_address),
        to_address: format!("{:#x}", to_address),
        amount,
    })
}

fn amount_from_log_data(data: &[u8], token_decimals: u32) -> Result<BigDecimal, String> {
    if data.len() != 32 {
        return Err(format!(
            "event amount data must be 32 bytes, got {}",
            data.len()
        ));
    }
    let amount = U256::from_big_endian(data);
    u256_to_decimal(amount, token_decimals)
}

fn u256_to_decimal(amount: U256, token_decimals: u32) -> Result<BigDecimal, String> {
    let raw = amount.to_string();
    if token_decimals == 0 {
        return BigDecimal::from_str(&raw)
            .map_err(|error| format!("failed to parse token amount: {error}"));
    }

    let decimals = token_decimals as usize;
    let decimal = if raw.len() <= decimals {
        format!("0.{}{}", "0".repeat(decimals - raw.len()), raw)
    } else {
        let split = raw.len() - decimals;
        format!("{}.{}", &raw[..split], &raw[split..])
    };
    let trimmed = decimal.trim_end_matches('0').trim_end_matches('.');
    BigDecimal::from_str(if trimmed.is_empty() { "0" } else { trimmed })
        .map_err(|error| format!("failed to parse token amount: {error}"))
}

fn address_from_topic(topic: H256) -> Option<Address> {
    Some(Address::from_slice(&topic.as_bytes()[12..]))
}

fn event_signature(signature: &str) -> H256 {
    H256::from_slice(&ethers::utils::keccak256(signature.as_bytes()))
}

fn parse_address(value: &str) -> Result<Address, String> {
    Address::from_str(value.trim())
        .map_err(|error| format!("invalid address '{}': {error}", value.trim()))
}

fn env_bool(key: &str, default: bool) -> bool {
    env::var(key)
        .ok()
        .map(|value| {
            matches!(
                value.to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(default)
}

fn should_log_indexer_poll(
    credited_count: usize,
    last_idle_log: Option<Instant>,
    idle_log_interval: Duration,
    now: Instant,
) -> bool {
    if credited_count > 0 {
        return true;
    }

    last_idle_log
        .map(|logged_at| now.duration_since(logged_at) >= idle_log_interval)
        .unwrap_or(true)
}

impl PollFailureLogState {
    fn record_failure(&mut self, now: Instant, warn_interval: Duration) -> PollFailureLogDecision {
        let first_failure_at = *self.first_failure_at.get_or_insert(now);
        self.consecutive_failures = self.consecutive_failures.saturating_add(1);
        let outage = now.saturating_duration_since(first_failure_at);

        let warn_due = outage >= warn_interval
            && self
                .last_warning_at
                .map(|last_warning_at| {
                    now.saturating_duration_since(last_warning_at) >= warn_interval
                })
                .unwrap_or(true);

        if warn_due {
            let suppressed_failure_count = self.suppressed_failure_count;
            self.suppressed_failure_count = 0;
            self.last_warning_at = Some(now);
            return PollFailureLogDecision {
                level: PollFailureLogLevel::Warn,
                consecutive_failures: self.consecutive_failures,
                suppressed_failure_count,
                outage_seconds: outage.as_secs(),
            };
        }

        if self.consecutive_failures == 1 {
            return PollFailureLogDecision {
                level: PollFailureLogLevel::Info,
                consecutive_failures: self.consecutive_failures,
                suppressed_failure_count: 0,
                outage_seconds: outage.as_secs(),
            };
        }

        self.suppressed_failure_count = self.suppressed_failure_count.saturating_add(1);
        PollFailureLogDecision {
            level: PollFailureLogLevel::Suppress,
            consecutive_failures: self.consecutive_failures,
            suppressed_failure_count: self.suppressed_failure_count,
            outage_seconds: outage.as_secs(),
        }
    }

    fn record_success(&mut self, now: Instant) -> Option<PollFailureRecovery> {
        let first_failure_at = self.first_failure_at?;
        let recovery = PollFailureRecovery {
            consecutive_failures: self.consecutive_failures,
            suppressed_failure_count: self.suppressed_failure_count,
            outage_seconds: now.saturating_duration_since(first_failure_at).as_secs(),
        };
        *self = Self::default();
        Some(recovery)
    }
}
