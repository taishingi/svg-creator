use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub struct Svg {
    svg: String,
    view_box: String,
    width: f64,
    height: f64,
    width_unit: String,
    height_unit: String,
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
    pub fn new(
        width: f64,
        width_unit: &str,
        height: f64,
        height_unit: &str,
        view: &str,
        id: &str,
    ) -> Self {
        Self {
            svg: String::new(),
            view_box: view.to_string(),
            width,
            height,
            width_unit: width_unit.to_string(),
            height_unit: height_unit.to_string(),
            id: id.to_string(),
        }
    }

    pub fn animate(&mut self) -> &mut Self {
        self.svg.push_str("<animate ");
        self
    }
    pub fn values(&mut self, value: &str) -> &mut Self {
        self.svg.push_str(format!("values=\"{value}\" ").as_str());
        self
    }
    pub fn filter(&mut self, f: &str) -> &mut Self {
        self.svg.push_str(format!("filter=\"{f}\" ").as_str());
        self
    }
    pub fn result(&mut self, f: &str) -> &mut Self {
        self.svg.push_str(format!("filter=\"{f}\" ").as_str());
        self
    }
    pub fn in1(&mut self, i: &str) -> &mut Self {
        self.svg.push_str(format!("in=\"{i}\" ").as_str());
        self
    }

    pub fn in2(&mut self, i: &str) -> &mut Self {
        self.svg.push_str(format!("in2=\"{i}\" ").as_str());
        self
    }

    pub fn fe_offset(&mut self) -> &mut Self {
        self.svg.push_str("<feOffset ");
        self
    }

    ///
    /// # Start the svg
    ///
    pub fn start(&mut self) -> &mut Self {
        self.svg.push_str(format!("<?xml version=\"1.0\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" width=\"{}{}\" height=\"{}{}\" viewBox=\"{}\" role=\"img\" id=\"{}\">", self.width,self.width_unit, self.height,self.height_unit,self.view_box,self.id).as_str());
        self
    }

    ///
    /// # Start a circle
    ///
    pub fn circle(&mut self) -> &mut Self {
        self.svg.push_str("<circle ");
        self
    }

    fn generate_badge(
        t: bool,
        img: &str,
        label: &str,
        s: &str,
        e: &str,
        dir: &str,
        f: &str,
    ) -> i32 {
        let mut svg = Self::new(164.0, "px", 28.0, "px", "0 0 164 28", label);
        let mut right_bg = String::new();
        let mut left_bg = String::new();
        let mut text = String::new();
        if t {
            right_bg.push_str("#00ff00");
            left_bg.push_str("#222");
            text.push_str(s);
        } else {
            right_bg.push_str("#ff0000");
            left_bg.push_str("#222");
            text.push_str(e);
        }
        svg.start()
            .g()
            .shape_rendering("crispEdges")
            .close_tag()
            .rect()
            .width(82.05, "px")
            .height(28.0, "px")
            .fill(left_bg.as_str())
            .close_tag()
            .close_rect()
            .image(img)
            .x(9.0, "px")
            .y(7.0, "px")
            .width(14.0, "px")
            .height(14.0, "px")
            .close()
            .close_group()
            .group()
            .rect()
            .x(82.05, "px")
            .width(82.1716, "px")
            .height(100.0, "px")
            .fill(right_bg.as_str())
            .close_tag()
            .close_rect()
            .text_anchor("middle")
            .text_rendering("geometricPrecision")
            .font_size(100.0, "px")
            .close_tag()
            .text()
            .x(495.26276, "px")
            .y(175.0, "px")
            .transform("scale(0.1)")
            .fill("#fff")
            .text_length(410.5255, "px")
            .close_tag()
            .content(label)
            .close_text()
            .text()
            .x(1231.3833, "px")
            .y(175.0, "px")
            .transform("scale(0.1)")
            .font_weight("bold")
            .fill("#fff")
            .text_length(581.71564, "px")
            .close_tag()
            .content(text.as_str())
            .close_text()
            .close_group()
            .end()
            .save(dir, f)
    }

    #[must_use]
    pub fn badge(
        t: bool,
        l: &str,
        e: &str,
        s: &str,
        url: &str,
        output_dir: &str,
        filename: &str,
    ) -> i32 {
        Self::generate_badge(t, url, l, s, e, output_dir, filename)
    }

    fn text_length(&mut self, length: f64, unit: &str) -> &mut Self {
        self.svg
            .push_str(format!("textLength=\"{length}{unit}\"").as_str());
        self
    }
    fn font_weight(&mut self, weight: &str) -> &mut Self {
        self.svg
            .push_str(format!("font-weight=\"{weight}\" ").as_str());
        self
    }
    fn text_rendering(&mut self, rendering: &str) -> &mut Self {
        self.svg
            .push_str(format!("text-rendering=\"{rendering}\" ").as_str());
        self
    }

    fn shape_rendering(&mut self, rendering: &str) -> &mut Self {
        self.svg
            .push_str(format!("shape-rendering=\"{rendering}\" ").as_str());
        self
    }

    ///
    /// # Start an image
    ///
    /// # Panics
    ///
    /// if base64 not work and wget not found
    ///
    pub fn image(&mut self, uri: &str) -> &mut Self {
        let img = uri.split('/').last().expect("");
        let base = File::create("b.txt").expect("");
        if Path::new(format!("./{img}").as_str()).is_file() {
            fs::remove_file(format!("./{img}").as_str()).expect("");
        }
        assert!(Command::new("wget")
            .arg("-q")
            .arg(uri)
            .current_dir(".")
            .spawn()
            .expect("")
            .wait()
            .expect("")
            .success());
        assert!(Command::new("base64")
            .arg(img)
            .stdout(base)
            .current_dir(".")
            .spawn()
            .expect("")
            .wait()
            .expect("")
            .success());
        self.svg.push_str(
            format!(
                "<image xlink:href=\"data:image/svg+xml;base64,{}\" ",
                fs::read_to_string("b.txt").expect("a")
            )
            .as_str(),
        );
        if Path::new(format!("./{img}").as_str()).is_file() {
            fs::remove_file(format!("./{img}").as_str()).expect("");
        }
        self
    }

    ///
    /// # Close a circle
    ///
    pub fn close_circle(&mut self) -> &mut Self {
        self.svg.push_str("</circle> ");
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
    /// # Create a group
    ///
    pub fn group(&mut self) -> &mut Self {
        self.svg.push_str("<g>");
        self
    }

    ///
    /// # Add a target to a link
    ///
    /// - `target` The target
    ///
    pub fn target(&mut self, target: &str) -> &mut Self {
        self.svg.push_str(format!("target=\"{target}\" ").as_str());
        self
    }

    ///
    /// # Add a font family
    ///
    /// - `family` The font
    ///
    pub fn font_family(&mut self, family: &str) -> &mut Self {
        self.svg
            .push_str(format!("font-family=\"{family}\" ").as_str());
        self
    }

    ///
    /// # Add a font size
    ///
    /// - `size` The font size
    /// - `unit` The font unity
    ///
    pub fn font_size(&mut self, size: f64, unit: &str) -> &mut Self {
        self.svg
            .push_str(format!("font-size=\"{size}{unit}\" ").as_str());
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
    pub fn close_a(&mut self) -> &mut Self {
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
    /// # Start an ellipse
    ///
    pub fn ellipse(&mut self) -> &mut Self {
        self.svg.push_str("<ellipse ");
        self
    }

    ///
    /// # Close a text
    ///
    pub fn close_text(&mut self) -> &mut Self {
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
    /// # Add a stroke miterlimit
    ///
    /// - `limit` The limit
    ///
    pub fn stroke_miterlimit(&mut self, limit: f64) -> &mut Self {
        self.svg
            .push_str(format!("stroke-miterlimit=\"{limit}\" ").as_str());
        self
    }

    ///
    /// # Add a pointer events
    ///
    /// - `event` The event
    ///
    pub fn pointer_events(&mut self, event: &str) -> &mut Self {
        self.svg
            .push_str(format!("pointer-events=\"{event}\" ").as_str());
        self
    }

    ///
    /// # Add a system language events
    ///
    /// - `lang` The lang event
    ///
    pub fn system_language(&mut self, lang: &str) -> &mut Self {
        self.svg
            .push_str(format!("systemLanguage=\"{lang}\" ").as_str());
        self
    }

    pub fn div(&mut self) -> &mut Self {
        self.svg
            .push_str("<div xmlns=\"http://www.w3.org/1999/xhtml\"  ");
        self
    }

    pub fn close_div(&mut self) -> &mut Self {
        self.svg.push_str("</div>");
        self
    }

    pub fn close_foreign_object(&mut self) -> &mut Self {
        self.svg.push_str("</foreignObject>");
        self
    }

    pub fn foreign_object(&mut self) -> &mut Self {
        self.svg.push_str("<foreignObject ");
        self
    }

    ///
    /// # Add a class
    ///
    /// - `css` The class name
    ///
    pub fn css(&mut self, css: &str) -> &mut Self {
        self.svg
            .push_str(format!("<style text=\"text/css\">{css}</style>").as_str());
        self
    }

    ///
    /// # Add a type
    ///
    /// - `t` The type name
    ///
    pub fn t(&mut self, t: &str) -> &mut Self {
        self.svg.push_str(format!("type=\"{t} ").as_str());
        self
    }

    ///
    /// # Close a tag
    ///
    pub fn close_tag(&mut self) -> &mut Self {
        self.svg.push_str("> ");
        self
    }

    ///
    /// # Start a tspan
    ///
    pub fn tspan(&mut self) -> &mut Self {
        self.svg.push_str("<tspan ");
        self
    }

    ///
    /// # Close a tspan
    ///
    pub fn close_tspan(&mut self) -> &mut Self {
        self.svg.push_str("</tspan> ");
        self
    }

    ///
    /// # Create a feMorphology
    ///
    pub fn fe_morphology(&mut self) -> &mut Self {
        self.svg.push_str("<feMorphology ");
        self
    }

    ///
    /// # Create a feConvolveMatrix
    ///
    pub fn fe_convolve_matrix(&mut self) -> &mut Self {
        self.svg.push_str("<feConvolveMatrix ");
        self
    }

    ///
    /// # Create a feBlend
    ///
    pub fn fe_blend(&mut self) -> &mut Self {
        self.svg.push_str("<feBlend ");
        self
    }

    ///
    /// # Create a feColorMatrix
    ///
    pub fn fe_color_matrix(&mut self) -> &mut Self {
        self.svg.push_str("<feColorMatrix ");
        self
    }

    ///
    /// # Start a span
    ///
    pub fn span(&mut self) -> &mut Self {
        self.svg.push_str("<span ");
        self
    }

    ///
    /// # Start a span
    ///
    pub fn b(&mut self, text: &str) -> &mut Self {
        self.svg.push_str(format!("<b>{text}</b>").as_str());
        self
    }

    ///
    /// # Close a span
    ///
    pub fn close_span(&mut self) -> &mut Self {
        self.svg.push_str("</span> ");
        self
    }

    ///
    /// # Create a switch
    ///
    pub fn switch(&mut self) -> &mut Self {
        self.svg.push_str("<switch> ");
        self
    }

    ///
    /// # Close a switch
    ///
    pub fn close_switch(&mut self) -> &mut Self {
        self.svg.push_str("</switch> ");
        self
    }

    ///
    /// # Close a rect
    ///
    pub fn close_rect(&mut self) -> &mut Self {
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
    /// # Set a maximum duration
    ///
    /// - `duration`    The max duration
    ///
    pub fn max(&mut self, duration: &str) -> &mut Self {
        self.svg.push_str(format!("max=\"{duration}\" ").as_str());
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
    /// # The maximum value
    ///
    /// - `to` The max value
    ///
    pub fn key_times(&mut self, key: &str) -> &mut Self {
        self.svg.push_str(format!("keyTimes=\"{key}\" ").as_str());
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
    /// # Set the text-anchor position
    ///
    /// - `align` The position
    ///
    pub fn alignment_baseline(&mut self, align: &str) -> &mut Self {
        self.svg
            .push_str(format!("alignment-baseline=\"{align}\" ").as_str());
        self
    }

    ///
    /// # Set the text-anchor position
    ///
    /// - `d` The new position
    ///
    pub fn dx(&mut self, d: &str) -> &mut Self {
        self.svg.push_str(format!("dx=\"{d}\" ").as_str());
        self
    }

    ///
    /// # Set d on y-axis
    ///
    /// - `d` The new position
    ///
    pub fn dy(&mut self, d: &str) -> &mut Self {
        self.svg.push_str(format!("dy=\"{d}\" ").as_str());
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
    pub fn close_group(&mut self) -> &mut Self {
        self.svg.push_str("</g> ");
        self
    }

    pub fn defs(&mut self) -> &mut Self {
        self.svg.push_str("<defs> ");
        self
    }

    pub fn close_deps(&mut self) -> &mut Self {
        self.svg.push_str("</defs> ");
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
    /// # Set a path length
    ///
    /// - `l` The path length
    ///
    pub fn path_length(&mut self, l: &str) -> &mut Self {
        self.svg.push_str(format!("pathLength=\"{l}\" ").as_str());
        self
    }

    ///
    /// # Define a width
    ///
    /// - `width`   The width
    /// - `unit`    The width unit
    ///
    pub fn width(&mut self, width: f64, unit: &str) -> &mut Self {
        self.svg
            .push_str(format!("width=\"{width}{unit}\" ").as_str());
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
    /// - `rx`   radius on the x-axis
    /// - `unit` radius on the x-axis unit
    ///
    pub fn rx(&mut self, rx: f64, unit: &str) -> &mut Self {
        self.svg.push_str(format!("rx=\"{rx}{unit}\" ").as_str());
        self
    }
    ///
    /// # Define  attribute defines a radius on the y-axis
    ///
    /// - `ry` radius on the y-axis
    /// - `unit` radius on the y-axis unit
    ///
    pub fn ry(&mut self, ry: f64, unit: &str) -> &mut Self {
        self.svg.push_str(format!("ry=\"{ry}{unit}\" ").as_str());
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
    /// - `r`       The rayon
    /// - `unit`    The rayon unit
    ///
    pub fn r(&mut self, r: f64, unit: &str) -> &mut Self {
        self.svg.push_str(format!("r=\"{r}{unit}\" ").as_str());
        self
    }

    ///
    /// # Defines the x-axis coordinate for the center point of an element.
    ///
    /// - `cx`      The x-axis coordinate
    /// - `unit`    The x-axis coordinate unit
    ///
    pub fn cx(&mut self, cx: f64, unit: &str) -> &mut Self {
        self.svg.push_str(format!("cx=\"{cx}{unit}\" ").as_str());
        self
    }

    ///
    /// # defines the y-axis coordinate for the center point of an element.
    ///
    /// - `cy`      The y-axis coordinate
    /// - `unit`    The y-axis coordinate unit
    ///
    pub fn cy(&mut self, cy: f64, unit: &str) -> &mut Self {
        self.svg.push_str(format!("cy=\"{cy}{unit}\" ").as_str());
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
    /// - `width`   The stoke width
    /// - `unit`    The stoke width unit
    ///
    pub fn stroke_width(&mut self, width: f64, unit: &str) -> &mut Self {
        self.svg
            .push_str(format!("stroke-width=\"{width}{unit}\" ").as_str());
        self
    }

    ///
    /// # Define a height
    ///
    /// - `height`  The height
    /// - `unit`    The height unit
    ///
    pub fn height(&mut self, height: f64, unit: &str) -> &mut Self {
        self.svg
            .push_str(format!("height=\"{height}{unit}\" ").as_str());
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
    /// - `x`       The position in x
    /// - `unit`    The position unit
    ///
    pub fn x(&mut self, x: f64, unit: &str) -> &mut Self {
        self.svg.push_str(format!("x=\"{x}{unit}\" ").as_str());
        self
    }

    ///
    /// # Pos in y
    ///
    /// - `y`       The position in y
    /// - `unit`    The position unit
    ///
    pub fn y(&mut self, y: f64, unit: &str) -> &mut Self {
        self.svg.push_str(format!("y=\"{y}{unit}\" ").as_str());
        self
    }

    ///
    /// # Configure a href
    ///
    /// - `href` the href
    ///
    pub fn href(&mut self, href: &str) -> &mut Self {
        self.svg.push_str(format!("href=\"{href}\" ").as_str());
        self
    }

    ///
    /// # Configure a href
    ///
    /// - `href` the href
    ///
    pub fn link(&mut self, href: &str) -> &mut Self {
        self.svg
            .push_str(format!("xlink:href=\"{href}\" ").as_str());
        self
    }

    ///
    /// # Create a title
    ///
    /// - `title` the title
    ///
    pub fn title(&mut self, title: &str) -> &mut Self {
        self.svg
            .push_str(format!("<title>{title}</title>").as_str());
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
    /// # Return
    ///
    /// 0 On success
    /// 1 on lint failure
    ///
    pub fn save(&mut self, dir: &str, filename: &str) -> i32 {
        if !Path::new(dir).exists() {
            fs::create_dir(dir).expect("failed to create the directory");
        }
        let file = format!("{dir}/{filename}.svg");
        let mut f = File::create(file.as_str()).expect("failed to create the file");
        f.write_all(self.svg.as_bytes()).expect("");
        assert!(f.sync_data().is_ok());
        if !Command::new("xmllint")
            .arg("--pedantic")
            .arg(file.as_str())
            .stderr(File::create(format!("{filename}_check")).expect("failed to create the file"))
            .output()
            .unwrap()
            .status
            .success()
        {
            return 1;
        }
        0
    }
}
