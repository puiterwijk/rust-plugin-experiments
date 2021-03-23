use anyhow::{Context, Result};
use libloading::{Library, os::unix::Symbol};

#[derive(Debug)]
struct Plugin {
    library: Library,

    // Symbols
    activate: Symbol<unsafe extern fn() -> u32>,
}

impl Plugin {
    pub fn load(path: &str) -> Result<Self> {
        let library = unsafe {
            Library::new(&path)
        }
        .with_context(|| format!("Error loading library at path {}", path))?;

        let activate = unsafe {
            library.get::<unsafe extern fn() -> u32>(b"activate\0")
                .context("Error loading symbol 'activate'")?
                .into_raw()
        };

        Ok(Plugin {
            library,

            // Symbols
            activate,
        })
    }

    fn activate(&self) -> u32 {
        let func = &self.activate;

        unsafe {
            func()
        }
    }
}

fn load(name: &str) -> Result<Plugin> {
    let libguest = Plugin::load(&format!("target/debug/libguest_{}.so", name)).with_context(|| format!("Error loading libguest_{}", name))?;
    println!("libguest_{}: {:?}", name, libguest);
    Ok(libguest)
}

fn call_activate(name: &str, plugin: &Plugin) {
    println!("- Calling libguest_{}->activate:", name);
    let result = plugin.activate();
    println!("-- result: {:?}", result);
}

fn main() -> Result<()> {
    println!("---- Loading from here ----");
    let libguest_rust = load("rust")?;
    let libguest_c = load("c")?;
    let libguest_go = load("go")?;


    // Activate calls
    println!("---- Calling activate from here ----");
    call_activate("rust", &libguest_rust);
    call_activate("c", &libguest_c);
    call_activate("go", &libguest_go);

    Ok(())
}
