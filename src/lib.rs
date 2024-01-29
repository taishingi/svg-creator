pub mod creator {
    pub mod svg;
}

#[cfg(test)]
mod tests {
    use crate::creator::svg::Svg;

    #[test]
    pub fn all() {
        let f = "flags";
        let d = "distros";
        assert!(Svg::new(900, 600, String::from("0 0 900 600"),String::from(
            "japan"
        ))
            .start()
            .rect(900,600,"white",None,None,None,None,None,None,None)
            .circle(450, 300, 180, "none", 0, "#bc002d")
            .end()
            .save(f, "japan")
            .is_ok());

        assert!(Svg::new(900, 600, String::from("0 0 900 600"),String::from("china"))
            .start()
            .path("#EE1C25", "M0 0h900v600H0",None,None,None)
            .gopen(Some("translate(150,150) scale(3)"))
            .path(
                "#FF0",
                "M0,-30 17.63355,24.27051 -28.53171,-9.27051H28.53171L-17.63355,24.27051",
                None,None,
                Some("s".to_string()),
            )
            .gclose()
            .uses("s", "translate(300,60) rotate(23.036243)",None)
            .uses("s", "translate(360,120) rotate(45.869898)",None)
            .uses("s", "translate(360,210) rotate(69.945396)",None)
            .uses("s", "translate(300,270) rotate(20.659808)",None)
            .end()
            .save(f, "china")
            .is_ok());

        assert!(Svg::new(900, 600, String::from("0 0 900 600"),String::from("france"))
            .start()
            .rect(300,600,"#fff",Some(300),None,None,None,None,None,None)
            .rect(300,600,"#ed2939",Some(600),None,None,None,None,None,None)
            .rect(300,600,"#002395",None,None,None,None,None,None,None)
            .end()
            .save(f, "france")
            .is_ok());

        assert!(Svg::new(900, 600, String::from("0 0 900 600"),String::from("italy"))
            .start()
            .rect(300,600,"#F1F2F1",Some(300),None,None,None,None,None,None)
            .rect(300,600,"#ce2b37",Some(600),None,None,None,None,None,None)
            .rect(300,600,"#009246",None,None,None,None,None,None,None)
            .end()
            .save(f, "italy")
            .is_ok());

        assert!(Svg::new(900, 600, String::from("0 0 900 600"),String::from("algeria"))
            .start()
            .path("#fff","M0 0h900v600H0z",None,None,None)
            .path("#063","M0 0h450v600H0z",None,None,None)
            .path("#d21034","M579.903811 225a150 150 0 1 0 0 150 120 120 0 1 1 0-150M585.676275 300 450 255.916106 533.852549 371.329239v-142.658277L450 344.083894z",None,None,None)
            .end()
            .save(f, "algeria")
            .is_ok());

        assert!(Svg::new(900, 600, String::from("0 0 90000 60000"),String::from("morocco"))
            .start()
            .path("#c1272d","m0 0h90000v60000H0z",None,None,None)
            .path("none","m45000 17308 7460 22960-19531-14190h24142L37540 40268z",Some("#006233".to_string()),Some(1426.0), None)
            .end()
            .save(f, "morocco")
            .is_ok());

        assert!(Svg::new(900, 600, String::from("0 0 27 18"),String::from("greece"))
            .start()
            .rect(27,18,"#0D5EAF",None,None,None,None,None,None,None)
            .path("none","M5,0V11 M0,5H10 M10,3H27 M10,7H27 M0,11H27 M0,15H27",Some("#FFF".to_string()),Some(2.0),None)
            .end()
            .save(f, "greece")
            .is_ok());

        assert!(Svg::new(512, 512, String::from("0 0 512 512"),String::from("arch"))
            .start()
            .rect(512,512,"#fff",None,None,None,None,None,None,Some(15))
            .path("none","M5,0V11 M0,5H10 M10,3H27 M10,7H27 M0,11H27 M0,15H27",Some("#FFF".to_string()),Some(2.0),None)
            .path("#1793d1","M256 72c-14 35-23 57-39 91 10 11 22 23 41 36-21-8-35-17-45-26-21 43-53 103-117 220 50-30 90-48 127-55-2-7-3-14-3-22v-1c1-33 18-58 38-56 20 1 36 29 35 62l-2 17c36 7 75 26 125 54l-27-50c-13-10-27-23-55-38 19 5 33 11 44 17-86-159-93-180-122-250z",None,None,None)
            .end()
            .save(d, "arch")
            .is_ok());

        assert!(Svg::new(267, 267, String::from("0 0 267 267"),String::from("fedora"))
            .start()
            .path("#294172","M 266.62575,133.50613 C 266.62575,59.98128 207.02222,0.37583 133.49792,0.37583 C 60.00668,0.37583 0.42639,59.93123 0.37425,133.41225 L 0.37425,236.4333 C 0.4138,253.11763 13.94545,266.62417 30.64027,266.62417 L 133.55192,266.62417 C 207.05167,266.59532 266.62575,207.01142 266.62575,133.50613",None,None,Some(String::from("voice")))
            .path("none","M 77.126289,142.09756 C 77.126289,142.09756 124.97104,142.09756 124.97104,142.09756 C 124.97104,142.09756 124.97104,189.94234 124.97104,189.94234 C 124.97104,216.35263 103.53659,237.78707 77.126289,237.78707 C 50.715979,237.78707 29.28153,216.35263 29.28153,189.94234 C 29.28153,163.53203 50.715979,142.09756 77.126289,142.09756 z",Some(String::from("#3c6eb4")),Some(29.21),Some(String::from("in")))
            .uses("in","matrix(-1,0,0,-1,249.71151,284.2882)",Some(String::from("finity")))
            .path("#ffffff","M 139.6074,127.52923 L 139.6074,189.87541 C 139.6074,224.37943 111.63203,252.35541 77.12679,252.35541 C 71.89185,252.35541 68.1703,251.7644 63.32444,250.49771 C 56.25849,248.64859 50.48398,242.85518 50.48158,236.1166 C 50.48158,227.97147 56.39394,222.0467 65.23187,222.0467 C 69.43824,222.0467 70.96454,222.85435 77.12679,222.85435 C 95.3184,222.85435 110.07443,208.11916 110.10634,189.92756 L 110.10634,161.27099 C 110.10634,158.70324 108.01971,156.62274 105.44767,156.62274 L 83.78246,156.61846 C 75.71034,156.61846 69.18845,150.18003 69.18845,142.0858 C 69.18414,133.94124 75.77725,127.52923 83.93653,127.52923",None,None,Some(String::from("free")))
            .uses("dom","matrix(-1,0,0,-1,249.71152,284.28821)",Some(String::from("free")))
            .path("#3c6eb4","M 243.65456,243.58425 C 243.65456,243.58425 243.6546,238.05286 243.6546,238.05286 L 241.12607,243.85062 C 241.12607,243.85062 238.66466,238.05286 238.66466,238.05286 L 238.66513,243.58425 L 237.24683,243.58425 L 237.24683,234.84933 L 238.73387,234.84933 C 238.73387,234.84933 241.16784,240.42984 241.16784,240.42984 L 243.56495,234.84933 L 245.07039,234.84933 L 245.07039,243.58425 L 243.65456,243.58425 z M 233.32154,236.31241 L 233.32154,243.58425 L 231.83941,243.58425 L 231.83941,236.31241 L 229.35453,236.31241 L 229.35453,234.84933 L 235.80399,234.84933 L 235.80399,236.31241",None,None,Some(String::from("TM")))
            .end()
            .save(d, "fedora")
            .is_ok());
    }
}
