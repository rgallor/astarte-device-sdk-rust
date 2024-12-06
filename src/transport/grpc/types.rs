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

use crate::interface::Ownership;
use crate::store::StoredProp;
use astarte_message_hub_proto::property::Value;
use astarte_message_hub_proto::AstarteDataTypeIndividual;
use std::collections::HashMap;

/// Wrapper for [StoredProperties](astarte_message_hub_proto::StoredProperties) proto message.
#[derive(Clone, Debug)]
pub struct StoredProperties {
    /// Protobuf properties.
    pub properties: astarte_message_hub_proto::StoredProperties,
}

impl From<astarte_message_hub_proto::StoredProperties> for StoredProperties {
    fn from(value: astarte_message_hub_proto::StoredProperties) -> Self {
        Self { properties: value }
    }
}

impl StoredProperties {
    /// Construct a [StoredProperties] object from a list of [StoredProp]s.
    pub fn from_props(props: Vec<StoredProp>) -> Self {
        let interface_properties =
            props
                .into_iter()
                .fold(HashMap::new(), |mut interface_properties, prop| {
                    let entry_prop =
                        interface_properties
                            .entry(prop.interface)
                            .or_insert_with(|| astarte_message_hub_proto::InterfaceProperties {
                                ownership: astarte_message_hub_proto::Ownership::from(
                                    prop.ownership,
                                ) as i32,
                                version_major: prop.interface_major,
                                properties: vec![],
                            });

                    let property = astarte_message_hub_proto::Property {
                        path: prop.path,
                        value: Some(Value::AstarteProperty(AstarteDataTypeIndividual::from(
                            prop.value,
                        ))),
                    };

                    entry_prop.properties.push(property);

                    interface_properties
                });

        Self {
            properties: astarte_message_hub_proto::StoredProperties {
                interface_properties,
            },
        }
    }
}

impl From<Ownership> for astarte_message_hub_proto::Ownership {
    fn from(value: Ownership) -> Self {
        match value {
            Ownership::Device => astarte_message_hub_proto::Ownership::Device,
            Ownership::Server => astarte_message_hub_proto::Ownership::Server,
        }
    }
}
