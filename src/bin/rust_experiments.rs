use elkodon::global_config::DEFAULT_CONFIG_FILE;
use elkodon_bb_log::{debug, error, info, set_logger, trace, warn};
use elkodon_bb_posix::config::does_system_satisfy_posix_requirements;
use elkodon_bb_posix::config::ComplianceCheckMode;

use clap::Parser;
use elkodon_bb_posix::system_configuration::print_system_configuration;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct TransmissionData {
    x: i32,
    y: i32,
    funky: f64,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    #[clap(short, long, default_value_t = DEFAULT_CONFIG_FILE.to_string())]
    config_file: String,
}

fn main() {
    static LOGGER: elkodon_bb_log::logger::buffer::Logger =
        elkodon_bb_log::logger::buffer::Logger::new();

    assert!(set_logger(&LOGGER));

    print_system_configuration();
    println!();
    does_system_satisfy_posix_requirements(ComplianceCheckMode::Verbose);
    println!();

    //info!("file path: {}", IceoryxConfig::get().global.root_path);

    trace!("trace message");
    trace!(from "trace origin", "trace message");
    debug!("hello {}", 123);
    debug!(from "debug origin", "hello {}", 123);
    info!("hello");
    info!(from "info origin", "hello");
    warn!("blyubbu");
    warn!(from "warn origin", "blyubbu sad a sd asd sa d asd a sd a sd a sd as d asd a sd as d as d as d s d  d q wd as d as d asdas dasd as da sd a sd asd as d asd as d asd as d as da sd as da sd as d as d asd as d as d asd as d a sd as da sd as d asd da s as d xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjooooooooooooooooooooooooooooooooo");
    warn!(from "warn origin akskldjalksdjaklsdjalksjdlakjdlkasjdlkajdl assdjklas dlajsdlasjdlkasjd laksjdla ksdjaslkd jaslkdjas dlkajssdl asjsdl asjdlkasjd lkasjdl kajdlkasjd lkajslk jasdlkajd lasjdkasj ldjlasljkasdjasl djaljkdasljkdsaljkdasl jksadljk sald jklsdaljkdasjlkdsajlkljkdsaljkasdlkj daslkj ljksdalkjadslkjlakjlkj daslkj sadlkj dasljklkjdsaljkasdljkdasljklkjasdlkjsadjkladjslasj dsa djjasdlasjlk dj alksdjalskjdas lkdj",
        "blyubbu sad a sd asd sa d asd a sd a sd a sd as d asd a sd as d as d as d s d  d q wd as d as d asdas dasd as da sd a sd asd as d asd as d asd as d as da sd as da sd as d as d asd as d as d asd as d a sd as da sd as d asd da s as d xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjooooooooooooooooooooooooooooooooo");
    error!("bla {}", 1);
    error!(from "error origin", "bla {}", 1);

    let log_content = LOGGER.content();

    for entry in log_content {
        println!("{:?} {} {}", entry.log_level, entry.origin, entry.message);
    }
}
