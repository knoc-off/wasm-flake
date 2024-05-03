use yew::prelude::*;
use web_sys::HtmlElement;
use web_sys::wasm_bindgen::JsCast;

struct Portfolio {
    name: String,
    github_url: String,
    linkedin_url: String,
}

impl Component for Portfolio {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Portfolio {
            name: "Nicholas Selby".to_string(),
            github_url: "https://github.com/knoc-off".to_string(),
            linkedin_url: "https://www.linkedin.com/in/niko-selby/".to_string(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let cv_onclick = Callback::from(|_| {
            let document = web_sys::window().unwrap().document().unwrap();
            let link = document.create_element("a").unwrap();
            let link_elem = link.dyn_into::<HtmlElement>().unwrap();
            link_elem.set_attribute("href", "static/cv.pdf").unwrap();
            link_elem.set_attribute("download", "cv.pdf").unwrap();
            link_elem.click();
        });

        html! {
            <div class="container">
                <header>
                    <h1>{ &self.name }</h1>
                    <nav>
                        <ul>
                            <li><a href={ self.github_url.clone() } target="_blank">{ "GitHub" }</a></li>
                            <li><a href={ self.linkedin_url.clone() } target="_blank">{ "LinkedIn" }</a></li>
                            <li><a href="#" onclick={ cv_onclick }>{ "CV" }</a></li>
                        </ul>
                    </nav>
                </header>
                <main>
                    <section class="about">
                        <h2>{ "About Me" }</h2>
                        <img src="static/img.png" alt={ self.name.clone() } />
                        <p>{ "Write a brief introduction about yourself here." }</p>
                    </section>
                    <section class="projects">
                        <h2>{ "Projects" }</h2>
                        <ul>
                            <li>{ "Project 1" }</li>
                            <li>{ "Project 2" }</li>
                            <li>{ "Project 3" }</li>
                        </ul>
                    </section>
                </main>
                <footer>
                    <p>{ format!("© 2024 {}. All rights reserved.", &self.name) }</p>
                </footer>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Portfolio>::new().render();
}
