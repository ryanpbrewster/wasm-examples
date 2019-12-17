use stdweb::js;
use stdweb::web::window;
use stdweb::web::History;
use stdweb::web::Location;

pub struct UrlLocation {
    pub query_param: String,
    location: Location,
    history: History,
}

impl UrlLocation {
    pub fn new() -> UrlLocation {
        let location = window().location().expect("How is this possible?");
        let queries = location.search().unwrap();
        let query_param = queries.split("%22").nth(1).unwrap_or("1 + 1");
        let decoded = js! {
            return decodeURIComponent(@{query_param});
        }
        .into_string()
        .expect("yo man");

        UrlLocation {
            location,
            query_param: decoded,
            history: window().history(),
        }
    }

    pub fn update_route(&mut self, query: String) {
        let full_query = format!("?expr=\"{}\"", query);
        self.query_param = query.clone();
        self.history.push_state("", "", Some(&full_query));
    }
}
