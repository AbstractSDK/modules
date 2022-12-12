use std::env;
use std::sync::Arc;



use boot_core::{DaemonOptionsBuilder, networks};
use boot_core::prelude::{instantiate_daemon_env};


use boot_core::networks::NetworkInfo;




// use template_app::msg::ConfigResponse;





const NETWORK: NetworkInfo = networks::UNI_5;

// To deploy the app we need to get the memory and then register it
// We can then deploy a test OS that uses that new app

const _MODULE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn deploy_extension() -> anyhow::Result<()> {
    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());

    let daemon_options = DaemonOptionsBuilder::default()
        .network(NETWORK)
        .build()?;

    // Setup the environment
    let (_sender, _chain) = instantiate_daemon_env(&rt, daemon_options)?;

    // Load Abstract Version Control
    let _version_control_address: String =
        env::var("VERSION_CONTROL_ADDRESS").expect("VERSION_CONTROL_ADDRESS must be set");

    // TODO: abstract-boot is not yet updated
    // let version_control = VersionControl::load(
    //     &chain,
    //     &Addr::unchecked(version_control_address),
    // );
    //
    // // Load Abstract Version Control
    // let ans_address: String =
    //     env::var("ANS_ADDRESS").expect("ANS_ADDRESS must be set");
    //
    // let ans_host = AnsHost::load(
    //     &chain,
    //     &Addr::unchecked(ans_address),
    // );
    //
    // // Upload and register your module
    // let mut extension = CwStakingExtension::new(&CW_STAKING, &chain);
    // let module_version = Version::parse(MODULE_VERSION)?;
    //
    // let extension_init_msg = extension::InstantiateMsg {
    //     base: extension::BaseInstantiateMsg {
    //         ans_host_address: ans_host.address()?.into_string(),
    //         version_control_address: version_control.address()?.into_string(),
    //     },
    //     app: Empty {},
    // };
    // version_control.upload_and_register_extension(&mut extension.as_instance_mut(), &extension_init_msg, &module_version)?;


    // Example queries
    // app.query_base(BaseQueryMsg::Admin {})?;

    // let app_config: ConfigResponse = app.query_app(TemplateQueryMsg::Config {})?;

    // TODO: Attach to an OS

    Ok(())
}

fn main() {
    dotenv().ok();
    env_logger::init();

    use dotenv::dotenv;

    if let Err(ref err) = deploy_extension() {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));

        // The backtrace is not always generated. Try to run this example
        // with `$env:RUST_BACKTRACE=1`.
        //    if let Some(backtrace) = e.backtrace() {
        //        log::debug!("backtrace: {:?}", backtrace);
        //    }

        ::std::process::exit(1);
    }
}
