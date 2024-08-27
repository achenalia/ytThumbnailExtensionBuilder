use std::fs::File;
use std::io::{Write, Read};
use colored::*;  // Add this line to import the colored crate

// Welcome the user and explain the program
fn welcome() {
    println!(
        "{}",
        "Welcome to the YouTube Thumbnail Extension Builder!"
            .bold().underline().red()
    );

    println!(
        "{}",
        "The purpose of this program is to help you build your own extension which injects a random image out of the given images into each video's thumbnail."
            .italic().white()
    );

    println!(
        "{}",
        "The images are recommended to be 664px x 365px and have a transparent background, but you can choose whichever size you want (though it may resize unexpectedly)."
            .italic().white()
    );

    println!(
        "{}",
        "To proceed, let's get some information about your extension:"
            .bold().white()
    );
}

// Accept user input or return an error
fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt.italic().white());
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

// Build the extension directory
fn dir_builder() -> Result<(), std::io::Error> {
    std::fs::create_dir_all("../../extension/assets/images")?;
    Ok(())
}

// Build the manifest for the extension
fn manifest_builder() -> Result<(), std::io::Error> {
    let name = get_input("Enter the name of your extension: ");
    let description = get_input("Enter a description of your extension: ");
    let version = get_input("Enter the version of your extension: ");

    let mut f = File::open("../../templates/manifestTemplate.json")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    drop(f); // Close the file before reopening in write mode

    let new_contents = contents
        .replace("extensionName", &name)
        .replace("extensionDescription", &description)
        .replace("1.0.0", &version);

    let mut f = File::create("../../extension/manifest.json")?;
    f.write_all(new_contents.as_bytes())?;

    Ok(())
}

// Build Firefox version of the main.js file
fn ff_builder(num_images: &str) -> Result<(), std::io::Error> {
    let mut f = File::open("../../templates/mainTemplateFF.js")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let new_contents = contents.replace("const numImages = 0;", &format!("const numImages = {};", num_images));
    let mut f = File::create("../../extension/main.js")?;
    f.write_all(new_contents.as_bytes())?;

    Ok(())
}

// Build Chrome version of the main.js file
fn chrome_builder(num_images: &str) -> Result<(), std::io::Error> {
    let mut f = File::open("../../templates/mainTemplateChrome.js")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let new_contents = contents.replace("const numImages = 0;", &format!("const numImages = {};", num_images));
    let mut f = File::create("../../extension/main.js")?;
    f.write_all(new_contents.as_bytes())?;
    Ok(())
}

// Build the main.js file
fn main_builder() -> Result<(), std::io::Error> {
    let browser_type = get_input("Enter the browser type you want to use (firefox, chrome): ");
    let num_images = get_input("Enter the number of images you want to use: ");
    println!("{}", "Please make sure to place these images in the /extension/assets/images directory before using your extension.".italic().white());

    if browser_type != "firefox" && browser_type != "chrome" {
        println!("{}", "Invalid browser type. Please enter 'firefox' or 'chrome'.".bold().red());
        std::process::exit(1);
    } else if browser_type == "firefox" {
        ff_builder(&num_images)?;
    } else if browser_type == "chrome" {
        chrome_builder(&num_images)?;
    }

    Ok(())
}

fn main() {
    welcome();
    if let Err(e) = dir_builder() {
        eprintln!("{}", format!("Failed to build directory: {}", e).bold().red());
    }
    if let Err(e) = manifest_builder() {
        eprintln!("{}", format!("Failed to build manifest: {}", e).bold().red());
    }
    if let Err(e) = main_builder() {
        eprintln!("{}", format!("Failed to build main.js: {}", e).bold().red());
    }
    println!("{}", "Your extension has been built! Press enter to exit this builder.".bold().white());
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
