use std::fs::File;
use std::io::Write;

// welcome the user and explain the program:
fn welcome() {
    println!("
Welcome to the Youtube Thumbnail Extension Builder! \n
The purpose of this program is to help you build your own extension which injects a a random image out of the given images into each video's thumbnail. \n
The images are recommended to be 664px x 365px and have a transparent background, but you can choose whichever size you want (though it may resize unexpectedly). \n
To proceed, let's get some information about your extension: \n
    ");
}

// accept user input else return an error:
fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()  // Trimming the input
}

// build the extension directory:
fn dir_builder() -> Result<(), std::io::Error> {
    std::fs::create_dir_all("../../extension/assets/images")?;
    Ok(())
}

// build the manifest for the extension:
fn manifest_builder() -> Result<(), std::io::Error> {
    let name = get_input("Enter the name of your extension: ");
    let description = get_input("Enter a description of your extension: ");
    let version = get_input("Enter the version of your extension: ");

    let mut f = File::open("../../templates/manifestTemplate.json")?;
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut f, &mut contents)?;
    
    drop(f); // Close the file before reopening in write mode

    let new_contents = contents
        .replace("extensionName", &name)
        .replace("extensionDescription", &description)
        .replace("1.0.0", &version);

    let mut f = File::create("../../extension/manifest.json")?;
    f.write_all(new_contents.as_bytes())?;

    Ok(())
}

// build the main.js file:
fn main_builder() -> Result<(), std::io::Error> {
    let num_images = get_input("Enter the number of images you want to use: ");
    println!("Please make sure to place these images in the /extension/assets/images directory before using your extension.");

    let mut f = File::open("../../templates/mainTemplate.js")?;
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut f, &mut contents)?;
    
    let new_contents = contents.replace("const numImages = 0;", &format!("const numImages = {};", num_images));
    let mut f = File::create("../../extension/main.js")?;
    f.write_all(new_contents.as_bytes())?;

    Ok(())
}

fn main() {
    welcome();
    if let Err(e) = dir_builder() {
        eprintln!("Failed to build directory: {}", e);
    }
    if let Err(e) = manifest_builder() {
        eprintln!("Failed to build manifest: {}", e);
    }
    if let Err(e) = main_builder() {
        eprintln!("Failed to build main.js: {}", e);
    }
    eprintln!("Your extension has been built! Press enter to exit this builder.");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}


