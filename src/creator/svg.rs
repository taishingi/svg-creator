use std::fs;
use std::io::Write;
use std::path::Path;

pub struct Svg {
    svg: String,
    view_box: String,
    width: i32,
    height: i32,
}

impl Svg {
    ///
    /// # Constructor
    ///
    /// - `width`       The svg width
    /// - `height`      The svg height
    /// - `view_box`    The svg viewBox
    ///
    pub fn new(width: i32, height: i32, view_box: Option<String>) -> Self {
        match view_box {
            None => Self {
                svg: String::new(),
                view_box: String::new(),
                width,
                height,
            },
            Some(x) => Self {
                svg: String::new(),
                view_box: x,
                width,
                height,
            },
        }
    }

    ///
    /// # Start the svg
    ///
    pub fn start(&mut self) -> &mut Self {
        if self.view_box.is_empty() {
            self.svg.push_str(format!("<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" width=\"{}\" height=\"{}\" role=\"img\">", self.width, self.height).as_str());
        } else {
            self.svg.push_str(format!("<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" width=\"{}\" height=\"{}\" viewBox=\"{}\" role=\"img\">", self.width, self.height,self.view_box).as_str());
        }
        self
    }
    ///
    /// # Create a rectangle
    ///
    /// - `x`
    /// - `y`
    /// - `width`
    /// - `height`
    /// - `fill`
    /// - `stoke`
    /// - `stroke_width`
    ///
    pub fn rect(
        &mut self,
        width: i32,
        height: i32,
        fill: &str,
        x: Option<i32>,
        y: Option<i32>,
        stroke: Option<String>,
        stroke_width: Option<i32>,
        style: Option<String>,
        id: Option<String>,
        rx: Option<i32>,
    ) -> &mut Self {
        self.render_rectangle(width,height,fill,id,x,y,stroke,stroke_width,style,rx)
    }

    ///
    /// # Create a circle
    ///
    /// - `cx`
    /// - `cy`
    /// - `r`
    /// - `stoke`
    /// - `stroke_width`
    /// - `fill`
    ///
    pub fn circle(
        &mut self,
        cx: i32,
        cy: i32,
        r: i32,
        stroke: &str,
        stroke_width: i32,
        fill: &str,
    ) -> &mut Self {
        self.svg.push_str(format!("<circle cx=\"{cx}\" cy=\"{cy}\" r=\"{r}\" stroke=\"{stroke}\" stroke-width=\"{stroke_width}\" fill=\"{fill}\" />").as_str());
        self
    }

    pub fn gopen(&mut self, transform: Option<&str>) -> &mut Self {
        match transform {
            None => {
                self.svg.push_str("<g>");
            }
            Some(x) => {
                self.svg.push_str(format!("<g transform=\"{x}\">").as_str());
            }
        }
        self
    }

    pub fn gclose(&mut self) -> &mut Self {
        self.svg.push_str("</g>");
        self
    }

    pub fn uses(&mut self, id: &str, transform: &str,href: Option<String>) -> &mut Self {
        if href.is_some() {
            self.svg
                .push_str(format!("<use xlink:href=\"#{id}\" transform=\"{transform}\" xlink:href=\"#{}\" /> ",href.unwrap().as_str()).as_str());
        }
        self.svg
            .push_str(format!("<use xlink:href=\"#{id}\" transform=\"{transform}\"/>").as_str());
        self
    }

    pub fn path(&mut self, fill: &str, d: &str,stroke:Option<String>,stroke_width:Option<f64>,id: Option<String>) -> &mut Self {
        self.render_path(fill,d,stroke,stroke_width,id)
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


    fn render_rectangle(
        &mut self,
        width: i32,
        height: i32,
        fill: &str,
        id: Option<String>,
        x: Option<i32>,
        y: Option<i32>,
        stroke: Option<String>,
        stroke_width: Option<i32>,
        style: Option<String>,
        rx: Option<i32>,

    ) -> &mut Self {
        let mut  rec = String::from(format!("<rect width=\"{width}\" height=\"{height}\" fill=\"{fill}\" ").as_str());

        if id.is_some()  {
            rec.push_str(format!("id=\"{}\" ",id.unwrap()).as_str());
        }
        if x.is_some() {
            rec.push_str(format!("x=\"{}\" ",x.unwrap()).as_str());
        }
        if y.is_some() {
            rec.push_str(format!("y=\"{}\" ",y.unwrap()).as_str());
        }
        if stroke.is_some() {
            rec.push_str(format!("stroke=\"{}\" ",stroke.unwrap()).as_str());
        }
        if stroke_width.is_some() {
            rec.push_str(format!("stroke-width=\"{}\" ",stroke_width.unwrap()).as_str());
        }
        if style.is_some() {
            rec.push_str(format!("style=\"{}\" ",style.unwrap()).as_str());
        }
        if rx.is_some() {
            rec.push_str(format!("rx=\"{}%\" ",rx.unwrap()).as_str());
        }
        rec.push_str("/>");
        self.svg.push_str(rec.as_str());
        self
    }


    fn render_path(&mut self, fill: &str, d: &str,stroke:Option<String>,stroke_width:Option<f64>,id: Option<String>) -> &mut Self {
        let mut  path = String::from(format!("<path fill=\"{fill}\" d=\"{d}\" ").as_str());

        if stroke.is_some() {
            path.push_str(format!("stroke=\"{}\" ",stroke.unwrap()).as_str());
        }
        if stroke_width.is_some() {
            path.push_str(format!("stroke-width=\"{}\" ",stroke_width.unwrap()).as_str());
        }
        if id.is_some()  {
            path.push_str(format!("id=\"{}\" ",id.unwrap()).as_str());
        }
        path.push_str("/>");
        self.svg.push_str(path.as_str());
        self
    }

}
