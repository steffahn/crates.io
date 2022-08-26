mod schema;

use diesel::{
    associations::{HasTable, Identifiable},
    prelude::{ExpressionMethods, PgConnection, RunQueryDsl},
    query_builder::{IntoUpdateTarget, UpdateStatement},
    Queryable,
};

struct Version {}

impl HasTable for Version {
    type Table = schema::table;
    fn table() -> Self::Table {
        todo!()
    }
}
impl<'ident> Identifiable for &'ident Version {
    type Id = &'ident i32;
    fn id(self) -> Self::Id {
        todo!()
    }
}

impl<__DB: diesel::backend::Backend, __ST> Queryable<__ST, __DB> for Version
where
    (i32, Option<String>): Queryable<__ST, __DB>,
{
    type Row = <(i32, Option<String>) as Queryable<__ST, __DB>>::Row;
    fn build(_row: Self::Row) -> Self {
        todo!()
    }
}

#[allow(unreachable_code, unused_variables)]
pub fn build(connection: &PgConnection) {
    let x: UpdateStatement<schema::table, <&Version as IntoUpdateTarget>::WhereClause> = todo!();
    let y = x.set::<diesel::expression::operators::Eq<
        schema::id,
        diesel::expression::bound::Bound<_, i32>, // <- changing _ to explitit `…::Integer` makes the error vanish
    >>(schema::id.eq(0));
    let _ = y.get_result::<Version>(connection);
}
