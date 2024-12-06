/*
 * This file is part of Astarte.
 *
 * Copyright 2024 SECO Mind Srl
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * SPDX-License-Identifier: Apache-2.0
 */

//! Sdk representation of the protobuf types

use crate::store::StoredProp;
use astarte_message_hub_proto::AstarteDataTypeIndividual;
use std::collections::HashMap;

/// Wrapper for [StoredProperties](astarte_message_hub_proto::StoredProperties) proto message.
#[derive(Clone, Debug)]
pub struct StoredProperties(pub astarte_message_hub_proto::StoredProperties);

impl From<astarte_message_hub_proto::StoredProperties> for StoredProperties {
    fn from(value: astarte_message_hub_proto::StoredProperties) -> Self {
        Self(value)
    }
}

impl StoredProperties {
    /// Construct a [StoredProperties] object from a list of [StoredProp]s.
    pub fn from_props(props: Vec<StoredProp>) -> Self {
        let mut interface_properties = HashMap::new();

        for prop in props {
            let StoredProp {
                interface,
                path,
                value,
                interface_major,
                ownership,
            } = prop;

            let entry_prop = interface_properties.entry(interface).or_insert_with(|| {
                astarte_message_hub_proto::InterfaceProperties {
                    ownership: ownership as i32, // TODO: check if a better conversion can be done
                    version_major: interface_major as u32, // TODO: could fail
                    properties: vec![],
                }
            });

            let property = astarte_message_hub_proto::Property {
                path,
                value: Some(AstarteDataTypeIndividual::from(value)),
            };

            entry_prop.properties.push(property);
        }

        Self(astarte_message_hub_proto::StoredProperties {
            interface_properties,
        })
    }
}
