use super::{aggregates, arguments, field, filter, order_by, relationships};
use crate::NdcFieldAlias;
use crate::{NdcRelationshipName, VariableName};
use indexmap::IndexMap;
use open_dds::{data_connector::CollectionName, types::DataConnectorArgumentName};
use std::collections::BTreeMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
// this represents an execution plan. all predicates only refer to local comparisons.
// remote predicates are represented as additional execution nodes
pub struct QueryExecutionPlan {
    pub remote_predicates: PredicateQueryTrees,
    pub query_node: QueryNodeNew,
    /// The name of a collection
    pub collection: CollectionName,
    /// Values to be provided to any collection arguments
    pub arguments: BTreeMap<DataConnectorArgumentName, arguments::Argument>,
    /// Any relationships between collections involved in the query request
    pub collection_relationships: BTreeMap<NdcRelationshipName, relationships::Relationship>,
    /// One set of named variables for each rowset to fetch. Each variable set
    /// should be subtituted in turn, and a fresh set of rows returned.
    pub variables: Option<Vec<BTreeMap<VariableName, serde_json::Value>>>,
    /// The data connector used to fetch the data
    pub data_connector: Arc<metadata_resolve::DataConnectorLink>,
}

/// A tree of queries that are used to execute remote predicates
#[derive(Debug, Clone, PartialEq)]
pub struct PredicateQueryTree {
    pub query: QueryExecutionPlan,
    pub children: PredicateQueryTrees,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PredicateQueryTrees(pub BTreeMap<Uuid, PredicateQueryTree>);

impl PredicateQueryTrees {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
    pub fn insert(&mut self, value: PredicateQueryTree) -> Uuid {
        let key = Uuid::new_v4();
        self.0.insert(key, value);
        key
    }
}

impl Default for PredicateQueryTrees {
    fn default() -> Self {
        Self::new()
    }
}

/// Query plan for fetching data
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QueryNodeNew {
    /// Optionally limit to N results
    pub limit: Option<u32>,
    /// Optionally offset from the Nth result
    pub offset: Option<u32>,
    /// Optionally sort results
    pub order_by: Option<Vec<order_by::OrderByElement>>,
    /// Optionally filter results
    pub predicate: Option<filter::ResolvedFilterExpression>,
    /// Aggregate fields of the query
    pub aggregates: Option<aggregates::AggregateSelectionSet>,
    /// Fields of the query
    pub fields: Option<FieldsSelection>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldsSelection {
    pub fields: IndexMap<NdcFieldAlias, field::Field>,
}

// FIXME: remove this; this is probably inaccurate.
// https://github.com/indexmap-rs/indexmap/issues/155
// Probably use ordermap (ref: https://github.com/indexmap-rs/indexmap/issues/67#issuecomment-2189801441)
impl std::hash::Hash for FieldsSelection {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (k, _v) in &self.fields {
            k.hash(state);
            // FIXME!!
            // v.hash(state);
        }
    }
}
