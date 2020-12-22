use structopt::StructOpt;

/// Easily triggering leave session actions: logout, reboot, shutdown, hibernate, and lock screen
#[derive(StructOpt, Debug)]
#[structopt(name = "LDE Leave")]
struct LeaveOpt {

    /// Logout.
    #[structopt(long)]
    logout: bool,

    /// Lockscreen.
    #[structopt(long)]
    lockscreen: bool,

    /// Suspend.
    #[structopt(long)]
    suspend: bool,

    /// Hibernate.
    #[structopt(long)]
    hibernate: bool,

    /// Shutdown.
    #[structopt(long)]
    shutdown: bool,

    /// Reboot.
    #[structopt(long)]
    reboot: bool,
}

fn main() {
    let opt = LeaveOpt::from_args();
    println!("{:#?}", opt);
}   