use crate::cloud::CloudProvisioner;

struct HetznerProvider {
    api_token: String,
}

impl CloudProvisioner for HetznerProvider {
    fn create_cluster(&self, name: String) -> Result<(), ()> {
        Ok(())
    }
}
