use soroban_sdk::{Address, Env, Vec};

use crate::reserves::{get_reserves};
use crate::error::SoroswapLibraryError;

/// Given some amount of an asset and pair reserves, returns an equivalent amount of the other asset.
///
/// # Arguments
///
/// * `amount_a` - The amount of the first asset.
/// * `reserve_a` - Reserves of the first asset in the pair.
/// * `reserve_b` - Reserves of the second asset in the pair.
///
/// # Returns
///
/// Returns `Result<i128, SoroswapLibraryError>` where `Ok` contains the calculated equivalent amount, and `Err` indicates an error such as insufficient amount or liquidity
pub fn quote(amount_a: i128, reserve_a: i128, reserve_b: i128) -> Result<i128, SoroswapLibraryError> {
    if amount_a <= 0 {
        return Err(SoroswapLibraryError::InsufficientAmount);
    }
    if reserve_a <= 0 || reserve_b <= 0 {
        return Err(SoroswapLibraryError::InsufficientLiquidity);
    }
    Ok(amount_a.checked_mul(reserve_b).ok_or(SoroswapLibraryError::InsufficientLiquidity)?.checked_div(reserve_a).ok_or(SoroswapLibraryError::InsufficientLiquidity)?)
}

/// Given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset.
///
/// # Arguments
///
/// * `amount_in` - The input amount of the asset.
/// * `reserve_in` - Reserves of the input asset in the pair.
/// * `reserve_out` - Reserves of the output asset in the pair.
///
/// # Returns
///
/// Returns `Result<i128, SoroswapLibraryError>` where `Ok` contains the calculated maximum output amount, and `Err` indicates an error such as insufficient input amount or liquidity.
pub fn get_amount_out(amount_in: i128, reserve_in: i128, reserve_out: i128) -> Result<i128, SoroswapLibraryError> {
    if amount_in <= 0 {
        return Err(SoroswapLibraryError::InsufficientInputAmount);
    }
    if reserve_in <= 0 || reserve_out <= 0 {
        return Err(SoroswapLibraryError::InsufficientLiquidity);
    }

    let amount_in_with_fee = amount_in.checked_mul(997).unwrap();
    let numerator = amount_in_with_fee.checked_mul(reserve_out).unwrap();

    let denominator = reserve_in.checked_mul(1000).unwrap().checked_add(amount_in_with_fee).unwrap();

    Ok(numerator.checked_div(denominator).unwrap())
}

/// Given an output amount of an asset and pair reserves, returns a required input amount of the other asset.
///
/// # Arguments
///
/// * `amount_out` - The output amount of the asset.
/// * `reserve_in` - Reserves of the input asset in the pair.
/// * `reserve_out` - Reserves of the output asset in the pair.
///
/// # Returns
///
/// Returns `Result<i128, SoroswapLibraryError>` where `Ok` contains the required input amount, and `Err` indicates an error such as insufficient output amount or liquidity.
pub fn get_amount_in(amount_out: i128, reserve_in: i128, reserve_out: i128) -> Result<i128, SoroswapLibraryError> {
    if amount_out <= 0 {
        return Err(SoroswapLibraryError::InsufficientOutputAmount);
    }
    if reserve_in <= 0 || reserve_out <= 0 {
        return Err(SoroswapLibraryError::InsufficientLiquidity);
    }
    let numerator = reserve_in.checked_mul(amount_out).unwrap().checked_mul(1000).unwrap();
    let denominator = reserve_out.checked_sub(amount_out).unwrap().checked_mul(997).unwrap();
    Ok(numerator.checked_div(denominator).unwrap().checked_add(1).unwrap())
}

/// Performs chained getAmountOut calculations on any number of pairs.
///
/// # Arguments
///
/// * `e` - The environment.
/// * `factory` - The factory address.
/// * `amount_in` - The input amount.
/// * `path` - Vector of token addresses representing the path.
///
/// # Returns
///
/// Returns `Result<Vec<i128>, SoroswapLibraryError>` where `Ok` contains a vector of calculated amounts, and `Err` indicates an error such as an invalid path.
pub fn get_amounts_out(e: Env, factory: Address, amount_in: i128, path: Vec<Address>) -> Result<Vec<i128>, SoroswapLibraryError> {
    if path.len() < 2 {
        return Err(SoroswapLibraryError::InvalidPath);
    }

    let mut amounts = Vec::new(&e);
    amounts.push_back(amount_in);

    for i in 0..path.len() - 1 {
        let (reserve_in, reserve_out) = get_reserves(e.clone(), factory.clone(), path.get(i).unwrap(), path.get(i+1).unwrap())?;
        amounts.push_back(get_amount_out(amounts.get(i).unwrap(), reserve_in, reserve_out)?);
    }

    Ok(amounts)
}

/// Performs chained getAmountIn calculations on any number of pairs.
///
/// # Arguments
///
/// * `e` - The environment.
/// * `factory` - The factory address.
/// * `amount_out` - The output amount.
/// * `path` - Vector of token addresses representing the path.
///
/// # Returns
///
/// Returns `Result<Vec<i128>, SoroswapLibraryError>` where `Ok` contains a vector of calculated amounts, and `Err` indicates an error such as an invalid path.
pub fn get_amounts_in(e: Env, factory: Address, amount_out: i128, path: Vec<Address>) -> Result<Vec<i128>, SoroswapLibraryError> {
    if path.len() < 2 {
        return Err(SoroswapLibraryError::InvalidPath);
    }

    let mut amounts = Vec::new(&e);
    amounts.push_front(amount_out);

    for i in (1..path.len()).rev() {
        let (reserve_in, reserve_out) = get_reserves(e.clone(), factory.clone(), path.get(i-1).unwrap(), path.get(i).unwrap())?;
        let new_amount = get_amount_in(amounts.get(0).unwrap(), reserve_in, reserve_out)?;
        amounts.push_front(new_amount);
    }

    Ok(amounts)
}
