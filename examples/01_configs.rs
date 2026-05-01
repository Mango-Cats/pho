use pho::{
    algorithms::{
        Aline,
        editex::config::Editex,
        jaro_winkler::config::JaroWinkler,
        levenshtein::config::{Costs, Levenshtein},
    },
    config_io::{export, import},
};

fn main() {
    println!("# Example 1: configuration workflow");

    // # Constructing Configs
    //
    // ## Manually Constructing Configs
    // Most configs do not have a constructor (i.e., `::new()`). So, we
    // create them directly.
    let config = Levenshtein::new(Costs::new(1., 2., 3.), true);

    println!("Constructed config: {:?}", config);

    // ## Reading Configs from TOML files
    // > These mirror the sample configs in tests/.
    //
    // We simply import `pho::config_io` and use the `import()`
    // function.
    //
    // Notice that the type of the config here is **required** since
    // the parsing logic is implied from the type. So, if the type is
    // `Editex` the `import()` function will read it as if it's
    // an config file for an Editex algorithm.
    //
    // Not specifying the type will lead to an error!
    let levenshtein = import::<Levenshtein>("tests/config_sample_levenshtein.toml").unwrap();
    let jaro_winkler = import::<JaroWinkler>("tests/config_sample_jaro_winkler.toml").unwrap();
    let editex = import::<Editex>("tests/config_sample_editex.toml").unwrap();
    let aline = import::<Aline>("tests/config_sample_aline.toml").unwrap();

    // # Validating Configs
    //
    // These checks are optional but recommended for user-provided files.
    // We simply call validate on the config itself.
    jaro_winkler.validate().unwrap();
    aline.validate().unwrap();

    println!("Levenshtein config: {:?}", levenshtein);
    println!("Jaro-Winkler config: {:?}", jaro_winkler);
    println!("Editex config: {:?}", editex);
    println!("ALINE config: {:?}", aline);

    // # Exporting Configs
    //
    // Suppose you defined a config in code, consider `my_conf` below.
    // You may want to save this as a TOML file so that they can share
    // it to somebody else. Calling `export` from `pho::config_io`
    // allows you to export it as a TOML file.
    //
    // For this example we will just re-export the `aline` config
    // read from the sample data (you can compare the sample data
    // and the exported one!).
    //
    // Similar to the `import()`, `export()` infers how to print it from
    // the type of the config.
    let my_conf = aline;
    export("my_conf", &my_conf, true).unwrap();
}
