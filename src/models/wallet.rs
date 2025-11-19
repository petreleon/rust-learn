use diesel::prelude::*;
use crate::db::schema::wallets;
use bigdecimal::BigDecimal;

#[derive(Queryable, Identifiable, Debug, Clone)]
#[diesel(table_name = wallets)]
pub struct Wallet {
    pub id: i32,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub value: BigDecimal,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = wallets)]
pub struct NewWallet {
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub value: BigDecimal,
}

impl Wallet {
    pub fn find_by_user_id(user_id: i32, conn: &mut PgConnection) -> QueryResult<Option<i32>> {
        wallets::table
            .select(wallets::id)
            .filter(wallets::user_id.eq(user_id).and(wallets::organization_id.is_null()))
            .first(conn)
            .optional()
    }

    pub fn find_by_organization_id(org_id: i32, conn: &mut PgConnection) -> QueryResult<Option<i32>> {
        wallets::table
            .select(wallets::id)
            .filter(wallets::organization_id.eq(org_id).and(wallets::user_id.is_null()))
            .first(conn)
            .optional()
    }

    pub fn create(new_wallet: NewWallet, conn: &mut PgConnection) -> QueryResult<i32> {
        diesel::insert_into(wallets::table)
            .values(&new_wallet)
            .returning(wallets::id)
            .get_result(conn)
    }

    pub fn update_balance_guarded(wallet_id: i32, amount: BigDecimal, conn: &mut PgConnection) -> QueryResult<usize> {
        let zero = BigDecimal::from(0);
        diesel::update(
            wallets::table.filter(
                wallets::id
                    .eq(wallet_id)
                    .and((wallets::value + amount.clone()).ge(zero)),
            ),
        )
        .set(wallets::value.eq(wallets::value + amount))
        .execute(conn)
    }

    pub fn lock_wallets(ids: Vec<i32>, conn: &mut PgConnection) -> QueryResult<Vec<(i32, BigDecimal)>> {
        wallets::table
            .select((wallets::id, wallets::value))
            .filter(wallets::id.eq_any(ids))
            .order(wallets::id.asc())
            .for_update()
            .load(conn)
    }
}
