// This file is part of Astarte.
//
// Copyright 2025 SECO Mind Srl
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::time::Duration;

use astarte_device_sdk::prelude::PropAccess;
use astarte_device_sdk::{AstarteData, Client, DeviceEvent, Value};
use eyre::{bail, ensure, OptionExt};
use tracing::info;

use crate::api::ApiClient;
use crate::data::{all_type_data, InterfaceData};
use crate::AstarteClient;

#[derive(Debug)]
struct ServerProperty {}

impl InterfaceData for ServerProperty {
    fn interface() -> String {
        "org.astarte-platform.rust.e2etest.ServerProperty".to_string()
    }

    fn data() -> eyre::Result<HashMap<String, AstarteData>> {
        let data = all_type_data().map(|(path, value)| (format!("/sensor_1/{path}"), value));

        Ok(HashMap::from_iter(data))
    }
}

/// Send a value with a long integer > 2^53 + 1
#[derive(Debug)]
struct ServerDatastreamOverflow {}

impl InterfaceData for ServerDatastreamOverflow {
    fn interface() -> String {
        "org.astarte-platform.rust.e2etest.ServerProperty".to_string()
    }

    fn data() -> eyre::Result<HashMap<String, AstarteData>> {
        let mut data = HashMap::with_capacity(2);

        data.insert(
            "/overflow/longinteger_endpoint".to_string(),
            AstarteData::LongInteger(2i64.pow(55)),
        );
        data.insert(
            "/overflow/longintegerarray_endpoint".to_string(),
            AstarteData::LongIntegerArray(vec![2i64.pow(55); 4]),
        );

        Ok(data)
    }
}

async fn validate<T>(api: &ApiClient, client: &AstarteClient) -> eyre::Result<()>
where
    T: InterfaceData,
{
    let data_interface = T::interface();
    let data = T::data()?;

    for (data_path, exp) in data {
        api.send_individual(&data_interface, &data_path, &exp)
            .await?;

        let DeviceEvent {
            interface,
            path,
            data,
        } = tokio::time::timeout(Duration::from_secs(2), client.recv()).await??;

        ensure!(interface == data_interface);
        ensure!(path == data_path);

        let value = data
            .as_property()
            .ok_or_eyre("received invalid data type")?;

        ensure!(*value == Some(exp.clone()));

        let prop = client
            .property(&data_interface, &data_path)
            .await?
            .ok_or_eyre("missing server property")?;

        ensure!(prop == exp);

        api.unset(&data_interface, &data_path).await?;

        let DeviceEvent {
            interface,
            path,
            data,
        } = tokio::time::timeout(Duration::from_secs(2), client.recv()).await??;

        ensure!(interface == data_interface);
        ensure!(path == data_path);

        let Value::Property(None) = data else {
            bail!("prop was not unseted")
        };

        info!(interface, path, "validated")
    }

    Ok(())
}

pub(crate) async fn check(api: &ApiClient, client: &AstarteClient) -> eyre::Result<()> {
    validate::<ServerProperty>(api, client).await?;
    validate::<ServerDatastreamOverflow>(api, client).await?;

    Ok(())
}
