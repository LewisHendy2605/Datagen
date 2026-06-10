use super::error::ForgeError;

pub fn write(path: &str, _records: u32) -> Result<(), ForgeError> {
    // Load a font from the file system
    let font_family = genpdf::fonts::from_files(
        "C:\\Users\\lewis\\Documents\\fonts\\LiberationSans",
        "LiberationSans",
        None,
    )
    .expect("Failed to load font family");

    // Create a document and set the default font family
    let mut doc = genpdf::Document::new(font_family);

    // Change the default settings
    doc.set_title("Demo document");

    // Customize the pages
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    // Add one or more elements
    doc.push(genpdf::elements::Paragraph::new("This is a demo document."));

    // Render the document and write it to a file
    doc.render_to_file(path).expect("Failed to write PDF file");

    Ok(())
}
