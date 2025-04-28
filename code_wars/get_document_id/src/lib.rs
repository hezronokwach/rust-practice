fn main() {
    // Example with successful chain
    let office_four = OfficeFour {
        document_id: Ok(42),
    };
    let office_three = OfficeThree {
        next_office: Ok(office_four),
    };
    let office_two = OfficeTwo {
        next_office: Ok(office_three),
    };
    let office_one = OfficeOne {
        next_office: Ok(office_two),
    };

    match office_one.get_document_id() {
        Ok(id) => println!("Document ID: {}", id), // Prints: Document ID: 42
        Err(e) => println!("Error: {:?}", e),
    }

    // Example with error in OfficeTwo
    let office_one_error = OfficeOne {
        next_office: Err(ErrorOffice::OfficeClosed(1)),
    };

    match office_one_error.get_document_id() {
        Ok(id) => println!("Document ID: {}", id),
        Err(e) => println!("Error: {:?}", e), // Prints: Error: OfficeClosed(1)
    }
}