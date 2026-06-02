use crate::db::schema::wallets;
use crate::models::wallet::{NewWallet, Wallet};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

#[derive(Debug)]
pub struct LinkedWallet {
    pub wallet: Wallet,
    pub created: bool,
}

fn is_unique_violation(error: &DieselError) -> bool {
    matches!(
        error,
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)
    )
}

pub async fn find_user_wallet(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> QueryResult<Option<Wallet>> {
    wallets::table
        .filter(wallets::user_id.eq(user_id))
        .filter(wallets::organization_id.is_null())
        .first(conn)
        .await
        .optional()
}

pub async fn find_organization_wallet(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> QueryResult<Option<Wallet>> {
    wallets::table
        .filter(wallets::organization_id.eq(organization_id))
        .filter(wallets::user_id.is_null())
        .first(conn)
        .await
        .optional()
}

pub async fn link_user_wallet(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> QueryResult<LinkedWallet> {
    if let Some(wallet) = find_user_wallet(conn, user_id).await? {
        return Ok(LinkedWallet {
            wallet,
            created: false,
        });
    }

    let new_wallet = NewWallet {
        user_id: Some(user_id),
        organization_id: None,
        value: BigDecimal::from(0),
    };

    match diesel::insert_into(wallets::table)
        .values(&new_wallet)
        .get_result(conn)
        .await
    {
        Ok(wallet) => Ok(LinkedWallet {
            wallet,
            created: true,
        }),
        Err(error) if is_unique_violation(&error) => {
            let wallet = find_user_wallet(conn, user_id)
                .await?
                .ok_or(DieselError::NotFound)?;
            Ok(LinkedWallet {
                wallet,
                created: false,
            })
        }
        Err(error) => Err(error),
    }
}

pub async fn link_organization_wallet(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> QueryResult<LinkedWallet> {
    if let Some(wallet) = find_organization_wallet(conn, organization_id).await? {
        return Ok(LinkedWallet {
            wallet,
            created: false,
        });
    }

    let new_wallet = NewWallet {
        user_id: None,
        organization_id: Some(organization_id),
        value: BigDecimal::from(0),
    };

    match diesel::insert_into(wallets::table)
        .values(&new_wallet)
        .get_result(conn)
        .await
    {
        Ok(wallet) => Ok(LinkedWallet {
            wallet,
            created: true,
        }),
        Err(error) if is_unique_violation(&error) => {
            let wallet = find_organization_wallet(conn, organization_id)
                .await?
                .ok_or(DieselError::NotFound)?;
            Ok(LinkedWallet {
                wallet,
                created: false,
            })
        }
        Err(error) => Err(error),
    }
}
