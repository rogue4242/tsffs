use anyhow::{bail, Result};
use simics_api::{
    call_python_module_function, clear_exception, free_attribute, init_command_line,
    init_environment, init_simulator, last_error, main_loop, make_attr_nil, make_attr_string_adopt,
    run_command, run_python, source_python,
    sys::{SIM_alloc_attr_list, SIM_attr_list_set_item, SIM_make_attr_list},
    AttrValue, InitArgs, SimException,
};
use std::{env::current_exe, path::Path};
use tracing::info;

pub mod home;

pub struct Simics {}

impl Simics {
    pub fn try_init(mut args: InitArgs) -> Result<()> {
        let exe = current_exe()?;
        let argv = &[exe.to_string_lossy()];
        init_environment(argv, false, false)?;
        init_simulator(&mut args);
        Ok(())
    }

    pub fn run() -> ! {
        main_loop()
    }

    pub fn interactive() -> ! {
        init_command_line();
        main_loop()
    }

    pub fn command<S: AsRef<str>>(command: S) -> Result<()> {
        info!("Running SIMICS command {}", command.as_ref());
        free_attribute(run_command(command)?);

        Ok(())
    }

    pub fn python<P: AsRef<Path>>(file: P) -> Result<()> {
        info!("Running SIMICS Python file {}", file.as_ref().display());
        source_python(file)
    }

    pub fn config<P: AsRef<Path>>(file: P) -> Result<()> {
        info!("Running SIMICS config {}", file.as_ref().display());

        // TODO: Figure out the C apis for doing this
        run_python(format!(
            "cli.global_cmds.run_script(script='{}')",
            file.as_ref().to_string_lossy()
        ))?;

        match clear_exception()? {
            SimException::NoException => Ok(()),
            exception => {
                bail!(
                    "Error running simics config: {:?}: {}",
                    exception,
                    last_error()
                );
            }
        }
    }
}