pub fn test()-> String {
    String::from("sdlkfjskdljfl")
}

pub fn image_to_text() -> String{
    let image_path = String::from("img/book_page.png");
    let language = String::from("eng");
    let ocr_result = tesseract::ocr(&image_path, &language);
    let result = match ocr_result {
        Ok(result) => result,
        Err(error) => {
        panic!("Problem opening the file: {:?}", error)
        },
    };
    result
}
