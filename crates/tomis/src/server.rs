// Copyright 2024 Cloudflavor GmbH

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::tomis_cloud;
use crate::tomis_cloud::tomis_server::Tomis as TomisServer;
use tonic::{async_trait, Request, Response, Status};

#[derive(Debug, Clone)]
pub struct TomisControlPlane;

#[async_trait]
impl TomisServer for TomisControlPlane {
    async fn get_cluster(
        &self,
        _request: Request<tomis_cloud::GetClusterRequest>,
    ) -> Result<Response<tomis_cloud::GetClusterResponse>, Status> {
        unimplemented!()
    }

    async fn list_clusters(
        &self,
        _request: Request<tomis_cloud::ListClustersRequest>,
    ) -> Result<Response<tomis_cloud::ListClustersResponse>, Status> {
        unimplemented!()
    }

    async fn create_cluster(
        &self,
        _request: Request<tomis_cloud::CreateClusterRequest>,
    ) -> Result<Response<tomis_cloud::CreateClusterResponse>, Status> {
        unimplemented!()
    }

    async fn update_cluster(
        &self,
        _request: Request<tomis_cloud::UpdateClusterRequest>,
    ) -> Result<Response<tomis_cloud::UpdateClusterResponse>, Status> {
        unimplemented!()
    }

    async fn delete_cluster(
        &self,
        _request: Request<tomis_cloud::DeleteClusterRequest>,
    ) -> Result<Response<tomis_cloud::DeleteClusterResponse>, Status> {
        unimplemented!()
    }
}
