use factorio_blueprint::{blueprint_string_to_model, model_to_blueprint_string, blueprint_string_to_pretty_json};
use clap::{App, Arg};

const _SAMPLE_BLUEPRINT: &str = "0eNrtWMuumzAQ/RcvK7jCBsJD6k9000UVIQJzE0tgI2OiRhH/3jG0uXkQCkRtIrWbRH6dGc85niNxJJuigUpxoUl8JDyToibxtyOp+VakhZnThwpITLiGklhEpKUZ5ZDxHJSdyXLDRaqlIq1FuMjhO4lpu7YICM01hx6tGxwS0ZQbULhhDMcilazxqBQmOsLZzLPIAf9d9uZjlJwryPp1ZhHMWCtZJBvYpXuO5/HQT9QE1/IOqTaz71zVOrm52J4r3eDMKad+h+2YG5l66NQUxzGDskpVl2RMPuMB2eiqmQG5VQCih60OmF0jdPKuZJlwgTgk1qqBto8q+it2iVPz0589KybPu0JmXGUN192QtdbFsne5TK+WV9en1xibmWAK8utQ7BoLOZ6SE25s27PAvzTAZmqAPUMDXy41QJ+sgQFabhRwjxZ3lF33DkvuKf0Sct6UNhSYlOKZXckChnhy3vwLpibeg40rObx7LzZL4/QW1xRbAN/uNrJRpl2F64FCeKdCpIrrXQkaazCta9GJiv0Afky0J4XVYHCSD/3aqBZZAcq3y4N8Wq7fR9oUHZXialr/8Bd6iPPfQy7r7Y+/vGj8AVFnjokEU03EnyaC1UIToX9TBF9f3kS8BSbyiPcHC2lznuj97PVo86fSFo4+yvAOS+ED3u/M8f5gvAO5d+8VjPcm9ze9ig6Yvztk/tFi82fRP2/+/hzzH6RogBGzb0kLmczHn+j8L9hCogUtZLphI1j32SA++8pgkT2oui99SL0gYoEX+V7gu237A6pardM=";
const _SAMPLE_BLUEPRINT_DECIDER: &str = "0eNq1l91KwzAUx9/lgDfa6dLvljkQvFbwShAt/YgusKYlTcUy+gC+hRf6Yj6JSSeoo93kQK5Kkp7/+f1SaMgGsnVLa8G4hHgDLK94A/HdBhr2xNO1npNdTSEGJmkJFvC01KNUMLkqqWT5LK/KjPFUVgJ6Cxgv6AvEpLcOZhQ0ZwUV4wF2f28B5ZJJRrdEw6BLeFtmVKgO+3IsqKtGlVZcd1dxMxJa0KmnMz/1VJeCCZpv120LlLUU1TrJ6Cp9ZqpeFX2nJmqtGJIaPav3R6Z6s+a6rKxTMTSMYQF6ou5UQctl8iiqMmG8btWrUrS07/WO7DjYSAdiyGGJcHBwDnZoyOHz9QNh4SItIkMW5wgHD+ngG/sSbwgLH2kRGLN4R1gEB/6Ue0Tcf4r8BO+4NFSPkz9KVU2V0BAHxzCKHOKRbdPIJ+PIER7ZMY18No5M5njmuWnmowlmgmcmpplnE8w2mpmEppkXiwloBw8dmYZ+mGB28cy+aeaLq8sJag9PHZimXi4noH08tPFT5fb6ZoIafxYSzzT1FlpdNYarSfzrNmTBMxXNtmlI3CCyAzfy3MBz+v4LCKeZFQ==";
const _SAMPLE_BLUEPRINT_XX: &str = "0eNqVkdtKxDAQht9lrrNCd1u6G/BCX8ILkZC2Ux1ok5BDsZS8u5NWRBBEr8Ic/m/mn2zQTQmdJxNBbkC9NQHk8waBXo2eSi6uDkECRZxBgNFziQbsaUB/6u3ckdHResgCyAz4DrLKLwLQRIqEB20PVmXS3KHnht84ApwNLLWmTGfcqboKWPm91HcNTxnIY3/UzwJ44+jtpDp80wuxnkWfVMW1YSeFkh3Jh6h+GFvIx8SZr52OjtNDcRSwMP4ueiwiNuO0381IuOcem6JL/xj9dFDcyg6SiWr0dlZkmAFy1FPAnMuJ9y+R335QwII+HJe5VnV7O7f1ranb5pLzB5+lp6U=";

enum Command {
    Decode(String),
    ReEncode(String),
}

fn parse_arguments() -> Option<Command> {
    let matches = App::new("JP Factorio Blueprint CLI")
        .version("0.1.0")
        .about("Factorio Blueprint Toolbox")
        .subcommand(App::new("reencode")
            .about("Decodes a blueprint string and then reencodes it")
            .arg(Arg::new("blueprint-string")
                .help("Factorio blueprint string as exported from the game")
                .required(true)
                .index(1)))
        .subcommand(App::new("decode")
            .about("Decodes a blueprint string and outputs the json")
            .arg(Arg::new("blueprint-string")
                .help("Factorio blueprint string as exported from the game")
                .required(true)
                .index(1)))
        .get_matches();

    match matches.subcommand() {
        Some(("decode", args)) => args
            .value_of("blueprint-string")
            .map(|b| Command::Decode(b.into())),
        Some(("reencode", args)) => args
            .value_of("blueprint-string")
            .map(|b| Command::ReEncode(b.into())),
        _ => None,
    }
}

fn reencode(blueprint: &str) -> factorio_blueprint::Result<()> {
    let model = blueprint_string_to_model(blueprint)?;
    let reencoded = model_to_blueprint_string(model)?;

    println!("{}", reencoded);

    Ok(())
}

fn decode(blueprint: &str) -> factorio_blueprint::Result<()> {
    let json = blueprint_string_to_pretty_json(blueprint)?;
    println!("{}", json);

    Ok(())
}

fn main() -> factorio_blueprint::Result<()> {
    match parse_arguments() {
        Some(Command::ReEncode(b)) => reencode(&b),
        Some(Command::Decode(b)) => decode(&b),
        _ => {
            eprintln!("Unknown command");
            Ok(())
        },
    }
}
