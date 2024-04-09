use leptos::{view as html, *};
use leptos_router::*;

mod components;
mod routes;

enum Mode {
  NORMAL,
  INSERT,
}

#[component]
fn App() -> impl IntoView {
  let (mode, set_mode) = create_signal(Mode::NORMAL);

  html! {
    <Router>
      <Routes>
        <Route path="/" view=routes::home::Home />
        <Route path="/about" view=routes::about::About />
        // <Route path="/blog" view=routes::blog::BlogList />
        // <Route path="/blog/*any" view=routes::blog::BlogTemplate />
        <Route path="/*any" view=|| html! { <h1>Not Found</h1>} />
      </Routes>
    </Router>
  }
}

fn main() {
  mount_to_body(App)
}
