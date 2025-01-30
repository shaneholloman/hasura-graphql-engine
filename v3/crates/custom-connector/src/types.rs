use ndc_models;
use std::collections::BTreeMap;

pub mod actor;
pub mod city;
pub mod country;
pub mod evaluated_institution;
pub mod genre;
pub mod institution;
pub mod location;
pub mod login;
pub mod movie;
pub mod name_query;
pub mod staff_member;

pub(crate) fn scalar_types() -> BTreeMap<ndc_models::ScalarTypeName, ndc_models::ScalarType> {
    BTreeMap::from_iter([
        (
            "String".into(),
            ndc_models::ScalarType {
                representation: ndc_models::TypeRepresentation::String,
                aggregate_functions: BTreeMap::from_iter([
                    ("max".into(), ndc_models::AggregateFunctionDefinition::Max),
                    ("min".into(), ndc_models::AggregateFunctionDefinition::Min),
                ]),
                comparison_operators: BTreeMap::from_iter([
                    (
                        "like".into(),
                        ndc_models::ComparisonOperatorDefinition::Custom {
                            argument_type: ndc_models::Type::Named {
                                name: "String".into(),
                            },
                        },
                    ),
                    (
                        "_eq".into(),
                        ndc_models::ComparisonOperatorDefinition::Equal,
                    ),
                    (
                        "starts_with".into(),
                        ndc_models::ComparisonOperatorDefinition::Custom {
                            argument_type: ndc_models::Type::Named {
                                name: "String".into(),
                            },
                        },
                    ),
                    (
                        "ends_with".into(),
                        ndc_models::ComparisonOperatorDefinition::Custom {
                            argument_type: ndc_models::Type::Named {
                                name: "String".into(),
                            },
                        },
                    ),
                    (
                        "_contains".into(),
                        ndc_models::ComparisonOperatorDefinition::Custom {
                            argument_type: ndc_models::Type::Named {
                                name: "String".into(),
                            },
                        },
                    ),
                    (
                        "istarts_with".into(),
                        ndc_models::ComparisonOperatorDefinition::Custom {
                            argument_type: ndc_models::Type::Named {
                                name: "String".into(),
                            },
                        },
                    ),
                    (
                        "iends_with".into(),
                        ndc_models::ComparisonOperatorDefinition::Custom {
                            argument_type: ndc_models::Type::Named {
                                name: "String".into(),
                            },
                        },
                    ),
                    (
                        "_icontains".into(),
                        ndc_models::ComparisonOperatorDefinition::Custom {
                            argument_type: ndc_models::Type::Named {
                                name: "String".into(),
                            },
                        },
                    ),
                ]),
                extraction_functions: BTreeMap::new(),
            },
        ),
        (
            "Int".into(),
            ndc_models::ScalarType {
                representation: ndc_models::TypeRepresentation::Int32,
                aggregate_functions: BTreeMap::from_iter([
                    ("max".into(), ndc_models::AggregateFunctionDefinition::Max),
                    ("min".into(), ndc_models::AggregateFunctionDefinition::Min),
                ]),
                comparison_operators: BTreeMap::from_iter([(
                    "_eq".into(),
                    ndc_models::ComparisonOperatorDefinition::Equal,
                )]),
                extraction_functions: BTreeMap::new(),
            },
        ),
        (
            "Bool".into(),
            ndc_models::ScalarType {
                representation: ndc_models::TypeRepresentation::Boolean,
                aggregate_functions: BTreeMap::new(),
                comparison_operators: BTreeMap::from_iter([(
                    "eq".into(),
                    ndc_models::ComparisonOperatorDefinition::Custom {
                        argument_type: ndc_models::Type::Named {
                            name: "Bool".into(),
                        },
                    },
                )]),
                extraction_functions: BTreeMap::new(),
            },
        ),
        (
            "Actor_Name".into(),
            ndc_models::ScalarType {
                representation: ndc_models::TypeRepresentation::String,
                aggregate_functions: BTreeMap::new(),
                comparison_operators: BTreeMap::new(),
                extraction_functions: BTreeMap::new(),
            },
        ),
        (
            "HeaderMap".into(),
            ndc_models::ScalarType {
                representation: ndc_models::TypeRepresentation::JSON,
                aggregate_functions: BTreeMap::new(),
                comparison_operators: BTreeMap::new(),
                extraction_functions: BTreeMap::new(),
            },
        ),
    ])
}

pub(crate) fn object_types() -> BTreeMap<ndc_models::ObjectTypeName, ndc_models::ObjectType> {
    BTreeMap::from_iter([
        ("actor".into(), actor::definition()),
        ("city".into(), city::definition()),
        ("country".into(), country::definition()),
        (
            "evaluated_institution".into(),
            evaluated_institution::definition(),
        ),
        ("movie".into(), movie::definition()),
        ("genre".into(), genre::definition()),
        ("name_query".into(), name_query::definition()),
        ("institution".into(), institution::definition()),
        ("location".into(), location::definition()),
        ("staff_member".into(), staff_member::definition()),
        ("login_response".into(), login::definition_login_response()),
        (
            "session_response".into(),
            login::definition_session_response(),
        ),
        ("session_info".into(), login::definition_session_info()),
    ])
}
