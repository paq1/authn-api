pub trait EnvService: Send + Sync {
    fn get_salt(&self) -> String;
}