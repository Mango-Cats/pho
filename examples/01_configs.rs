use pho::{
    algorithms::{
        Aline,
        editex::config::Editex,
        jaro_winkler::config::JaroWinkler,
        levenshtein::config::{Costs, Levenshtein},
    },
    utils::io::{export, import},
};

fn main() {
    println!("🍜\t| # Example 1: configuration files");

    // Configs
    // ...
    //  Configuration files are the heart of the algorithms defined in
    //  pho. This allows users to define their own configurations for
    //  the algorithms, allowing the algorithm's implementation to be
    //  language-agnostic.

    // (Manually) Constructing Configs
    // ...
    //  Configs can be created by using the constructor `::new()`.
    //
    //  While some special algorithms use the constructor
    //  `::try_new()` since the values need to be evaluated.
    //  For instance, the Jaro-Winkler algorithm requires the
    //  prefix scale to be within the bounds of [0.0, 0.25].
    let config = Levenshtein::new(Costs::new(1., 2., 3.), true);

    println!("\t| Constructed config: {:?}", config);

    // Reading Configs from TOML files
    // ...
    //  To make configs human-readable, pho also supports importing
    //  configs (using `import()`) from a TOML file.
    //
    //  Notice that the type of the config here is **required** since
    //  the parsing logic is implied from the type. So, if the type is
    //  `Editex` the `import()` function will read it as if it's
    //  an config file for an Editex algorithm.
    //
    //  Not specifying the type will lead to an error!
    let levenshtein = import::<Levenshtein>("tests/config_sample_levenshtein.toml").unwrap();
    let jaro_winkler = import::<JaroWinkler>("tests/config_sample_jaro_winkler.toml").unwrap();
    let editex = import::<Editex>("tests/config_sample_editex.toml").unwrap();
    let aline = import::<Aline>("tests/config_sample_aline.toml").unwrap();

    // Validating Configs
    // ...
    //  Using `import()` simply parses and constructs the Config
    //  **without validation**. So, if the algorithm you're using
    //  requires the config to be validated. Call `validate()`
    //  immediately after importing.
    //
    //  In this case, both Jaro-Winkler and Aline require validation.
    //  The others dont.
    jaro_winkler.validate().unwrap();
    aline.validate().unwrap();

    println!("\t| Levenshtein config: {:?}", levenshtein);
    println!("\t| Jaro-Winkler config: {:?}", jaro_winkler);
    println!("\t| Editex config: {:?}", editex);
    println!("\t| ALINE config: {:?}", aline);

    // Exporting Configs
    // ...
    //  You may want to save configs as a TOML files so that they can
    //  be easily shared.
    //
    //  Calling `export` from `pho::io` allows you to export it
    //  as a TOML file.
    //
    //  For this example we will just re-export the `aline` config
    //  read from the sample data (you can compare the sample data
    //  and the exported one!).
    //
    //  Similar to the `import()`, `export()` infers how to print it
    //  from the type of the config.
    let my_conf = aline;
    export("my_conf", &my_conf, true).unwrap();
}
