use failure::Fallible;

use rga::adapters::spawning::map_exe_error;
use rga::adapters::*;
use rga::args::*;


use std::process::Command;


fn main() -> Fallible<()> {
    env_logger::init();

    let (args, passthrough_args) = split_args()?;
    let adapters = get_adapters_filtered(&args.adapters)?;

    if args.list_adapters {
        println!("Adapters:\n");
        for adapter in adapters {
            let meta = adapter.metadata();
            let matchers = meta
                .fast_matchers
                .iter()
                .map(|m| match m {
                    FastMatcher::FileExtension(ext) => format!(".{}", ext),
                })
                .collect::<Vec<_>>()
                .join(", ");
            print!(
                " - {}\n     {}\n     Extensions: {}\n",
                meta.name, meta.description, matchers
            );
            println!("");
        }
        return Ok(());
    }

    let extensions = adapters
        .iter()
        .flat_map(|a| &a.metadata().fast_matchers)
        .filter_map(|m| match m {
            FastMatcher::FileExtension(ext) => Some(ext as &str),
        })
        .collect::<Vec<_>>()
        .join(",");

    let exe = std::env::current_exe().expect("Could not get executable location");
    let preproc_exe = exe.with_file_name("rga-preproc");
    let mut child = Command::new("rg")
        .arg("--no-line-number")
        .arg("--pre")
        .arg(preproc_exe)
        .arg("--pre-glob")
        .arg(format!("*.{{{}}}", extensions))
        .args(passthrough_args)
        .spawn()
        .map_err(|e| map_exe_error(e, "rg", "Please make sure you have ripgrep installed."))?;

    child.wait()?;
    Ok(())
}
