use std::collections::HashMap;


struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

struct BasicRouter{
    routes: HashMap<String, BoxedCallback>,
}

impl BasicRouter {
    fn new() -> BasicRouter {
        BasicRouter {
            routes: HashMap::new(),
        }
    }

    fn add_route<C>(&mut self, url: &str, callback: C) 
    where C: Fn(&Request) -> Response + 'static {
        self.routes.insert(url.to_string(), Box::new(callback))
        ;
    }

    fn handle_request(&self, req: Request) -> Response {
        match self.routes.get(&req.url) {
            Some(callback) => callback(&req),
            None => Response {
                code: 404,
                headers: HashMap::new(),
                body: Vec::new(),
            },
        }
    }
}

fn main() {
    let mut router = BasicRouter::new();
    
    router.add_route("/", |req| {
        Response {
            code: 200,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    });

    router.add_route("/gcd", |req: &Request| {
        Response {
            code: 200,
            headers: HashMap::new(),

            body: "gcd".as_bytes().to_vec(),
        }
    });

    let req = Request {
        method: "GET".to_string(),
        url: "/gcd".to_string(),
        headers: HashMap::new(),
        body: Vec::new(),
    };

    let response = router.handle_request(req);
    println!("Response code: {}", response.code);
    println!("Response body: {:?}", response.body);
}
