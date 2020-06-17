use std::env;
use std::fs::read_dir;
// use std::fs::DirEntry;
// use std::fs::File;
// use std::io::Write;
use std::path::Path;

//use capnpc::CompilerCommand;


fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
		let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
		let src_path = Path::new(&src_dir);
    let dest = Path::new(&out_dir).join("rsmsg");

    //TODO use platform-independent path
    let raw_msgs_path = src_path.join("cpmsg");
		let raw_msgs_dir = read_dir(raw_msgs_path).unwrap();

    let mut cmd = capnpc::CompilerCommand::new();
		cmd.src_prefix("cpmsg");
		cmd.output_path(dest);
       
    for dir_entry in raw_msgs_dir {
			if let Ok(file_entry) = dir_entry {
				cmd.file(file_entry.path());
			}
    }
 
    cmd.run().expect("schema compilation failed");

}

