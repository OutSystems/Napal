use std::fs::File;
use std::time::Instant;
use std::path::Path;
use std::io::prelude::*;
use handlebars::{Handlebars};
use anyhow::{Context, Result};

use crate::parameters::Parameters;
use crate::statistics::Statistics;


pub fn generate_html(statistics: &Statistics, param: &Parameters) -> Result<()> {
    let start = Instant::now();
    let template_location = param.base_directory.join("templates").join("template.hbs");

    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("table", &template_location)
        .with_context(|| format!("Could not register handlebars template {:?}", &template_location))?;

    let base_path = Path::new(&param.target_directory);
    let index_path = base_path.join("index.htm");
    let index_content = handlebars.render("table", &statistics.jsonify()).unwrap();
    let mut index_file = File::create(&index_path)
        .with_context(|| format!("Could not create file {:?}", &index_path))?;
    index_file.write_all(index_content.as_bytes())
        .with_context(|| format!("Could not write to file {:?}", index_path))?;

    println!("Sequencial HTML generation: {:?}", start.elapsed());
    Ok(())
}