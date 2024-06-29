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

use anyhow::Result;
use structopt::StructOpt;
use tomis::server::TomisControlPlane;
use tomis::tomis_cloud;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = tomis::Opt::from_args();
    let address = format!("{}:{}", opts.address, opts.port).parse()?;
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(opts.log_level)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting server on {}", address);

    tonic::transport::Server::builder()
        .add_service(tomis_cloud::tomis_server::TomisServer::new(
            TomisControlPlane,
        ))
        .serve(address)
        .await?;

    Ok(())
}
