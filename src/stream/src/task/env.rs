// Copyright 2022 Singularity Data
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use risingwave_common::config::StreamingConfig;
use risingwave_common::util::addr::HostAddr;
use risingwave_source::dml_manager::DmlManagerRef;
use risingwave_source::{TableSourceManager, TableSourceManagerRef};
use risingwave_storage::StateStoreImpl;

pub(crate) type WorkerNodeId = u32;

/// The global environment for task execution.
/// The instance will be shared by every task.
#[derive(Clone, Debug)]
pub struct StreamEnvironment {
    /// Endpoint the stream manager listens on.
    server_addr: HostAddr,

    /// Endpoint of the source connector node
    connector_source_endpoint: String,

    /// Reference to the source manager.
    source_manager: TableSourceManagerRef,

    /// Streaming related configurations.
    config: Arc<StreamingConfig>,

    /// Current worker node id.
    worker_id: WorkerNodeId,

    /// State store for table scanning.
    state_store: StateStoreImpl,

    /// Manages dml information.
    dml_manager: DmlManagerRef,
}

impl StreamEnvironment {
    pub fn new(
        source_manager: TableSourceManagerRef,
        server_addr: HostAddr,
        connector_source_endpoint: String,
        config: Arc<StreamingConfig>,
        worker_id: WorkerNodeId,
        state_store: StateStoreImpl,
        dml_manager: DmlManagerRef,
    ) -> Self {
        StreamEnvironment {
            server_addr,
            connector_source_endpoint,
            source_manager,
            config,
            worker_id,
            state_store,
            dml_manager,
        }
    }

    // Create an instance for testing purpose.
    #[cfg(test)]
    pub fn for_test() -> Self {
        use risingwave_source::dml_manager::DmlManager;
        use risingwave_storage::monitor::StateStoreMetrics;
        StreamEnvironment {
            server_addr: "127.0.0.1:5688".parse().unwrap(),
            connector_source_endpoint: "127.0.0.1:60061".parse().unwrap(),
            source_manager: Arc::new(TableSourceManager::default()),
            config: Arc::new(StreamingConfig::default()),
            worker_id: WorkerNodeId::default(),
            state_store: StateStoreImpl::shared_in_memory_store(Arc::new(
                StateStoreMetrics::unused(),
            )),
            dml_manager: Arc::new(DmlManager::default()),
        }
    }

    pub fn server_address(&self) -> &HostAddr {
        &self.server_addr
    }

    pub fn source_manager(&self) -> &TableSourceManager {
        &self.source_manager
    }

    pub fn source_manager_ref(&self) -> TableSourceManagerRef {
        self.source_manager.clone()
    }

    pub fn config(&self) -> &StreamingConfig {
        self.config.as_ref()
    }

    pub fn worker_id(&self) -> WorkerNodeId {
        self.worker_id
    }

    pub fn state_store(&self) -> StateStoreImpl {
        self.state_store.clone()
    }

    pub fn connector_source_endpoint(&self) -> String {
        self.connector_source_endpoint.clone()
    }

    pub fn dml_manager_ref(&self) -> DmlManagerRef {
        self.dml_manager.clone()
    }
}
