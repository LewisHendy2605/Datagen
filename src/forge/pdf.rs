use super::error::ForgeError;

pub fn write(path: &str, records: u32) -> Result<(), ForgeError> {
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

    for i in 0..records {
        if i > 0 {
            doc.push(genpdf::elements::PageBreak::new());
        }
        doc.push(genpdf::elements::Paragraph::new(format!(
            "Record {}",
            i + 1
        )));
    }

    // Render the document and write it to a file
    doc.render_to_file(path).expect("Failed to write PDF file");

    Ok(())
}
