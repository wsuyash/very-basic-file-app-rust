use std::{
    path::Path,
    env,
    time::Duration,
    sync::mpsc::channel,
    fs::{ File, self },
    io::{ Read, Write },
};
use notify::{ EventKind, Watcher, RecommendedWatcher, Config, event::ModifyKind };

fn main() {
    // commands
    const CREATE_FILE: &str = "create a file";
    const DELETE_FILE: &str = "delete the file";
    const RENAME_FILE: &str = "rename the file";
    const ADD_TO_FILE: &str = "add to the file";

    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(
        tx,
        Config::default().with_poll_interval(Duration::from_millis(1)).with_compare_contents(true)
    ).unwrap();

    let file_name = "command.txt";
    let path_string = String::from(
        format!("{}\\{file_name}", env::current_dir().unwrap().to_str().unwrap())
    );

    let path = Path::new(&path_string);

    watcher.watch(path, notify::RecursiveMode::NonRecursive).unwrap();

    for res in rx {
        match res {
            Ok(event) =>
                match event.kind {
                    EventKind::Modify(ModifyKind::Any) => {
                        let mut command_file_handler = File::open(path).unwrap();
                        // get file size
                        let file_size = command_file_handler.metadata().unwrap().len();

                        // allocate buffer with size of file
                        let mut buf = Vec::with_capacity(file_size as usize);

                        // read the content and fill buffer defined above
                        command_file_handler.read_to_end(&mut buf).unwrap();

                        let command = String::from_utf8(buf).unwrap();

                        // create a file:
                        // create a file <path>
                        if command.starts_with(CREATE_FILE) {
                            let file_path: String = command
                                .chars()
                                .skip(CREATE_FILE.len() + 1)
                                .collect();

                            create_file(file_path);
                        }

                        // delete a file:
                        // delete the file <path>
                        if command.starts_with(DELETE_FILE) {
                            let file_path: String = command
                                .chars()
                                .skip(DELETE_FILE.len() + 1)
                                .collect();

                            delete_file(file_path);
                        }

                        // rename a file
                        // rename the file <old> to <new>
                        if command.starts_with(RENAME_FILE) {
                            let idx = command.find(" to ").unwrap();

                            let old_path: String = command
                                .chars()
                                .skip(RENAME_FILE.len() + 1)
                                .take(idx - RENAME_FILE.len())
                                .collect();

                            let new_path: String = command
                                .chars()
                                .skip(idx + " to ".len())
                                .collect();

                            rename_file(old_path, new_path);
                        }

                        // add to file
                        // add to the file <path> this content: <content>
                        if command.starts_with(ADD_TO_FILE) {
                            let idx = command.find(" this content: ").unwrap();
                            let file_path: String = command
                                .chars()
                                .skip(ADD_TO_FILE.len() + 1)
                                .take(idx - ADD_TO_FILE.len())
                                .collect();

                            let content: String = command
                                .chars()
                                .skip(idx + " this content: ".len())
                                .collect();

                            add_to_file(file_path, content);
                        }
                    }
                    _ => (),
                }
            Err(e) => println!("Error: {:#?}", e),
        }
    }
}

fn create_file(file_path: String) -> () {
    let path = Path::new(&file_path);

    match File::open(path) {
        Ok(_) => println!("File {:?} already exists.", path),
        Err(_) => {
            match File::create(path) {
                Ok(_) => println!("File created."),
                Err(e) => println!("File could not be created. Error: {:#?}", e),
            }
        }
    }

    ()
}

fn delete_file(file_path: String) -> () {
    match fs::remove_file(Path::new(&file_path)) {
        Ok(()) => println!("File deleted."),
        Err(e) => println!("Could not delete the file. Error: {:#?}", e),
    }

    ()
}

fn rename_file(old_path: String, new_path: String) -> () {
    match fs::rename(Path::new(&old_path), Path::new(&new_path)) {
        Ok(()) => println!("File renamed."),
        Err(e) => println!("Could not rename the file. Error: {:#?}", e),
    }
}

fn add_to_file(file_path: String, content: String) -> () {
    let file_handle = File::options().append(true).open(Path::new(&file_path));

    match file_handle {
        Ok(mut file) => {
            match file.write_all(&mut content.as_bytes()) {
                Ok(()) => println!("Added to the file."),
                Err(e) => println!("Could not append to file. Error: {:#?}", e),
            }
        }
        Err(e) => println!("Could not append to file. Error: {:#?}", e),
    }

    ()
}
