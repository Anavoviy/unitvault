/* @refresh reload */
import "./index.css";

import { render } from "solid-js/web";
import { Router, Route } from "@solidjs/router";

import Hello from "./pages/Hello.tsx";
import Create from "./pages/Create.tsx";
import Open from "./pages/Open.tsx";

const wrapper = document.getElementById("app") as HTMLElement

if(!wrapper) {
    throw new Error("Wrapper div with ID 'app' not found");
}

render(
    () => (
    <Router>
        <Route path={"/"} component={Hello} />
        <Route path={"/open"} component={Open} />
        <Route path={"/create"} component={Create} />
    </Router>
    ), 
    wrapper
);
