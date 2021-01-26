use std::fs;
use std::str;
use scraper::{Html,Selector};
use std::io::Write;

fn main() -> std::result::Result<(),Box<dyn std::error::Error + 'static>> {
	
	// let dirname = r"C:\Users\user\Desktop\unreal 2\VR quick start\OEBPS\";
	// let book_title = "VR ";

	let dirname = r"C:\Users\user\Desktop\unreal 2\scripting cookbook\";
	let book_title = "Script ";

	// let dirname = r"C:\Users\user\Desktop\unreal 2\learning c++ by sharan violin\";
	// let book_title = "C++ ";

//	let book_title = "BPs ";
//	let dirname = r"C:\Users\user\Desktop\unreal 2\blueprints project\OEBPS\";


    let filename = dirname.to_owned() + "toc.xhtml";
    let mut toc_text = fs::read_to_string(filename)?;

    toc_text = str::replace(&toc_text, "chapter section ", "");
    toc_text = str::replace(&toc_text, "&amp;#xA0;", " ");

    let doc = Html::parse_document(&toc_text);

    let sel_li = Selector::parse("li.chapter").unwrap();

    let sel_a = Selector::parse("a").unwrap();

    let chapters = doc.select(&sel_li);

    // for el in chapters {
    //     println!("Hello, wooorld! {:?}", el.html());
    // }

    let mut i = 0;

    for chap in chapters {
   		i+=1;

//    	let i: i32 = chap;
   		let mut anchor = chap.select(&sel_a);
   		let chapter_title = anchor.next().unwrap().inner_html();
   		let mut chapter_filename = format!("{:02} {}.html", i, 
   			&chapter_title);

   		let mut chapter_text: String = r#"<!DOCTYPE html>
			<html dir="ltr" lang="en">
			<meta charset="utf-8" />
			<head><title>"#.to_string();
		chapter_text.push_str(format!("{}{} {}", book_title, i, &chapter_title).as_str());
		chapter_text.push_str("</title></head><body>\n\n");

		let anchors = chap.select(&sel_a);

		println!("Chapter title: {:?}", chapter_filename);


		for a in anchors {
			let chapname = format!("{}{}", dirname, a.value().attr("href").unwrap());

			let section = trim_section(&chapname);

			chapter_text.push_str(&section);
		}

//		println!("Chapter text: {:?}", chapter_text);
	    chapter_filename = str::replace(&chapter_filename, ":", "");

		let output_filename = format!("{}{}", dirname, chapter_filename);
println!("output: {}", output_filename);

	    let mut file = fs::File::create(output_filename)?;
	    file.write_all(chapter_text.as_bytes())?;
    }

    Ok(())
}

fn trim_section(filename: &str) -> String {

		let text = match fs::read_to_string(filename) {
			  Result::Ok(val) => val,
			  Result::Err(err) =>
			  	panic!("error while opening file: {:?} {:?}", filename, err),
			};

		let start = text.find("<body>").expect("open body tag not found");
		let end = text.find("</body>").expect("close body tag not found");

		return text[start+6..end].to_string();
}