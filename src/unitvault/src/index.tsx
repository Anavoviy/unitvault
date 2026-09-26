/* @refresh reload */
import "./index.css";

import { render } from "solid-js/web";
import { Router, Route } from "@solidjs/router";

import Hello from "./pages/Hello.tsx";

const wrapper = document.getElementById("app") as HTMLElement

if(!wrapper) {
    throw new Error("Wrapper div with ID 'app' not found");
}

render(
    () => (
    <Router>
        <Route path={"/"} component={Hello} />
    </Router>
    ), 
    wrapper
);
