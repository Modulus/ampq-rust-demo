use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name="demo", about="Application variables")]
pub struct Options {
    #[structopt(short= "s" , env="SERVER", long = "server", default_value="locathost:5672")]
    pub server: String,

    #[structopt(short ="l", env = "LEVEL", long = "level", default_value="INFO")]
    pub level: String
}