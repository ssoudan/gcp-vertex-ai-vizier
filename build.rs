// Copyright 2022 Sebastien Soudan.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Builds GRPC client from the proto files.

fn main() -> std::io::Result<()> {
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_server(false)
        .compile(
            &[
                "protos/google/longrunning/operations.proto",
                "protos/google/cloud/aiplatform/v1/operation.proto",
                "protos/google/cloud/aiplatform/v1/vizier_service.proto",
                "protos/google/cloud/aiplatform/v1/study.proto",
                "protos/google/api/client.proto",
                "protos/google/api/http.proto",
                "protos/google/api/annotations.proto",
                "protos/google/api/field_behavior.proto",
                "protos/google/api/resource.proto",
                "protos/google/rpc/status.proto",
            ],
            &["protos"],
        )
}
