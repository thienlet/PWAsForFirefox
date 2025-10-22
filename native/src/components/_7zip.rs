use std::ffi::OsStr;
use std::fs::remove_file;
use std::os::windows::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

use anyhow::{Context, Result, bail};
use cfg_if::cfg_if;
use const_format::formatcp;
use log::{info, warn};
use tempfile::Builder;
use windows::Win32::System::Com::{
    COINIT_APARTMENTTHREADED,
    COINIT_DISABLE_OLE1DDE,
    CoInitializeEx,
};
use windows::Win32::System::Threading::{GetExitCodeProcess, INFINITE, WaitForSingleObject};
use windows::Win32::UI::Shell::{
    SEE_MASK_NOASYNC,
    SEE_MASK_NOCLOSEPROCESS,
    SHELLEXECUTEINFOW,
    ShellExecuteExW,
};
use windows::core::{HSTRING, PCWSTR, w};
use windows_registry::LOCAL_MACHINE;

#[inline]
const fn get_download_url() -> &'static str {
    #[allow(unused_imports)]
    use const_format::formatcp;

    #[allow(dead_code)]
    const VERSION: &str = "2500";

    cfg_if! {
        if #[cfg(target_arch = "x86")] {
            const ARCHITECTURE: &str = "";
        } else if #[cfg(target_arch = "x86_64")] {
            const ARCHITECTURE: &str = "-x64";
        } else if #[cfg(target_arch = "aarch64")] {
            const ARCHITECTURE: &str = "-arm64";
        } else {
            panic!("Cannot install 7-Zip: Unsupported architecture!");
        }
    }

    formatcp!("https://7-zip.org/a/7z{VERSION}{ARCHITECTURE}.exe")
}

#[inline]
fn run_as_admin<S: AsRef<OsStr>>(cmd: S) -> std::io::Result<ExitStatus> {
    unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE).ok()? };

    let mut code = 1;
    let file = HSTRING::from(cmd.as_ref());

    let mut sei = SHELLEXECUTEINFOW {
        cbSize: std::mem::size_of::<SHELLEXECUTEINFOW>() as u32,
        fMask: SEE_MASK_NOASYNC | SEE_MASK_NOCLOSEPROCESS,
        lpVerb: w!("runas"),
        lpFile: PCWSTR(file.as_ptr()),
        nShow: 1,
        ..Default::default()
    };

    unsafe {
        ShellExecuteExW(&mut sei)?;
        let process = { sei.hProcess };

        if process.is_invalid() {
            return Err(std::io::Error::last_os_error());
        };

        WaitForSingleObject(process, INFINITE);
        GetExitCodeProcess(process, &mut code)?;
    };

    Ok(ExitStatus::from_raw(code))
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct _7Zip {
    pub version: Option<String>,
    executable: Option<PathBuf>,
}

impl _7Zip {
    pub fn new() -> Result<Self> {
        match Self::new_from_registry().context("Failed to search 7-Zip in registry")? {
            registry if registry.version.is_some() => Ok(registry),
            _ => Self::new_from_path().context("Failed to search 7-Zip in PATH variable"),
        }
    }

    fn new_from_registry() -> Result<Self> {
        let key = LOCAL_MACHINE.open(r"Software\Microsoft\Windows\CurrentVersion\Uninstall\7-Zip");

        let version;
        let executable;

        match key {
            Ok(key) => {
                let display_version = key.get_string("DisplayVersion")?;
                let install_location = key.get_string("InstallLocation")?;

                version = Some(display_version);
                executable = Some(PathBuf::from(install_location).join("7z.exe"));
            }
            Err(_) => {
                version = None;
                executable = None;
            }
        }

        Ok(Self { version, executable })
    }

    fn new_from_path() -> Result<Self> {
        let exe = std::env::var_os("PATH").and_then(|paths| {
            std::env::split_paths(&paths)
                .filter_map(|directory| {
                    let executable = directory.join("7z.exe");
                    match executable.is_file() {
                        true => Some(executable),
                        false => None,
                    }
                })
                .next()
        });

        match exe {
            Some(exe) => Ok(Self { version: Some("0.0.0".into()), executable: Some(exe) }),
            None => Ok(Self { version: None, executable: None }),
        }
    }

    pub fn install(self) -> Result<()> {
        const TEMP_FILE_ERROR: &str = "Failed to create a temporary file";
        const DOWNLOAD_ERROR: &str = "Failed to download the 7-Zip installer";
        const EXEC_ERROR: &str = "Failed to execute the 7-Zip installer";
        const CLEANUP_ERROR: &str = "Failed to clean up the 7-Zip installer";

        warn!("This will install 7-Zip on your system");
        warn!("7-Zip is made by Igor Pavlov, and licensed under the GNU LGPL license and others");
        warn!("This project is not affiliated with the 7-Zip project or its developers in any way");
        warn!("7-Zip License: https://7-zip.org/license.txt");
        warn!("7-Zip Website: https://7-zip.org/");

        let mut installer = Builder::new()
            .prefix("firefoxpwa-7zip-")
            .suffix(".exe")
            .tempfile()
            .context(TEMP_FILE_ERROR)?;

        info!("Downloading the 7-Zip installer");
        let mut response = reqwest::blocking::get(get_download_url()).context(DOWNLOAD_ERROR)?;
        (response.copy_to(&mut installer.as_file_mut())).context(DOWNLOAD_ERROR)?;
        let (_, path) = installer.keep().context(DOWNLOAD_ERROR)?;

        info!("Executing the 7-Zip installer");
        warn!("Please follow the installer to install 7-Zip");
        warn!("You might need to accept the User Account Control prompt");

        if !run_as_admin(&path).context(EXEC_ERROR)?.success() {
            bail!(EXEC_ERROR)
        }

        remove_file(path).context(CLEANUP_ERROR)?;

        info!("7-Zip installed!");
        Ok(())
    }

    #[inline]
    pub fn run(&self, args: Vec<&str>) -> Result<ExitStatus> {
        let executable = match &self.executable {
            Some(executable) => executable,
            None => bail!("7-Zip is currently not installed"),
        };

        Ok(Command::new(executable).args(args).status()?)
    }
}
