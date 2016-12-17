extern crate iron;
extern crate alexa;
use iron::prelude::*;

struct RequestHandler {}
impl alexa::RequestHandler for RequestHandler {
    fn handle_request(&self, req: &alexa::Request) -> alexa::Response {
        println!("Ding");
        match req.body {
            alexa::RequestBody::IntentRequest(ref ir) => {
                println!("Got request : {}", ir.name.as_str());
                match ir.name.as_str() {
                    "DoubleNumber" => {
                        let num_o: Option<f64> = ir.slots.get("num").and_then(|n| n.parse().ok());
                        match num_o {
                            Some(num) => doubled_number_response(num),
                            None => i_dont_understand(),
                        }
                    }
                    "Hex" => hex_response(),
                    _ => i_dont_understand(),
                }
            }
            _ => i_dont_understand(),
        }
    }
}

fn hex_response<'a>() -> alexa::Response<'a> {
    alexa::Response {
        session_attributes: None,
        card: None,
        reprompt: None,
        output_speech: Some(alexa::OutputSpeech::Text(format!("Without the disguise listens \
                                                               the bush curve.")
            .into())),
        should_end_session: true,
    }
}

fn doubled_number_response<'a>(num: f64) -> alexa::Response<'a> {
    alexa::Response {
        session_attributes: None,
        card: None,
        reprompt: None,
        output_speech: Some(alexa::OutputSpeech::Text(format!("Double {} is {}", num, num * 2f64)
            .into())),
        should_end_session: true,
    }
}

fn i_dont_understand<'a>() -> alexa::Response<'a> {


    alexa::Response {
        session_attributes: None,
        card: None,
        reprompt: None,
        output_speech: Some(alexa::OutputSpeech::Text("Oh no, I don't understand what you said!"
            .into())),
        should_end_session: true,
    }
}

fn main() {
    use std::path::Path;

    let key = Path::new("localhost.key").to_path_buf();
    let cert = Path::new("localhost.crt").to_path_buf();
    let rh = RequestHandler {};
    let ih = alexa::IronHandler::new("amzn1.ask.skill.ca61e3b4-a0de-4a5d-9dc6-d6a5cd9c5eb8"
                                         .to_owned(),
                                     Box::new(rh));
    let chain = Chain::new(ih);
    Iron::new(chain).https("0.0.0.0:443", cert, key).unwrap();
}
