// Copyright 2017 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate exonum;
extern crate libloading;

use libloading::{Library, Symbol};

use exonum::helpers::fabric::{NodeBuilder, ServiceFactory};

//use exonum_configuration::ConfigurationService;

fn main() {
    exonum::helpers::init_logger().unwrap();

    let configuration_lib = Library::new("target/debug/libexonum_configuration.so")
        .expect("Unable to load library");
    let get_factory: Symbol<extern fn() -> Box<ServiceFactory>> = unsafe {
        configuration_lib
            .get(b"get_service_factory")
            .expect("Unable to find 'get_service_factory' function")
    };
    let service_factory = get_factory();

    NodeBuilder::new()
        .with_service(service_factory)
        .run();
}
