error_chain! {
    errors {
        Http {
            description("HTTP error")
        }
        Deserialize(bytes: Vec<u8>) {
            description("failed to deserialize value")
            display("could not deserialize value `{}`", String::from_utf8_lossy(&bytes))
        }
        Uri(uri: String) {
            description("invalid URI")
            display("could not parse URI: `{}`", uri)
        }
    }
}
