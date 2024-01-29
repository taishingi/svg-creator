use std::fs;
use std::io::Write;
use std::path::Path;

pub struct Svg {
    svg: String,
    view_box: String,
    width: i32,
    height: i32,
    id: String,
}

impl Svg {
    ///
    /// # Constructor
    ///
    /// - `width`       The svg width
    /// - `height`      The svg height
    /// - `view_box`    The svg viewBox
    ///
    #[must_use]
    pub fn new(width: i32, height: i32, view_box: String, id: String) -> Self {
        Self {
            svg: String::new(),
            view_box,
            width,
            height,
            id,
        }
    }

    ///
    /// # Start the svg
    ///
    pub fn start(&mut self) -> &mut Self {
        if self.view_box.is_empty() {
            self.svg.push_str(format!("<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" width=\"{}\" height=\"{}\" role=\"img\" id=\"{}\">", self.width, self.height,self.id).as_str());
        } else {
            self.svg.push_str(format!("<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" width=\"{}\" height=\"{}\" viewBox=\"{}\" role=\"img\" id=\"{}\">", self.width, self.height,self.view_box,self.id).as_str());
        }
        self
    }

    ///
    /// # Start a circle
    ///
    pub fn circle(&mut self) -> &mut Self {
        self.svg.push_str("<circle ");
        self
    }

    ///
    /// # Close the opened svg tag
    ///
    pub fn close(&mut self) -> &mut Self {
        self.svg.push_str("/>");
        self
    }
    ///
    /// # Start a group
    ///
    pub fn g(&mut self) -> &mut Self {
        self.svg.push_str("<g ");
        self
    }
    ///
    /// # Start a link
    ///
    pub fn a(&mut self) -> &mut Self {
        self.svg.push_str("<a ");
        self
    }

    ///
    /// # Close a link
    ///
    pub fn a_end(&mut self) -> &mut Self {
        self.svg.push_str("</a> ");
        self
    }

    ///
    /// # Start a text
    ///
    pub fn text(&mut self) -> &mut Self {
        self.svg.push_str("<text ");
        self
    }

    ///
    /// # Close a text
    ///
    pub fn text_end(&mut self) -> &mut Self {
        self.svg.push_str("</text> ");
        self
    }

    ///
    /// # Add a class
    ///
    /// - `class` The class name
    ///
    pub fn class(&mut self, class: &str) -> &mut Self {
        self.svg.push_str(format!("class=\"{class}\" ").as_str());
        self
    }

    ///
    /// # Close a tag
    ///
    pub fn tag_close(&mut self) -> &mut Self {
        self.svg.push_str("> ");
        self
    }

    ///
    /// # Close a rect
    ///
    pub fn rec_end(&mut self) -> &mut Self {
        self.svg.push_str("</rect> ");
        self
    }

    ///
    /// # Set a duration
    ///
    /// - `duration`    The duration
    ///
    pub fn dur(&mut self, duration: &str) -> &mut Self {
        self.svg.push_str(format!("dur=\"{duration}\" ").as_str());
        self
    }
    ///
    /// # Set the beginning of the duration
    ///
    /// - `duration` The beginning value
    ///
    pub fn begin(&mut self, duration: &str) -> &mut Self {
        self.svg.push_str(format!("begin=\"{duration}\" ").as_str());
        self
    }

    ///
    /// # Define the restart policy
    ///
    /// - `restart` The restart policy value
    ///
    pub fn restart(&mut self, restart: &str) -> &mut Self {
        self.svg
            .push_str(format!("restart=\"{restart}\" ").as_str());
        self
    }
    ///
    /// # The maximum value
    ///
    /// - `to` The max value
    ///
    pub fn to(&mut self, to: &str) -> &mut Self {
        self.svg.push_str(format!("to=\"{to}\" ").as_str());
        self
    }

    ///
    /// # Set the attribute name
    ///
    /// - `name` The name
    ///
    pub fn attribute_name(&mut self, name: &str) -> &mut Self {
        self.svg
            .push_str(format!("attributeName=\"{name}\" ").as_str());
        self
    }

    ///
    /// # Set the attribute content
    ///
    /// - `data` The value
    ///
    pub fn content(&mut self, data: &str) -> &mut Self {
        self.svg.push_str(data);
        self
    }

    ///
    /// # Set the text-anchor position
    ///
    /// - `anchor` The position
    ///
    pub fn text_anchor(&mut self, anchor: &str) -> &mut Self {
        self.svg
            .push_str(format!("text-anchor=\"{anchor}\" ").as_str());
        self
    }

    ///
    /// # Set the attribute type
    ///
    /// - `t` The type
    ///
    pub fn attribute_type(&mut self, t: &str) -> &mut Self {
        self.svg
            .push_str(format!("attributeType=\"{t}\" ").as_str());
        self
    }

    ///
    /// # Set the repeat count attribute
    ///
    /// - `t`   The value
    ///
    pub fn repeat_count(&mut self, t: &str) -> &mut Self {
        self.svg.push_str(format!("repeatCount=\"{t}\" ").as_str());
        self
    }

    ///
    /// # Set the provenance value
    ///
    /// - `from` The provenance value
    ///
    pub fn from(&mut self, from: &str) -> &mut Self {
        self.svg.push_str(format!("from=\"{from}\" ").as_str());
        self
    }

    ///
    /// # Close a group
    ///
    pub fn g_end(&mut self) -> &mut Self {
        self.svg.push_str("</g>");
        self
    }

    ///
    /// # Add a transformation
    ///
    /// - `transform` The transformation
    ///
    pub fn transform(&mut self, transform: &str) -> &mut Self {
        self.svg
            .push_str(format!("transform=\"{transform}\" ").as_str());
        self
    }

    ///
    /// # Define a width
    ///
    /// - `width` The width
    ///
    pub fn width(&mut self, width: f64) -> &mut Self {
        self.svg.push_str(format!("width=\"{width}\" ").as_str());
        self
    }

    ///
    /// # Set a fill color
    ///
    /// - `fill` The color
    ///
    pub fn fill(&mut self, fill: &str) -> &mut Self {
        self.svg.push_str(format!("fill=\"{fill}\" ").as_str());
        self
    }

    ///
    /// # Add a data
    ///
    /// - `d` The data
    ///
    pub fn d(&mut self, d: &str) -> &mut Self {
        self.svg.push_str(format!("d=\"{d}\" ").as_str());
        self
    }

    ///
    /// # Define  attribute defines a radius on the x-axis
    ///
    /// - `rx` radius on the x-axis
    ///
    pub fn rx(&mut self, rx: f64) -> &mut Self {
        self.svg.push_str(format!("rx=\"{rx}\" ").as_str());
        self
    }
    ///
    /// # Define  attribute defines a radius on the y-axis
    ///
    /// - `ry` radius on the y-axis
    ///
    pub fn ry(&mut self, ry: f64) -> &mut Self {
        self.svg.push_str(format!("ry=\"{ry}\" ").as_str());
        self
    }

    ///
    /// # Start a line
    ///
    /// - `x1`      The first x-coordinate
    /// - `y1`      The first y-coordinate
    /// - `x2`      The second x-coordinate
    /// - `y2`      The second y-coordinate
    /// - `stroke`  The line color
    ///
    pub fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, stroke: &str) -> &mut Self {
        self.svg.push_str(
            format!("<line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\" stroke=\"{stroke}\" ")
                .as_str(),
        );

        self
    }

    ///
    /// # define a rayon
    ///
    /// - `r`  The rayon
    ///
    pub fn r(&mut self, r: f64) -> &mut Self {
        self.svg.push_str(format!("r=\"{r}\" ").as_str());
        self
    }

    ///
    /// # Defines the x-axis coordinate for the center point of an element.
    ///
    /// - `cx` the x-axis coordinate
    ///
    pub fn cx(&mut self, cx: f64) -> &mut Self {
        self.svg.push_str(format!("cx=\"{cx}\" ").as_str());
        self
    }

    ///
    /// # defines the y-axis coordinate for the center point of an element.
    ///
    /// - `cy` the y-axis coordinate
    ///
    pub fn cy(&mut self, cy: f64) -> &mut Self {
        self.svg.push_str(format!("cy=\"{cy}\" ").as_str());
        self
    }

    ///
    /// # define a stroke
    ///
    /// - `stoke` the stoke color
    ///
    pub fn stroke(&mut self, stroke: &str) -> &mut Self {
        self.svg.push_str(format!("stroke=\"{stroke}\" ").as_str());
        self
    }

    ///
    /// # Define a stoke width
    ///
    /// - `width`  The stoke width
    ///
    pub fn stroke_width(&mut self, width: f64) -> &mut Self {
        self.svg
            .push_str(format!("stroke-width=\"{width}\" ").as_str());
        self
    }

    ///
    /// # Define a height
    ///
    /// - `height`  The height
    ///
    pub fn height(&mut self, height: f64) -> &mut Self {
        self.svg.push_str(format!("height=\"{height}\" ").as_str());
        self
    }

    ///
    /// # Define and id
    ///
    /// - `id` The id name
    ///
    pub fn id(&mut self, id: &str) -> &mut Self {
        self.svg.push_str(format!("id=\"{id}\" ").as_str());
        self
    }

    ///
    /// # Pos in x
    ///
    /// - `x` The position in x
    ///
    pub fn x(&mut self, x: f64) -> &mut Self {
        self.svg.push_str(format!("x=\"{x}\" ").as_str());
        self
    }

    ///
    /// # Pos in y
    ///
    /// - `y`   The position in y
    ///
    pub fn y(&mut self, y: f64) -> &mut Self {
        self.svg.push_str(format!("y=\"{y}\" ").as_str());
        self
    }

    ///
    /// # Configure a href
    ///
    /// - `href` the href
    ///
    pub fn href(&mut self, href: &str) -> &mut Self {
        self.svg
            .push_str(format!("xlink:href=\"#{href}\" ").as_str());
        self
    }

    ///
    /// # Start a use
    ///
    pub fn u(&mut self) -> &mut Self {
        self.svg.push_str("<use ");
        self
    }

    ///
    /// # Start a path
    ///
    pub fn path(&mut self) -> &mut Self {
        self.svg.push_str("<path ");
        self
    }

    ///
    /// # Add a style
    ///
    /// - `style`   The style to add
    ///
    pub fn style(&mut self, style: &str) -> &mut Self {
        self.svg.push_str(format!("style=\"{style}\" ").as_str());
        self
    }

    ///
    /// # Start a rectangle
    ///
    pub fn rect(&mut self) -> &mut Self {
        self.svg.push_str("<rect  ");
        self
    }

    ///
    /// # Close the svg
    ///
    pub fn end(&mut self) -> &mut Self {
        self.svg.push_str("</svg>");
        self
    }

    ///
    /// # Write the svg to disk
    ///
    /// # Panics
    ///
    /// On svg creation failure
    ///
    /// # Errors
    ///
    /// On dir creation failure
    ///
    /// - `dir`         The directory to save the svg
    /// - `filename`    The filename without the extension
    ///
    pub fn save(&mut self, dir: &str, filename: &str) -> std::io::Result<()> {
        if !Path::new(dir).exists() {
            fs::create_dir(dir).expect("failed to create the directory");
        }
        let mut f = fs::File::create(format!("{dir}/{filename}.svg").as_str())
            .expect("failed to create the file");
        f.write_all(self.svg.as_bytes()).expect("");
        f.sync_data()
    }
}
