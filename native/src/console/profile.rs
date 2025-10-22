use std::fs::{create_dir_all, remove_dir_all};
use std::io;
use std::io::Write;
use std::path::PathBuf;

use anyhow::{Context, Result};
use fs_extra::dir::{CopyOptions, copy};
use log::{info, warn};
use ulid::Ulid;

use crate::components::profile::Profile;
use crate::console::app::{
    ProfileCreateCommand,
    ProfileListCommand,
    ProfileRemoveCommand,
    ProfileUpdateCommand,
};
use crate::console::{Run, store_value};
use crate::directories::ProjectDirs;
use crate::integrations;
use crate::integrations::IntegrationUninstallArgs;
use crate::storage::Storage;
use crate::utils::sanitize_string;

fn apply_profile_template(
    template: &Option<PathBuf>,
    profile: &Ulid,
    dirs: &ProjectDirs,
) -> Result<()> {
    if let Some(template) = template {
        let mut options = CopyOptions::new();
        options.content_only = true;
        options.overwrite = true;

        info!("Copying a profile template");
        let target = dirs.userdata.join("profiles").join(profile.to_string());
        create_dir_all(&target).context("Failed to create a profile directory")?;
        copy(template, target, &options).context("Failed to copy a profile template")?;
    }

    Ok(())
}

impl Run for ProfileListCommand {
    fn run(&self) -> Result<()> {
        let dirs = ProjectDirs::new()?;
        let storage = Storage::load(&dirs)?;

        for (_, profile) in storage.profiles {
            println!(
                "{:=^60}\nDescription: {}\nID: {}",
                format!(
                    " {} ",
                    sanitize_string(&profile.name.unwrap_or_else(|| "* Unnamed *".into()))
                ),
                sanitize_string(&profile.description.unwrap_or_else(|| "* Nothing *".into())),
                profile.ulid
            );

            if !profile.sites.is_empty() {
                println!("\nApps:");
            }

            for site in profile.sites {
                let site = storage.sites.get(&site).context("Profile with invalid web app")?;

                let url = if site.config.manifest_url.scheme() != "data" {
                    &site.config.manifest_url
                } else {
                    &site.config.document_url
                };

                println!("- {}: {} ({})", site.name(), url, site.ulid);
            }

            println!();
        }

        Ok(())
    }
}

impl Run for ProfileCreateCommand {
    fn run(&self) -> Result<()> {
        self._run()?;
        Ok(())
    }
}

impl ProfileCreateCommand {
    pub fn _run(&self) -> Result<Ulid> {
        let dirs = ProjectDirs::new()?;
        let mut storage = Storage::load(&dirs)?;

        info!("Creating the profile");

        let profile = Profile::new(self.name.clone(), self.description.clone());
        let ulid = profile.ulid;

        storage.profiles.insert(ulid, profile);
        storage.write(&dirs)?;

        apply_profile_template(&self.template, &ulid, &dirs)?;

        info!("Profile created: {ulid}");
        Ok(ulid)
    }
}

impl Run for ProfileRemoveCommand {
    fn run(&self) -> Result<()> {
        let dirs = ProjectDirs::new()?;
        let mut storage = Storage::load(&dirs)?;

        let profile = storage.profiles.get_mut(&self.id).context("Profile does not exist")?;

        if !self.quiet {
            warn!(
                "This will completely remove the profile and all associated web apps, including their data"
            );
            warn!("You might not be able to fully recover this action");

            print!("Do you want to continue (y/n)? ");
            io::stdout().flush()?;

            let mut confirm = String::new();
            io::stdin().read_line(&mut confirm)?;
            confirm = confirm.trim().into();

            if confirm != "Y" && confirm != "y" {
                info!("Aborting!");
                return Ok(());
            }
        }

        if profile.ulid == Ulid::nil() {
            warn!("Default profile cannot be completely removed");
            warn!("Web apps and data will be cleared, but the profile will stay");
        }

        info!("Removing directories");
        let _ = remove_dir_all(dirs.userdata.join("profiles").join(self.id.to_string()));

        info!("Removing web apps");
        for site in &profile.sites {
            if let Some(site) = storage.sites.remove(site) {
                integrations::uninstall(&IntegrationUninstallArgs { site: &site, dirs: &dirs })
                    .context("Failed to uninstall system integration")?;
            }
        }

        if profile.ulid != Ulid::nil() {
            info!("Removing the profile");
            storage.profiles.remove(&self.id);
        } else {
            profile.sites.clear();
        }

        storage.write(&dirs)?;

        info!("Profile removed!");
        Ok(())
    }
}

impl Run for ProfileUpdateCommand {
    fn run(&self) -> Result<()> {
        let dirs = ProjectDirs::new()?;
        let mut storage = Storage::load(&dirs)?;

        let profile = storage.profiles.get_mut(&self.id).context("Profile does not exist")?;

        info!("Updating the profile");
        store_value!(profile.name, self.name);
        store_value!(profile.description, self.description);
        storage.write(&dirs)?;

        apply_profile_template(&self.template, &self.id, &dirs)?;

        info!("Profile updated!");
        Ok(())
    }
}
