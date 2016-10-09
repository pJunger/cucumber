use std::io;

error_chain! { 
    types {
        Error, ErrorKind, ChainErr, Result;
    }

    foreign_links {
        io::Error, IoErrorKind;
    }
    errors {
        NotACargoProjectError {
            description("Cargo.toml not found")
            display("This is not a valid cargo project!")
        }
    }
    errors {
        ProjectAlreadyExistsError {
            description("features directory found")
            display("A Cucumber project seems to exist already!")
        }
    }
    errors {
        UserAbort {
            description("user aborted")
            display("Quitting.")
        }
    }

    errors {
        NoValidInput {
            description("invalid input")
            display("This input is not valid!")
        }
    }

    errors {
        NoValidIdentifier {
            description("invalid identifier")
            display("Please choose a valid identifier!")
        }
    }
}
