use ethers::types::U256;


pub fn u256_to_string(amount: U256, decimals: U256) -> String {
    let factor = U256::from(10).pow(decimals);
    let left = amount / factor;
    let right = amount % factor;
    let right_str = format!("{:0>width$}", right, width = decimals.as_usize());
    let right_trimmed = right_str.trim_end_matches('0');

    if right_trimmed.is_empty() {
        format!("{left}")
    } else {
        format!("{left}.{right_trimmed}")
    }
}

pub fn parse_u256(amount: &str, decimals: U256) -> Result<U256, Box<dyn std::error::Error>> {
    let factor = U256::from(10).pow(decimals);
    let parts: Vec<&str> = amount.split('.').collect();
    let left = U256::from_dec_str(parts[0])?;
    
    let right = if parts.len() > 1 {
        let right_str = format!("{:0<width$}", parts[1], width = decimals.as_usize());
        U256::from_dec_str(&right_str)?
    } else {
        U256::zero()
    };

    Ok(left * factor + right)
}

pub fn ether_to_string(amount: U256) -> String {
    u256_to_string(amount, U256::from(18))
}

pub fn parse_ether(amount: &str) -> Result<U256, Box<dyn std::error::Error>> {
    parse_u256(amount, U256::from(18))
}