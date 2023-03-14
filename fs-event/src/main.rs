use nix::sys::inotify::{AddWatchFlags, InitFlags, Inotify};
fn main() {
    let inotify = Inotify::init(InitFlags::empty()).unwrap();
    let path = "/tmp";
    let add_watch = inotify.add_watch(
        path,
        AddWatchFlags::IN_CREATE | AddWatchFlags::IN_CREATE | AddWatchFlags::IN_DELETE,
    );
    let mut _wd;
    match add_watch {
        Ok(wd) => _wd = wd,
        Err(error) => panic!("Problem while adding a watcher: {:?}", error),
    };
    let read_events = inotify.read_events();
    let events;
    match read_events {
        Ok(ev) => events = ev,
        Err(error) => panic!("Problem while reading watcher events: {:?}", error),
    };
    for event in events {
        if (event.mask & AddWatchFlags::IN_ISDIR).bits() > 0 {
            match event.mask {
                AddWatchFlags::IN_CREATE => {
                    println!("The directory {:#?} was created.\n", event.name.unwrap())
                }
                AddWatchFlags::IN_MODIFY => {
                    println!("The directory {:#?} was modified.\n", event.name.unwrap())
                }
                AddWatchFlags::IN_DELETE => {
                    println!("The directory {:#?} was deleted.\n", event.name.unwrap())
                }
                _ => println!("Unhandled event {:#?}", event.name.unwrap()),
            }
        } else {
            match event.mask {
                AddWatchFlags::IN_CREATE => {
                    println!("The file {:#?} was created.\n", event.name.unwrap())
                }
                AddWatchFlags::IN_MODIFY => {
                    println!("The file {:#?} was modified.\n", event.name.unwrap())
                }
                AddWatchFlags::IN_DELETE => {
                    println!("The file {:#?} was deleted.\n", event.name.unwrap())
                }
                _ => println!("Unhandled event {:#?}", event.name.unwrap()),
            }
        }
    }
}
