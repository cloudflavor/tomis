pub trait CloudProvisioner: Send + Sync {
    fn create_cluster(&self, name: String) -> Result<(), ()>;
}
