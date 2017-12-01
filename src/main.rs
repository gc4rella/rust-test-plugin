#[macro_use]
extern crate log;
extern crate serde;
extern crate env_logger;
extern crate log4rs;
extern crate openbaton;

use openbaton::openbaton::VimDriver;
use openbaton::entities::BaseVimInstance;


struct TestVimDriver(BaseVimInstance);

impl VimDriver for TestVimDriver {
    type T = BaseVimInstance;

    fn refresh(&self, vim_instance: BaseVimInstance) -> BaseVimInstance {
        println!("{:?}", vim_instance.name);
        vim_instance
    }
}

impl Clone for TestVimDriver {
    fn clone(&self) -> Self {
        return TestVimDriver(BaseVimInstance::new());
    }

    fn clone_from(&mut self, _source: &Self) {
//        self = &mut source.clone();
    }
}

fn main() {
    let vim_driver = TestVimDriver(BaseVimInstance::new());
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    openbaton::start_instances(1, vim_driver, "rust", "rust");
    info!("Started!");
}
