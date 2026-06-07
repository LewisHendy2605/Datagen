use fake::Fake;
use fake::faker::address::en::*;
use fake::faker::creditcard::en::CreditCardNumber;
use fake::faker::filesystem::ar_sa::FileExtension;
use fake::faker::filesystem::en::FileName;
use fake::faker::filesystem::en::FilePath;
use fake::faker::http::en::RfcStatusCode;
use fake::faker::internet::en::*;
use fake::faker::lorem::en::Word;
use fake::faker::name::en::*;
use fake::faker::phone_number::ar_sa::PhoneNumber;
use fake::faker::time::ar_sa::Date;
use fake::faker::time::ar_sa::DateTime;
use fake::faker::time::ar_sa::Time;
use fake::uuid::UUIDv4;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;

// Opens file and returns file handle
fn open_file(path: &str) -> Result<File, io::Error> {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
}

// Generates and writes csv file from input params
pub fn write_csv(path: &str, lines: u32, delim: u8, columns: Vec<String>) -> Result<(), io::Error> {
    let file = open_file(&path)?;
    let mut writer = csv::WriterBuilder::new().delimiter(delim).from_writer(file);

    let mut headers: Vec<String> = Vec::new();
    let mut types: Vec<String> = Vec::new();

    // Parse schema
    for col in columns {
        let (col_name, col_type) = col.split_once(":").unwrap_or((&col, "string"));

        headers.push(col_name.to_string());
        types.push(col_type.to_string());
    }

    writer.write_record(headers)?;

    // Data
    for i in 1..=lines {
        // Build row
        let mut row: Vec<String> = Vec::new();
        for t in &types {
            let col = match t.as_str() {
                "index" => i.to_string(),
                "uuid" => UUIDv4.fake(),

                "char" => {
                    let w: String = Word().fake();
                    w.chars().next().unwrap_or('a').to_string()
                }
                "quoted" => Word().fake::<String>() + &(delim as char).to_string() + Word().fake(),

                "post_code" => PostCode().fake(),
                "zip_code" => ZipCode().fake(),
                "country_name" => CountryName().fake(),
                "city_name" => CityName().fake(),
                "street_name" => StreetName().fake(),
                "building_number" => BuildingNumber().fake(),

                "title" => Title().fake(),
                "full_name" => Name().fake(),
                "first_name" => FirstName().fake(),
                "last_name" => LastName().fake(),
                "username" => Username().fake(),
                "password" => Password(8..20).fake(),

                "ip" => IPv4().fake(),
                "email" => SafeEmail().fake(),
                "phone" => PhoneNumber().fake(),
                "credit_card_number" => CreditCardNumber().fake(),

                "date" => Date().fake(),
                "time" => Time().fake(),
                "date_time" => DateTime().fake(),

                "file_path" => FilePath().fake(),
                "file_name" => FileName().fake(),
                "file_ext" => FileExtension().fake(),

                "http_status_code" => RfcStatusCode().fake(),
                "rfc_status_code" => RfcStatusCode().fake(),

                _ => String::new(),
            };

            row.push(col)
        }

        writer.write_record(row)?;
    }

    Ok(())
}
