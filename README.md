# skel
**Create and save skeleton directories**

## Usage
### Add a directory to the registry `skel [-a/--add] [FOLDER]`
Use the `-a` or `--add` flag to copy a directory to the configuration folder (located at `~/.config/skel`).

### Remove a directory from the registry `skel [-r/--remove] [FOLDERS...]`
Use the `-r` or `--remove` flag to remove directories from the configuration folder. You can specify multiple templates to delete.

### Create a new directory using a saved template `skel [TEMPLATE] [NEW_FOLDER_NAME (optional)]`
Use the `skel` program without any flags to create a new folder using the specified saved template. If you provide a second argument, it will be used as the name of the new folder (as opposed to the name of the template).

## Credits
Programmed in Rust. Uses the [home](https://crates.io/crates/home) crate.