use const_format::concatcp;
use crate::helper::{find_sourcemod_path, InstallType};
use crate::version::RemoteVersionResponse;
use crate::wizard::BeansError;

pub mod depends;
pub mod helper;
pub mod wizard;
pub mod version;



#[cfg(not(windows))]
pub const DATA_DIR: &str = "/open_fortress/";
#[cfg(windows)]
pub const DATA_DIR: &str = "\\open_fortress\\";

pub const SOURCE_URL: &str = "https://beans.adastral.net/";
pub const UPDATE_HASH_URL_WINDOWS: &str = concatcp!(SOURCE_URL, "beans_sha512sum_windows");
pub const UPDATE_HASH_URL_LINUX: &str = concatcp!(SOURCE_URL, "beans_sha512sum_linux");
#[cfg(windows)]
pub const ARIA2C_BINARY: &[u8] = include_bytes!("../Binaries/aria2c.exe");
#[cfg(not(windows))]
pub const ARIA2C_BINARY: &[u8] = include_bytes!("../Binaries/aria2c");

#[cfg(windows)]
pub const BUTLER_BINARY: &[u8] = include_bytes!("../Binaries/butler.exe");
#[cfg(not(windows))]
pub const BUTLER_BINARY: &[u8] = include_bytes!("../Binaries/butler");


#[derive(Debug, Clone)]
pub struct RunnerContext
{
    pub sourcemod_path: String,
    pub remote_version_list: RemoteVersionResponse,
    pub current_version: Option<usize>
}
impl RunnerContext
{
    pub async fn create_auto() -> Result<Self, BeansError>
    {
        depends::try_write_deps();
        depends::try_install_vcredist();
        let sourcemod_path = match find_sourcemod_path() {
            Some(v) => v,
            None => {
                return Err(BeansError::SourceModLocationNotFound);
            }
        };
        let version_list = crate::version::get_version_list().await;

        if helper::install_state() == InstallType::OtherSource {
            version::update_version_file();
        }

        return Ok(Self
        {
            sourcemod_path,
            remote_version_list: version_list,
            current_version: crate::version::get_current_version()
        });
    }
}