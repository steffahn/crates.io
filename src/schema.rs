use diesel::{
    associations::HasTable,
    query_builder::{nodes::Identifier, AsQuery, QueryFragment, SelectStatement},
    query_source::{
        joins::{Join, LeftOuter},
        AppearsInFromClause, Never, Once,
    },
    sql_types::{Bpchar, Int4, Nullable},
    AppearsOnTable, QuerySource, SelectableExpression, Table,
};

#[allow(non_camel_case_types)]
pub(crate) struct table;

pub(crate) type SqlType = (Int4, Nullable<Bpchar>);
impl QuerySource for table {
    type FromClause = Identifier<'static>;
    type DefaultSelection = <Self as Table>::AllColumns;
    fn from_clause(&self) -> Self::FromClause {
        todo!()
    }
    fn default_selection(&self) -> Self::DefaultSelection {
        todo!()
    }
}
impl AsQuery for table {
    type SqlType = SqlType;
    type Query = SelectStatement<Self>;
    fn as_query(self) -> Self::Query {
        todo!()
    }
}
impl Table for table {
    type PrimaryKey = id;
    type AllColumns = (id, checksum);
    fn primary_key(&self) -> Self::PrimaryKey {
        todo!()
    }
    fn all_columns() -> Self::AllColumns {
        todo!()
    }
}
impl HasTable for table {
    type Table = Self;
    fn table() -> Self::Table {
        todo!()
    }
}
impl AppearsInFromClause<table> for table {
    type Count = Once;
}

#[allow(non_camel_case_types)]
pub(crate) struct id;

impl diesel::expression::Expression for id {
    type SqlType = Int4;
}
impl<DB> diesel::query_builder::QueryFragment<DB> for id
where
    DB: diesel::backend::Backend,
    <table as QuerySource>::FromClause: QueryFragment<DB>,
{
    fn walk_ast(
        &self,
        _out: diesel::query_builder::AstPass<DB>,
    ) -> diesel::result::QueryResult<()> {
        todo!()
    }
}
impl SelectableExpression<table> for id {}

impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}

impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
where
    id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
    Left: AppearsInFromClause<table, Count = Once>,
    Right: AppearsInFromClause<table, Count = Never>,
{
}

impl diesel::expression::NonAggregate for id {}

impl diesel::query_source::Column for id {
    type Table = table;
    const NAME: &'static str = ("id");
}
impl<T> diesel::EqAll<T> for id
where
    T: diesel::expression::AsExpression<Int4>,
    diesel::dsl::Eq<id, T>: diesel::Expression<SqlType = diesel::sql_types::Bool>,
{
    type Output = diesel::dsl::Eq<Self, T>;
    fn eq_all(self, _rhs: T) -> Self::Output {
        todo!()
    }
}
#[allow(non_camel_case_types)]
pub(crate) struct checksum;

impl diesel::expression::Expression for checksum {
    type SqlType = Nullable<Bpchar>;
}
impl<DB> diesel::query_builder::QueryFragment<DB> for checksum
where
    DB: diesel::backend::Backend,
    <table as QuerySource>::FromClause: QueryFragment<DB>,
{
    fn walk_ast(
        &self,
        _out: diesel::query_builder::AstPass<DB>,
    ) -> diesel::result::QueryResult<()> {
        todo!()
    }
}
impl SelectableExpression<table> for checksum {}

impl<QS> AppearsOnTable<QS> for checksum where QS: AppearsInFromClause<table, Count = Once> {}

impl diesel::expression::NonAggregate for checksum {}
