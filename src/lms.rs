extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::str::FromStr;
use tokio_core::reactor::Core;

use hyper::{Method, Request, Uri, Client};
use hyper::header::{ContentLength, ContentType};

#[derive(Clone)]
pub struct LMS {
    base_url: Option<String>,
    player_mac: Option<String>
}

impl LMS {
  pub fn new(base_url: Option<String>, player_mac: Option<String>) -> LMS {
    LMS {
      base_url: Some(format!("http://{}/jsonrpc.js", base_url.unwrap_or("localhost:9000".to_string()))),
      player_mac: player_mac
    }
  }

  pub fn is_configured(&self) -> bool {
    if self.base_url != None {
      if self.player_mac != None {
        return true;
      }
    }

    return false;
  }

  pub fn play(&self) {
    self.request(r#"["spottyconnect","start"]"#.to_string())
  }

  pub fn stop(&self) {
    self.request(r#"["spottyconnect","stop"]"#.to_string())
  }

  pub fn change(&self) {
    self.request(r#"["spottyconnect","change"]"#.to_string())
  }

  pub fn volume(&self, volume: u16) {
    // we're not using the volume here, as LMS will read player state anyway
    self.request(format!(r#"["spottyconnect","volume",{}]"#, volume.to_string()))
  }

  pub fn request(&self, command: String) {
    debug!("Base URL to talk to LMS: {}", self.base_url.clone().unwrap());

    if let Some(ref base_url) = self.base_url {
    debug!("Player MAC address to control: {}", self.player_mac.clone().unwrap());
      if let Some(ref player_mac) = self.player_mac {
        let mut core = Core::new().unwrap();
        let client = Client::new(&core.handle());

        debug!("Command to send to player: {}", command);
        let json = format!(r#"{{"id": 1,"method":"slim.request","params":["{}",{}]}}"#, player_mac, command);
        let uri = Uri::from_str(base_url).unwrap();
        let mut req = Request::new(Method::Post, uri);

        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(json.len() as u64));
        req.set_body(json);

        let post = client.request(req);

        // ugh... just send that thing and don't care about the rest...
        match core.run(post) {
          Ok(_) => (),
          Err(_) => ()
        }
      }
    }
  }
}

/*
pub fn x() {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

info!("x");
    let json = r#"{"id": 1,"method":"slim.request","params":["62:00:e0:48:67:01",["status",0,99]]}"#;
    let uri = Uri::from_str("http://localhost:9000/jsonrpc.js").unwrap();
    let mut req = Request::new(Method::Post, uri);
    req.headers_mut().set(ContentType::json());
    req.headers_mut().set(ContentLength(json.len() as u64));
    req.set_body(json);

    let post = client.request(req);

    // ugh... just send that thing and don't care about the rest...
    match core.run(post) {
      Ok(_) => (),
      Err(_) => ()
    }
}
*/