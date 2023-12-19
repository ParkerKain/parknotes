use std::path::PathBuf;

struct Config {
    root_dir: PathBuf,
}

fn detect_clife_folder(config: &Config) -> bool {
    let exists = config.root_dir.try_exists();
    if exists.is_ok() {
        return exists.unwrap();
    } else {
        panic!("Failed to parse root dir {}", config.root_dir.display());
    }
}

fn main() {
    println!("Welcome to clife!");

    let config = Config {
        root_dir: PathBuf::from("/home/parker/.clife"),
    };

    if !detect_clife_folder(&config) {
        println!("No clife folder detected at {}", config.root_dir.display())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_clife_folder_exists() {
        let config = Config {
            root_dir: PathBuf::from("/home"),
        };
        let result: bool = detect_clife_folder(&config);
        assert_eq!(result, true)
    }

    #[test]
    fn test_detect_clife_folder_does_not_exist() {
        let config = Config {
            root_dir: PathBuf::from("~/nonsense_folder_ntuyfwntw/"),
        };
        let result: bool = detect_clife_folder(&config);
        assert_eq!(result, false)
    }
}
