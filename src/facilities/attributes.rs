//! Describes all the attributes the server manages.

/// Provides a description of an attribute.
#[derive(Debug)]
pub struct AttributeDescription {
    /// The name of the attribute for the front end.
    name: &'static str,
    /// The values the attribute can have.
    domain: AttributeDomain,
    /// The path to the attribute in the database.
    database_path: &'static [&'static str],
}

impl AttributeDescription {
    /// Returns the name of this attribute.
    pub fn get_name(&self) -> &str {
        self.name
    }

    /// Returns the value of the attribute within the facility, if it exists.
    pub fn get_value(&self, facility: &serde_json::Value) -> Option<serde_json::Value> {
        let mut current_path = facility;

        for path_component in self.database_path {
            current_path = &current_path[path_component];
        }

        match current_path {
            serde_json::Value::Null => None,
            val if self.domain.is_valid_value(val) => Some(val.clone()),
            _ => None,
        }
    }
}

/// Represents all values that the attribute can have.
#[derive(Debug)]
pub enum AttributeDomain {
    /// The attribute values must be one of the elements in the given slice.
    Enum(&'static [&'static str]),
    /// The attribute values may either be `true` or `false`.
    Boolean,
}

impl AttributeDomain {
    /// Determines if the value is valid for the given attribute.
    pub fn is_valid_value(&self, value: &serde_json::Value) -> bool {
        match self {
            AttributeDomain::Enum(possible_values) if value.is_string() => {
                possible_values.contains(&value.as_str().unwrap())
            }
            AttributeDomain::Boolean if value.is_boolean() => true,
            _ => false,
        }
    }
}

/// Represents all the attributes the server manages.
pub static ATTRIBUTES: [AttributeDescription; 13] = [
    AttributeDescription {
        name: "wheelchairAccess",
        domain: AttributeDomain::Enum(&["noSteps", "oneStep", "multipleSteps"]),
        database_path: &[
            "properties",
            "accessibility",
            "accessibleWith",
            "wheelchair",
        ],
    },
    AttributeDescription {
        name: "gender",
        domain: AttributeDomain::Enum(&["female", "male", "unisex"]),
        database_path: &["properties", "accessibility", "gender"],
    },
    AttributeDescription {
        name: "facilityType",
        domain: AttributeDomain::Enum(&["public", "private"]),
        database_path: &["properties", "accessibility", "facilityType"],
    },
    AttributeDescription {
        name: "key",
        domain: AttributeDomain::Enum(&["euroKey", "radarKey", "askStaff", "none"]),
        database_path: &["properties", "accessibility", "key"],
    },
    AttributeDescription {
        name: "fee",
        domain: AttributeDomain::Boolean,
        database_path: &["properties", "accesibility", "fee"],
    },
    AttributeDescription {
        name: "spacious",
        domain: AttributeDomain::Boolean,
        database_path: &["properties", "accessibility", "spacious"],
    },
    AttributeDescription {
        name: "grabRail",
        domain: AttributeDomain::Enum(&["both", "left", "right", "none"]),
        database_path: &["properties", "accessibility", "grabRail"],
    },
    AttributeDescription {
        name: "lateralAccess",
        domain: AttributeDomain::Boolean,
        database_path: &["properties", "accessibility", "lateralAccess"],
    },
    AttributeDescription {
        name: "bottomClearance",
        domain: AttributeDomain::Boolean,
        database_path: &["properties", "accessibility", "bottomClearance"],
    },
    AttributeDescription {
        name: "sinkInsideCabin",
        domain: AttributeDomain::Boolean,
        database_path: &["properties", "accessibility", "sinkInsideCabin"],
    },
    AttributeDescription {
        name: "reachableControls",
        domain: AttributeDomain::Boolean,
        database_path: &["properties", "accessibility", "reachableControls"],
    },
    AttributeDescription {
        name: "emergencyCall",
        domain: AttributeDomain::Boolean,
        database_path: &["properties", "accessibility", "emergencyCall"],
    },
    AttributeDescription {
        name: "shower",
        domain: AttributeDomain::Boolean,
        database_path: &["properties", "accessibility", "shower"],
    },
];
