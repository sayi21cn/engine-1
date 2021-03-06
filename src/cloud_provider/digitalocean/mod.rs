pub mod api_structs;
pub mod application;
pub mod common;
pub mod databases;
pub mod kubernetes;
pub(crate) mod router;

extern crate digitalocean;

use std::any::Any;

use digitalocean::DigitalOcean;

use crate::cloud_provider::{CloudProvider, Kind, TerraformStateCredentials};
use crate::error::{EngineError, EngineErrorCause};
use crate::models::{Context, Listener, Listeners};

pub struct DO {
    context: Context,
    id: String,
    organization_id: String,
    name: String,
    pub token: String,
    spaces_access_id: String,
    spaces_secret_key: String,
    terraform_state_credentials: TerraformStateCredentials,
    listeners: Listeners,
}

impl DO {
    pub fn new(
        context: Context,
        id: &str,
        organization_id: &str,
        token: &str,
        spaces_access_id: &str,
        spaces_secret_key: &str,
        name: &str,
        terraform_state_credentials: TerraformStateCredentials,
    ) -> Self {
        DO {
            context,
            id: id.to_string(),
            organization_id: organization_id.to_string(),
            name: name.to_string(),
            token: token.to_string(),
            spaces_access_id: spaces_access_id.to_string(),
            spaces_secret_key: spaces_secret_key.to_string(),
            terraform_state_credentials,
            listeners: vec![],
        }
    }

    pub fn client(&self) -> DigitalOcean {
        DigitalOcean::new(self.token.as_str()).unwrap()
    }
}

impl CloudProvider for DO {
    fn context(&self) -> &Context {
        &self.context
    }

    fn kind(&self) -> Kind {
        Kind::DO
    }

    fn id(&self) -> &str {
        self.id.as_str()
    }

    fn organization_id(&self) -> &str {
        self.organization_id.as_str()
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn is_valid(&self) -> Result<(), EngineError> {
        let client = DigitalOcean::new(&self.token);
        match client {
            Ok(_x) => Ok(()),
            Err(_) => {
                return Err(
                    self.engine_error(
                        EngineErrorCause::User("Your AWS account seems to be no longer valid (bad Credentials). \
                    Please contact your Organization administrator to fix or change the Credentials."),
                        format!("failed to login to Digital Ocean {}", self.name_with_id()))
                );
            }
        }
    }

    fn add_listener(&mut self, listener: Listener) {
        self.listeners.push(listener);
    }

    fn terraform_state_credentials(&self) -> &TerraformStateCredentials {
        &self.terraform_state_credentials
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
