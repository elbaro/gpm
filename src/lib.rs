#![allow(unused_imports, unused_variables, dead_code)]
#[macro_use]
extern crate duct;
use duct::cmd;
use std::error::Error;
use std::path::{Path, PathBuf};

struct Binary {
    name: String,
    path: PathBuf,
}
impl Binary {
    fn new<P:AsRef<Path>+Into<String>>(name: P) -> Result<Binary, Box<Error>> {
        if Path::new(name.as_ref()).exists() {
            let name = name.into();
            Ok(Binary {
                name: name.clone(),
                path: PathBuf::from(name),
            })
        } else {
            Binary::from_path_env(name)
        }
    }
    fn from_path_env<P:AsRef<Path>+Into<String>>(name: P) -> Result<Binary, Box<Error>> {
        for path in std::env::split_paths(&std::env::var("PATH").unwrap_or("".to_string())) {
            let p = PathBuf::from(path).join(name.as_ref());
            if p.exists() {
                return Ok(Binary{name: name.into(), path: p});
            }
        }
        Err(format!("not found: {}", name.into()).into())
    }
    // fn run(&[]) -> Result<> {

    // }
}

trait Package {}

trait PackageManager {
    type P;

    fn new() -> Result<Self, Box<Error>> where Self: std::marker::Sized;
    // fn package<S:AsRef<str>>(name:S) -> Result<Self::P, Box<Error>>;
    fn sync() -> Result<(), Box<Error>>;
    fn install<S:AsRef<str>>(&self, name: S) -> Result<(), Box<Error>>;
    fn uninstall<S:AsRef<str>>(&self, name: S) -> Result<(), Box<Error>>;
    fn is_installed<S:AsRef<str>>(&self, name: S) -> bool;

    fn remove<S:AsRef<str>>(&self, name: S) -> Result<(), Box<Error>> {
        self.uninstall(name)
    }
}

struct AptPackage{}
struct AptGet {binary: Binary}

impl PackageManager for AptGet {
    type P = AptPackage;
    fn new() -> Result<Self, Box<Error>> {
        Binary::from_path_env("dpkg-query")?;
        Ok(AptGet{ binary: Binary::from_path_env("apt-get")? })
    }
    fn sync() -> Result<(), Box<Error>> {
        let output = cmd!("apt-get", "update").read()?;
        println!("output {}", output);
        Ok(())
    }
    fn install<S:AsRef<str>>(&self, name: S) -> Result<(), Box<Error>> {
        // self.binary.run("install", "run");
        let output = cmd!("apt-get", "install", name.as_ref()).read()?;
        println!("output {}", output);
        Ok(())
    }
    fn uninstall<S:AsRef<str>>(&self, name: S) -> Result<(), Box<Error>> {
        let output = cmd!("apt-get", "uninstall", name.as_ref()).read()?;
        println!("output {}", output);
        Ok(())
    }
    fn is_installed<S:AsRef<str>>(&self, name: S) -> bool {
        cmd!("dpkg-query", "-W", "-f='${Status}'", name.as_ref()).read().is_ok()
    }
}

struct ArchPackage{}
struct Pacman {binary: Binary}

impl PackageManager for Pacman {
    type P = ArchPackage;
    fn new() -> Result<Self, Box<Error>> {
        Ok(Pacman{ binary: Binary::from_path_env("pacman")? })
    }
    fn sync() -> Result<(), Box<Error>> {
        let output = cmd!("pacman", "-Syy").read()?;
        Ok(())
    }
    fn install<S:AsRef<str>>(&self, name: S) -> Result<(), Box<Error>> {
        // self.binary.run("install", "run");
        let output = cmd!("pacman", "-S", name.as_ref()).read()?;
        println!("output {}", output);
        Ok(())
    }
    fn uninstall<S:AsRef<str>>(&self, name: S) -> Result<(), Box<Error>> {
        let output = cmd!("pacman", "-R", name.as_ref()).read()?;
        println!("output {}", output);
        Ok(())
    }
    fn is_installed<S:AsRef<str>>(&self, name: S) -> bool {
        cmd!("pacman", "-Q", name.as_ref()).read().is_ok()
    }
}

#[cfg(test)]
mod tests {
    use ::Pacman;
    use ::PackageManager;
    use ::Binary;
    #[test]
    fn pacman() {
        // Pacman::new().unwrap().install("pacman").unwrap();
        assert!(Pacman::new().unwrap().is_installed("pacman"));
    }

    #[test]
    fn binary_new() {
        assert!(Binary::from_path_env("ls").is_ok());
        assert!(Binary::from_path_env("sh").is_ok());
    }
}
